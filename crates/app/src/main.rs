use eframe::egui;
use parking_lot::Mutex;
use services::file_search::{search, FinderOptions};
use services::organizer::{apply, build_plan, ApplyReport, ProposedPlan};
use services::support::network_diagnostics;
use shared::search_types::{SearchQuery, SearchResult};
use shared::settings::AppSettings;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab { Search, Organize, Support }

struct AppState {
    settings: AppSettings,
    first_run: bool,
    // Finder UI
    query: String,
    ext_filter: String,
    results: Vec<SearchResult>,
    selected_paths: Vec<String>,
    // Organizer UI
    move_dir: String,
    rename_prefix: String,
    plan: Option<ProposedPlan>,
    apply_report: Option<ApplyReport>,
    // Support UI
    diag_summary: String,
    diag_details: Vec<String>,
    // General
    status: String,
    tab: Tab,
}

impl Default for AppState {
    fn default() -> Self {
        let (settings, first_run) = load_settings_or_default();
        Self {
            settings,
            first_run,
            query: String::new(),
            ext_filter: String::new(),
            results: vec![],
            selected_paths: vec![],
            move_dir: String::new(),
            rename_prefix: String::new(),
            plan: None,
            apply_report: None,
            diag_summary: String::new(),
            diag_details: vec![],
            status: String::new(),
            tab: Tab::Search,
        }
    }
}

impl AppState {
    fn do_search(&mut self) {
        let allowed_dirs: Vec<PathBuf> = self
            .settings
            .allowed_dirs
            .iter()
            .map(|s| PathBuf::from(s))
            .collect();
        if allowed_dirs.is_empty() {
            self.status = "Please add at least one allowed folder".into();
            return;
        }
        let exts = if self.ext_filter.trim().is_empty() {
            None
        } else {
            Some(self.ext_filter.split(',').map(|s| s.trim().to_string()).collect())
        };
        let opts = FinderOptions { allowed_dirs, max_results: self.settings.max_results };
        match search(opts, SearchQuery { text: self.query.clone(), extensions: exts }) {
            Ok(mut r) => {
                self.status = format!("{} results", r.len());
                self.results.clear();
                self.results.append(&mut r);
            }
            Err(e) => self.status = format!("Error: {}", e),
        }
    }

    fn build_plan(&mut self) {
        let paths = self.selected_paths.clone();
        self.plan = build_plan(paths, if self.move_dir.trim().is_empty() { None } else { Some(self.move_dir.clone()) }, if self.rename_prefix.trim().is_empty() { None } else { Some(self.rename_prefix.clone()) }).ok();
    }

    fn apply_plan(&mut self) {
        if let Some(plan) = self.plan.clone() {
            match apply(plan) {
                Ok(rep) => { self.apply_report = Some(rep); }
                Err(e) => { self.status = format!("Apply error: {}", e); }
            }
        }
    }
}

fn config_path() -> Option<std::path::PathBuf> {
    if let Some(proj) = directories::ProjectDirs::from("com.local", "Claudia", "ClaudiaLite") {
        let p = proj.config_dir().join("settings.json");
        let _ = fs::create_dir_all(proj.config_dir());
        Some(p)
    } else { None }
}

fn load_settings_or_default() -> (AppSettings, bool) {
    if let Some(path) = config_path() {
        if path.exists() {
            if let Ok(bytes) = fs::read(&path) {
                if let Ok(s) = serde_json::from_slice::<AppSettings>(&bytes) {
                    return (s, false);
                }
            }
        }
    }
    (AppSettings::default(), true)
}

fn save_settings(settings: &AppSettings) {
    if let Some(path) = config_path() {
        if let Ok(bytes) = serde_json::to_vec_pretty(settings) {
            let _ = fs::write(path, bytes);
        }
    }
}

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Claudia Lite",
        options,
        Box::new(|_cc| Box::new(ClaudiaApp { state: Arc::new(Mutex::new(AppState::default())) })),
    )
}

struct ClaudiaApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for ClaudiaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut s = self.state.lock();

        if s.first_run {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Initial Approval");
                ui.label("Add allowed folders (comma-separated)");
                let mut dirs = s.settings.allowed_dirs.join(", ");
                if ui.text_edit_singleline(&mut dirs).changed() {
                    s.settings.allowed_dirs = dirs
                        .split(',')
                        .map(|p| p.trim().to_string())
                        .filter(|p| !p.is_empty())
                        .collect();
                }
                ui.separator();
                ui.label("Local model (Ollama)");
                ui.text_edit_singleline(&mut s.settings.model.local_model);
                ui.checkbox(&mut s.settings.model.enable_gemini, "Enable Gemini fallback (opt-in)");
                ui.checkbox(&mut s.settings.enable_internet_research, "Enable web research (opt-in)");

                if ui.button("Approve & Save").clicked() {
                    save_settings(&s.settings);
                    s.first_run = false;
                }
            });
            return;
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("Claudia Lite");
                ui.separator();
                ui.selectable_value(&mut s.tab, Tab::Search, "Search");
                ui.selectable_value(&mut s.tab, Tab::Organize, "Organize");
                ui.selectable_value(&mut s.tab, Tab::Support, "Support");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Save Settings").clicked() { save_settings(&s.settings); }
                });
            });
        });

        egui::SidePanel::left("settings").resizable(true).show(ctx, |ui| {
            ui.heading("Settings");

            ui.label("Allowed folders (comma-separated)");
            let mut dirs = s.settings.allowed_dirs.join(", ");
            if ui.text_edit_singleline(&mut dirs).changed() {
                s.settings.allowed_dirs = dirs
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect();
            }
            ui.add(egui::Slider::new(&mut s.settings.max_results, 10..=2000).text("Max results"));

            ui.separator();
            ui.label("Local model (Ollama)");
            ui.text_edit_singleline(&mut s.settings.model.local_model);

            ui.separator();
            ui.checkbox(&mut s.settings.model.enable_gemini, "Enable Gemini fallback (opt-in)");
            ui.checkbox(&mut s.settings.enable_internet_research, "Enable web research (opt-in)");
        });

        egui::CentralPanel::default().show(ctx, |ui| match s.tab {
            Tab::Search => {
                ui.heading("Find files");
                ui.horizontal(|ui| {
                    ui.label("Query:");
                    ui.text_edit_singleline(&mut s.query);
                    if ui.button("Search").clicked() { s.do_search(); }
                });
                ui.horizontal(|ui| {
                    ui.label("Extensions (csv, e.g., pdf,md)");
                    ui.text_edit_singleline(&mut s.ext_filter);
                });

                ui.separator();
                ui.label(&s.status);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut to_add = Vec::new();
                    let mut to_remove = Vec::new();
                    
                    for r in &s.results {
                        ui.horizontal(|ui| {
                            let mut selected = s.selected_paths.iter().any(|p| p == &r.path);
                            if ui.checkbox(&mut selected, "").changed() {
                                if selected {
                                    to_add.push(r.path.clone());
                                } else {
                                    to_remove.push(r.path.clone());
                                }
                            }
                            ui.label(&r.file_name);
                            ui.small(egui::RichText::new(&r.path).monospace());
                        });
                    }
                    
                    // Apply changes after the loop
                    for path in to_add {
                        if !s.selected_paths.contains(&path) {
                            s.selected_paths.push(path);
                        }
                    }
                    for path in to_remove {
                        s.selected_paths.retain(|p| p != &path);
                    }
                });
            }
            Tab::Organize => {
                ui.heading("Organizer");
                ui.label(format!("Selected: {} files", s.selected_paths.len()));
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for p in &s.selected_paths { ui.small(egui::RichText::new(p).monospace()); }
                });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Move to dir:");
                    ui.text_edit_singleline(&mut s.move_dir);
                });
                ui.horizontal(|ui| {
                    ui.label("Rename prefix:");
                    ui.text_edit_singleline(&mut s.rename_prefix);
                });
                ui.horizontal(|ui| {
                    if ui.button("Preview Plan").clicked() { s.build_plan(); }
                    if ui.button("Apply").clicked() { s.apply_plan(); }
                });

                if let Some(plan) = &s.plan {
                    ui.separator();
                    ui.label("Plan:");
                    for a in &plan.actions {
                        match a {
                            services::organizer::OrganizeAction::Move { from, to_dir } => {
                                ui.small(format!("Move: {} -> {}/", from, to_dir));
                            }
                            services::organizer::OrganizeAction::Rename { from, to } => {
                                ui.small(format!("Rename: {} -> {}", from, to));
                            }
                        }
                    }
                }

                if let Some(rep) = &s.apply_report {
                    ui.separator();
                    ui.label(format!("Applied: {}, Skipped: {}", rep.applied, rep.skipped));
                    for e in &rep.errors {
                        ui.colored_label(egui::Color32::RED, format!("{}: {}", e.action, e.error));
                    }
                }
            }
            Tab::Support => {
                ui.heading("Support");
                if ui.button("Run Network Diagnostics").clicked() {
                    let rep = network_diagnostics().unwrap_or_else(|_| services::support::DiagnosticReport { summary: "Diagnostics error".into(), details: vec![] });
                    s.diag_summary = rep.summary;
                    s.diag_details = rep.details;
                }
                if !s.diag_summary.is_empty() { ui.label(&s.diag_summary); }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for d in &s.diag_details { ui.small(d); }
                });
            }
        });
    }
}

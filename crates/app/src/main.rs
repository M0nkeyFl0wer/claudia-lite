use eframe::egui;
use parking_lot::Mutex;
use services::file_search::{search, FinderOptions};
use shared::search_types::{SearchQuery, SearchResult};
use std::path::PathBuf;
use std::sync::Arc;

struct AppState {
    query: String,
    ext_filter: String,
    results: Vec<SearchResult>,
    status: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            query: String::new(),
            ext_filter: String::new(),
            results: vec![],
            status: "Welcome to Little Helper! Enter a search term to find files.".into(),
        }
    }
}

impl AppState {
    fn do_search(&mut self) {
        if self.query.trim().is_empty() {
            self.status = "Please enter a search term".into();
            return;
        }
        
        let exts = if self.ext_filter.trim().is_empty() {
            None
        } else {
            Some(self.ext_filter.split(',').map(|s| s.trim().to_string()).collect())
        };
        
        // Search from home directory with full access
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        let allowed_dirs = vec![PathBuf::from(home_dir)];
        let opts = FinderOptions { allowed_dirs, max_results: 500 };
        
        match search(opts, SearchQuery { text: self.query.clone(), extensions: exts }) {
            Ok(mut r) => {
                self.status = format!("Found {} files", r.len());
                self.results.clear();
                self.results.append(&mut r);
            }
            Err(e) => self.status = format!("Search error: {}", e),
        }
    }
}

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Little Helper - Tarah's File Finder",
        options,
        Box::new(|_cc| Box::new(LittleHelperApp { state: Arc::new(Mutex::new(AppState::default())) })),
    )
}

struct LittleHelperApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for LittleHelperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut s = self.state.lock();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🔍 Little Helper");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.small("Tarah's File Finder");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                ui.label("🔎 Search for files:");
                let response = ui.text_edit_singleline(&mut s.query);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    s.do_search();
                }
                if ui.button("Search").clicked() { 
                    s.do_search(); 
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("📁 File types (optional):");
                ui.text_edit_singleline(&mut s.ext_filter);
                ui.small("e.g., pdf,txt,jpg");
            });

            ui.add_space(10.0);
            ui.separator();
            
            if !s.status.is_empty() {
                ui.label(&s.status);
                ui.add_space(10.0);
            }
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                for result in &s.results {
                    ui.horizontal(|ui| {
                        if ui.button("📂").on_hover_text("Open folder").clicked() {
                            if let Some(parent) = std::path::Path::new(&result.path).parent() {
                                let _ = std::process::Command::new("xdg-open")
                                    .arg(parent)
                                    .spawn();
                            }
                        }
                        
                        ui.vertical(|ui| {
                            ui.strong(&result.file_name);
                            ui.small(egui::RichText::new(&result.path).monospace().color(egui::Color32::GRAY));
                        });
                    });
                    ui.separator();
                }
            });
        });
    }
}

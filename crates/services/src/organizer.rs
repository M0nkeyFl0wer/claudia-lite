use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum OrganizeAction {
    Rename { from: String, to: String },
    Move { from: String, to_dir: String },
}

#[derive(Debug, Clone)]
pub struct ProposedPlan {
    pub actions: Vec<OrganizeAction>,
}

#[derive(Debug, Clone)]
pub struct ApplyError {
    pub action: String,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct ApplyReport {
    pub applied: usize,
    pub skipped: usize,
    pub errors: Vec<ApplyError>,
}

pub fn build_plan(paths: Vec<String>, move_dir: Option<String>, prefix: Option<String>) -> Result<ProposedPlan> {
    let mut actions = Vec::new();
    let mv = move_dir.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let px = prefix.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

    for p in paths {
        if let Some(ref dir) = mv {
            actions.push(OrganizeAction::Move { from: p.clone(), to_dir: dir.clone() });
        }
        if let Some(ref pre) = px {
            let from = Path::new(&p);
            if let Some(name) = from.file_name().and_then(|s| s.to_str()) {
                let to_name = format!("{}{}", pre, name);
                let to = from.parent().unwrap_or_else(|| Path::new(".")).join(to_name);
                actions.push(OrganizeAction::Rename { from: p.clone(), to: to.to_string_lossy().into_owned() });
            }
        }
    }

    Ok(ProposedPlan { actions })
}

pub fn apply(plan: ProposedPlan) -> Result<ApplyReport> {
    let mut report = ApplyReport { applied: 0, skipped: 0, errors: vec![] };
    for action in plan.actions {
        match action.clone() {
            OrganizeAction::Move { from, to_dir } => {
                let src = PathBuf::from(&from);
                let dst_dir = PathBuf::from(&to_dir);
                if !dst_dir.exists() { fs::create_dir_all(&dst_dir).ok(); }
                let dst = match src.file_name() { Some(name) => dst_dir.join(name), None => { report.skipped += 1; continue; } };
                if dst.exists() {
                    report.skipped += 1;
                    continue;
                }
                if let Err(e) = fs::rename(&src, &dst) {
                    report.errors.push(ApplyError { action: format!("Move {} -> {}", from, dst.display()), error: e.to_string() });
                } else {
                    report.applied += 1;
                }
            }
            OrganizeAction::Rename { from, to } => {
                let src = PathBuf::from(&from);
                let dst = PathBuf::from(&to);
                if dst.exists() {
                    report.skipped += 1;
                    continue;
                }
                if let Err(e) = fs::rename(&src, &dst) {
                    report.errors.push(ApplyError { action: format!("Rename {} -> {}", from, to), error: e.to_string() });
                } else {
                    report.applied += 1;
                }
            }
        }
    }
    Ok(report)
}

//! The response fired when an unauthorized USB change is detected.
//!
//! Runs any configured pre-commands, then performs the action. In dry-run mode
//! nothing is executed — every step is logged with a `[dry-run]` prefix — so the
//! guard can be tested safely without powering off the machine.

use std::process::Command;

use crate::config::{Action, Config};

pub fn trigger(cfg: &Config, dry_run: bool) {
    for cmd in &cfg.pre_commands {
        run(cmd, dry_run);
    }

    match cfg.action {
        Action::Shutdown => {
            if dry_run {
                log::warn!("[dry-run] would power off now");
            } else {
                #[cfg(target_os = "linux")]
                run("shutdown -h now", false);
                #[cfg(target_os = "macos")]
                run("shutdown -h now", false);
                #[cfg(target_os = "windows")]
                run("shutdown /s /t 0", false);
            }
        }
        Action::Lock => {
            log::warn!("locking session{}", if dry_run { " [dry-run]" } else { "" });
            if !dry_run {
                #[cfg(target_os = "linux")]
                run("loginctl lock-session", false);
                #[cfg(target_os = "macos")]
                run("pmset displaysleepnow", false);
                #[cfg(target_os = "windows")]
                run("rundll32.exe user32.dll,LockWorkStation", false);
            }
        }
        Action::LogOnly => {
            log::warn!("action=log — event recorded, no shutdown");
        }
    }
}

fn run(cmd: &str, dry_run: bool) {
    if dry_run {
        log::warn!("[dry-run] would run: {cmd}");
        return;
    }
    log::info!("running: {cmd}");
    let mut parts = cmd.split_whitespace();
    if let Some(program) = parts.next() {
        let status = Command::new(program).args(parts).status();
        if let Err(e) = status {
            log::error!("command failed ({cmd}): {e}");
        }
    }
}

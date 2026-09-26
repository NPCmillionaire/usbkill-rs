//! usbkill — anti-forensic kill-switch.
//!
//! Polls the set of connected USB devices. If that set changes (a device is
//! inserted or removed) while the guard is armed, it triggers the configured
//! response — by default, an immediate shutdown — to protect an encrypted,
//! unattended machine from physical tampering or seizure.
//!
//! Rust rewrite of hephaest0s/usbkill (GPLv3). Only acts on the machine it runs
//! on, under the operator's own configuration.

use std::collections::BTreeSet;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;

mod config;
mod response;
mod usb;

use config::Config;

#[derive(Parser, Debug)]
#[command(name = "usbkill", version, about = "Anti-forensic USB kill-switch")]
struct Cli {
    /// Path to the INI config file.
    #[arg(short, long, default_value = "/etc/usbkill.ini")]
    config: String,

    /// Print detected USB devices and exit (does not arm the guard).
    #[arg(long)]
    list: bool,

    /// Arm but never fire — log what *would* happen. For safe testing.
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    let cfg = Config::load(&cli.config).unwrap_or_else(|e| {
        log::warn!("could not load {}: {e}; using defaults", cli.config);
        Config::default()
    });

    if cli.list {
        for d in usb::current_devices() {
            println!("{d}");
        }
        return;
    }

    let dry_run = cli.dry_run || cfg.dry_run;
    log::info!(
        "usbkill armed (poll {}ms, dry_run={})",
        cfg.poll_interval_ms,
        dry_run
    );

    let baseline: BTreeSet<String> = usb::current_devices().into_iter().collect();
    log::info!("baseline: {} device(s)", baseline.len());

    loop {
        let now: BTreeSet<String> = usb::current_devices().into_iter().collect();
        if now != baseline {
            let added: Vec<_> = now.difference(&baseline).collect();
            let removed: Vec<_> = baseline.difference(&now).collect();
            log::warn!("USB change detected — added: {added:?}, removed: {removed:?}");
            response::trigger(&cfg, dry_run);
            if dry_run {
                sleep(Duration::from_millis(cfg.poll_interval_ms));
                continue;
            }
            break;
        }
        sleep(Duration::from_millis(cfg.poll_interval_ms));
    }
}

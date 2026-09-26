//! Configuration, loaded from an INI file compatible in spirit with the
//! original `usbkill.ini`.

use ini::Ini;

#[derive(Debug, Clone)]
pub struct Config {
    /// How often to poll the USB device set, in milliseconds.
    pub poll_interval_ms: u64,
    /// Response when a change is detected.
    pub action: Action,
    /// Commands run *before* the action fires (e.g. wipe RAM, sync). Optional.
    pub pre_commands: Vec<String>,
    /// If true, never actually fire — just log. Overridable by --dry-run.
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Power off immediately (default, matches original usbkill).
    Shutdown,
    /// Lock the session instead of powering off.
    Lock,
    /// Only log the event — useful for tuning before arming for real.
    LogOnly,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            poll_interval_ms: 250,
            action: Action::Shutdown,
            pre_commands: Vec::new(),
            dry_run: false,
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, String> {
        let ini = Ini::load_from_file(path).map_err(|e| e.to_string())?;
        let mut cfg = Config::default();
        if let Some(s) = ini.section(Some("config")) {
            if let Some(v) = s.get("poll_interval_ms").and_then(|x| x.parse().ok()) {
                cfg.poll_interval_ms = v;
            }
            cfg.action = match s.get("action").map(str::trim) {
                Some("lock") => Action::Lock,
                Some("log") | Some("logonly") => Action::LogOnly,
                _ => Action::Shutdown,
            };
            cfg.dry_run = matches!(s.get("dry_run"), Some("true") | Some("1"));
            if let Some(cmds) = s.get("pre_commands") {
                cfg.pre_commands = cmds
                    .split(';')
                    .map(|c| c.trim().to_string())
                    .filter(|c| !c.is_empty())
                    .collect();
            }
        }
        Ok(cfg)
    }
}

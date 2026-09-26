//! USB device enumeration.
//!
//! Uses `nusb` for a pure-Rust, cross-platform device list. Each device is
//! reduced to a stable identity string (bus/address + vid:pid + serial when
//! available) so insert/remove is detected even for identical device models.

/// Return the current set of connected USB device identity strings.
pub fn current_devices() -> Vec<String> {
    match nusb::list_devices() {
        Ok(devs) => devs
            .map(|d| {
                let serial = d.serial_number().unwrap_or("-");
                format!(
                    "{:03}:{:03} {:04x}:{:04x} {}",
                    d.bus_number(),
                    d.device_address(),
                    d.vendor_id(),
                    d.product_id(),
                    serial
                )
            })
            .collect(),
        Err(e) => {
            log::error!("USB enumeration failed: {e}");
            Vec::new()
        }
    }
}

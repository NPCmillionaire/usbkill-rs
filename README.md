# usbkill-rs

An anti-forensic **kill-switch**. It watches your machine's USB ports and, the
instant a device is plugged in or removed while armed, runs a configured
response — by default an immediate shutdown — so an encrypted, unattended
laptop protects itself against physical tampering or seizure.

A modern Rust rewrite of [hephaest0s/usbkill](https://github.com/hephaest0s/usbkill)
(Python, GPLv3). Same idea, rebuilt as a single static binary: no interpreter,
no dependency install on the target, cross-platform USB enumeration, and a
`--dry-run` mode so you can arm it safely before trusting it.

> **Scope & ethics.** usbkill only acts on the machine it runs on, under your
> own configuration. It is a data-protection tool for hardware you own or are
> authorized to protect. It is **not** designed to affect any other system.

## Status

Early scaffold — device polling, config, and response logic are stubbed and
compile-ready. Not yet battle-tested. Use `dry_run = true` until you've verified
behavior on your hardware.

## How it works

1. On start, usbkill records the set of currently connected USB devices as the
   trusted **baseline**.
2. It polls that set every `poll_interval_ms`.
3. If the set ever differs from the baseline, it fires the configured `action`.

## Build

```sh
cargo build --release
# binary at target/release/usbkill
```

## Usage

```sh
usbkill --list                     # show detected USB devices, then exit
usbkill --dry-run                  # arm, but only log what it would do
sudo usbkill -c /etc/usbkill.ini   # arm for real (needs privilege to shut down)
```

## Configuration

See [`usbkill.ini`](./usbkill.ini). Key options: `poll_interval_ms`, `action`
(`shutdown` | `lock` | `log`), optional `pre_commands`, and the `dry_run` safety
switch (defaults to `true` in the sample config).

## License

GPL-3.0-or-later, matching the original usbkill. See [`LICENSE`](./LICENSE).

## Roadmap

- [ ] Verify `nusb` enumeration across Linux / macOS / Windows
- [ ] `whitelist` of allowed device IDs (mirror original's `--whitelist`)
- [ ] Optional secure RAM/swap wipe as a built-in pre-command
- [ ] systemd unit + launchd plist for install
- [ ] Integration tests with a virtual USB device

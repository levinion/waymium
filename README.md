# waymium

A lightweight, Vimium-inspired window switcher for Wayland, built with Rust and GTK4.

![showcase](/assets/show.webp)

## Features
* **Hint-based Switching**: Quickly jump to windows using generated key sequences (e.g., `AS`, `JK`).
* **Coordinate Overlay**: Accurately places hint labels over window positions.
* **Low Latency**: Built with Rust for immediate responsiveness and minimal resource usage.

## Support & Roadmap
**waymium** aims to support all major wlroots-based compositors compositors.

- **Compositor Support (PRs Welcome!):**

    - [x] [Ura](https://github.com/levinion/ura) (My compositor!)   
    - [ ] Hyprland
    - [ ] Sway
    - [ ] River
    - [ ] Niri

## Prerequisites
* **gtk4**
* **gtk4-layer-shell**

## Usage
1. Launch `waymium`.
2. Type the displayed hint to focus a window.
3. Press `Esc` to cancel and exit.

## Build
```bash
cargo build --release

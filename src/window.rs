use std::rc::Rc;

use anyhow::Result;
use gtk4::{Align, ApplicationWindow, EventControllerKey, gdk::Key, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::{hint, label::Label, state::State};

pub struct Window {
    win: Rc<ApplicationWindow>,
    state: Rc<State>,
}

impl Window {
    pub fn new(app: &gtk4::Application, state: Rc<State>) -> Self {
        let win = Rc::new(gtk4::ApplicationWindow::new(app));
        Self { win, state }
    }

    pub fn init(&self) {
        self.win.init_layer_shell();
        self.win.set_layer(Layer::Overlay);
        self.win.set_exclusive_zone(-1);
        self.win
            .set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
        let anchors = [
            (Edge::Left, true),
            (Edge::Right, true),
            (Edge::Top, true),
            (Edge::Bottom, true),
        ];
        for (anchor, state) in anchors {
            self.win.set_anchor(anchor, state);
        }
    }

    pub fn load_css(&self) {
        let provider = gtk4::CssProvider::new();
        let default_style = include_str!("./style.css");
        provider.connect_parsing_error(|provider, section, error| {
            eprintln!(
                "Error while parsing css: {:?}:{:?} {:?}",
                section.file().and_then(|f| f.path()),
                section.start_location(),
                error
            );
            eprintln!("Fallback to default css style...");
            provider.load_from_data(default_style);
        });
        match &self.state.style {
            Some(style) => {
                provider.load_from_path(style);
            }
            None => provider.load_from_data(default_style),
        }
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn load_keyboard_controller(&mut self) {
        let controller = EventControllerKey::new();
        let state = self.state.clone();
        let win = self.win.clone();
        controller.connect_key_pressed(move |_, key, _keycode, _state| match key {
            Key::Escape => {
                win.close();
                gtk4::glib::Propagation::Stop
            }
            _ => {
                let key = key.to_unicode();
                match key {
                    Some(key) => {
                        state
                            .buffer
                            .borrow_mut()
                            .push_str(&key.to_lowercase().to_string());

                        let mut matched = false;

                        for (label, i) in state.labels.borrow().iter() {
                            let hint = label.text();
                            if hint == *state.buffer.borrow() {
                                state.compositor.borrow().activate(*i).unwrap();
                                win.close();
                            } else if hint.starts_with(state.buffer.borrow().as_str()) {
                                matched = true;

                                // update label
                                label.matched.set_text(state.buffer.borrow().as_str());
                                let unmatched =
                                    hint.strip_prefix(state.buffer.borrow().as_str()).unwrap();
                                label.unmatched.set_text(unmatched);
                            }
                        }

                        // if not prefix matched, then quit
                        if !matched {
                            win.close();
                        }

                        gtk4::glib::Propagation::Stop
                    }
                    None => gtk4::glib::Propagation::Stop,
                }
            }
        });
        self.win.add_controller(controller);
    }

    pub fn present(&self) {
        self.win.present();
    }

    pub fn update(&mut self) -> Result<()> {
        self.state.compositor.borrow_mut().update()?;

        let overlay = gtk4::Fixed::new();
        overlay.add_css_class("overlay");

        let windows = self.state.compositor.borrow().windows()?;
        if windows.is_empty() {
            self.win.close();
        }
        for (i, win) in windows.iter().enumerate() {
            let hint_text = hint::get_hint(i, windows.len(), &self.state.config.charset)?;
            let label = Label::new(&hint_text);
            self.state.labels.borrow_mut().push((label.clone(), i));

            let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            container.append(&label.matched);
            container.append(&label.unmatched);
            container.add_css_class("hint-label");

            let anchor = match self.state.config.hint_anchor.as_str() {
                "top-left" => (Align::Start, Align::Start),
                "top-right" => (Align::End, Align::Start),
                "bottom-left" => (Align::Start, Align::End),
                "bottom-right" => (Align::End, Align::End),
                "center" => (Align::Center, Align::Center),
                "left" => (Align::Start, Align::Center),
                "top" => (Align::Center, Align::Start),
                "right" => (Align::End, Align::Center),
                "bottom" => (Align::Center, Align::End),
                _ => (Align::Start, Align::Start),
            };

            container.set_halign(anchor.0);
            container.set_valign(anchor.1);

            let window_container = gtk4::Overlay::new();
            container.add_css_class("window-container");
            window_container.set_size_request(win.width as i32, win.height as i32);
            window_container.add_overlay(&container);

            overlay.put(&window_container, win.x as f64, win.y as f64);
        }
        self.win.set_child(Some(&overlay));
        Ok(())
    }
}

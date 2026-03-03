use std::rc::Rc;

use anyhow::Result;
use gtk4::{ApplicationWindow, EventControllerKey, gdk::Key, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::{hint, state::State};

pub struct Window {
    win: ApplicationWindow,
    state: Rc<State>,
}

impl Window {
    pub fn new(app: &gtk4::Application, state: Rc<State>) -> Self {
        let win = gtk4::ApplicationWindow::new(app);
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
        provider.load_from_data(
            "
            window {
                background-color: transparent;
            }
            .hint-label {
                font-size: 24px;
                font-weight: bold;
            }
        ",
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn load_keyboard_controller(&mut self) {
        let controller = EventControllerKey::new();
        let state = self.state.clone();
        controller.connect_key_pressed(move |ctrl, key, _keycode, _state| match key {
            Key::Escape => {
                if let Some(widget) = ctrl.widget() {
                    widget
                        .activate_action("window.close", None)
                        .unwrap_or_default();
                }
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

                        for (hint, i) in state.charset.borrow().iter() {
                            if *hint == *state.buffer.borrow() {
                                state.compositor.activate(*i).unwrap();
                                if let Some(widget) = ctrl.widget() {
                                    widget
                                        .activate_action("window.close", None)
                                        .unwrap_or_default();
                                }
                            }
                        }

                        gtk4::glib::Propagation::Stop
                    }
                    None => gtk4::glib::Propagation::Stop,
                }
            }
        });
        self.win.add_controller(controller);
    }

    pub fn show(&self) {
        self.win.show();
    }

    pub fn setup_components(&mut self) -> Result<()> {
        let background = gtk4::Fixed::new();
        let windows = self.state.compositor.windows()?;
        for (i, win) in windows.iter().enumerate() {
            let hint_text = hint::get_hint(i);
            self.state.charset.borrow_mut().push((hint_text.clone(), i));
            let label = gtk4::Label::builder().label(&hint_text).build();
            label.add_css_class("hint-label");
            background.put(&label, win.x as f64, win.y as f64);
        }
        self.win.set_child(Some(&background));
        Ok(())
    }
}

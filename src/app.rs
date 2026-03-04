use crate::state::State;
use crate::window::Window;
use anyhow::Result;
use gtk4::gio::ApplicationFlags;
use gtk4::prelude::*;
use std::rc::Rc;

pub struct Application {
    app: gtk4::Application,
    state: Rc<State>,
}

impl Application {
    pub fn new(state: State) -> Self {
        let app = gtk4::Application::new(Some("de.levinion.waymium"), ApplicationFlags::default());
        Self {
            app,
            state: Rc::new(state),
        }
    }

    pub fn run(&self) {
        let state = self.state.clone();
        self.app.connect_activate(move |app| {
            let f = || -> Result<()> {
                let mut win = Window::new(app, state.clone());
                win.init();
                win.load_css();
                win.load_keyboard_controller();
                win.update()?;
                win.present();
                Ok(())
            };
            if let Err(err) = f() {
                eprintln!("{}", err);
                app.quit();
            }
        });
        self.app.run();
    }
}

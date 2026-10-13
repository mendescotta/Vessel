mod atomic_write;
mod backend;
mod cli;
mod generate;
mod output;
mod profile;
mod snapshot;
mod ui;

use adw::prelude::*;

const APP_ID: &str = "org.voidlinux.vessel";

fn load_app_icon() {
    if let Some(display) = gtk::gdk::Display::default() {
        if let Some(exe_dir) = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
        {
            let source_icons = exe_dir.join("../../resources/images");
            if source_icons.join(format!("{APP_ID}.svg")).is_file() {
                gtk::IconTheme::for_display(&display).add_search_path(source_icons);
            }
        }
    }
    gtk::Window::set_default_icon_name(APP_ID);
}

fn main() -> glib::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(code) = cli::run(&args) {
        return glib::ExitCode::from(code);
    }
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        load_app_icon();
        ui::window::build(app).present();
    });
    app.run()
}

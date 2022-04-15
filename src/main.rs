mod app;

fn main() {
    let options = eframe::NativeOptions { 
        resizable: true,
        initial_window_size: Some(egui::Vec2::new(1200.0, 700.0)),
        decorated: true,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(Box::new(app::controller::Application::default()), options);
}

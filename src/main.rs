//sandbox
mod app;
mod enums;
mod file;
mod finder;

use futures::executor; // 0.3.1

fn main() {
    let sd = "/Users/matthew/tmp/file_types/audio/Short.mp3";
    let filter = [true, true, true, true, true];
    let single_file = return_dfer2(sd, filter);
    println!("single_file => {:#?}", single_file);

    let sd = "/Users/matthew/tmp/file_types/audio";
    let filter = [true, true, true, true, true];
    let multi_batch = return_dfer2(sd, filter);
    //println!("x => {:#?}", multi_batch);

    let single = single_file.data_set;
    let multi = multi_batch.data_set;

    let mut matched = vec![];
    // Loop through files to find
    for collection in single {
        let (key, _value) = collection;

        // Loop through multi
        for item in multi.iter() {
            let (key2, _value2) = item;

            if &key == key2 { 
                matched.push(item);
            }
        }
    }

    dbg!("{:#?}", matched);

    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::Vec2::new(400.0, 280.0)),
        decorated: true,
        transparent: true,
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(Box::new(app::controller::Application::default()), options);
}

fn return_dfer2(path: &str, filters: [bool; 5]) -> finder::finder::Finder {
    let mut ff = finder::finder::Finder::new();
    //Block to connect to async values
    executor::block_on(ff.rayon_walk_dir(path, filters));

    ff.adjust_file_order();

    ff
}

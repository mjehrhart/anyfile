use byte_unit::Byte;
use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, Sense, TextStyle};
use egui_extras::RetainedImage;
//use rayon::vec;

use crate::{enums::enums::FileType, file::meta::Meta, return_dfer2};
use std::path::PathBuf;

pub struct Application<'a> {
    pub shrink: bool,
    pub test_size2: f32,
    pub test_offset: egui::Vec2,
    pub test_size: egui::Vec2,
    pub stage: Vec<Meta>,
    pub staging: Vec<(String, Vec<Meta>)>,
    pub staging2: Vec<(&'a String, &'a Vec<Meta>)>,
    pub directory: String,
    pub image_directory: RetainedImage,
    pub image_exit: RetainedImage,
    pub image_settings: RetainedImage,
}
//&home::home_dir().unwrap()
impl Application<'_> {
    pub fn default() -> Self {
        Self {
            //
            stage: vec![],
            staging: vec![],
            staging2: vec![],
            directory: String::new(),
            image_directory: RetainedImage::from_image_bytes(
                "Directory",
                include_bytes!("../../resources/folder.png"),
            )
            .unwrap(),
            image_exit: RetainedImage::from_svg_bytes(
                "Exit",
                include_bytes!("../../resources/close.svg"),
            )
            .unwrap(),
            image_settings: RetainedImage::from_svg_bytes(
                "Settingd",
                include_bytes!("../../resources/settings.svg"),
            )
            .unwrap(),
            //
            shrink: false,
            test_size2: 0.,
            test_offset: egui::Vec2::new(0., 0.),
            test_size: egui::Vec2::new(35., 35.),
            // sub: vec![],
        }
    }

    pub fn add_label(&mut self, ui: &mut egui::Ui, text: String) {
        ui.add(egui::Label::new(
            egui::RichText::new(text).color(egui::Color32::from_rgb(255, 255, 255)),
        ));
    }
}

impl epi::App for Application<'_> {
    fn name(&self) -> &str {
        "AnyFile 0.0.1"
    }

    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
        self.directory = home::home_dir().unwrap().as_path().display().to_string();

        // Testing
        self.directory = "/Users/matthew/tmp/".to_string();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        //
        //********************************************************************************************* */
        // File Drops
        if !ctx.input().raw.dropped_files.is_empty() {
            //println!("Files dropped");

            let dropped_files = ctx
                .input()
                .raw
                .dropped_files
                .clone()
                .iter()
                .map(|file| file.path.as_ref().unwrap().clone())
                .collect::<Vec<PathBuf>>();

            //println!("{:?}", dropped_files);

            //****************************************************** */
            let mut needles = vec![];
            for dropped_file in dropped_files {
                //
                //let sd = "/Users/matthew/tmp/file_types/audio/Short.mp3";
                let sd = &dropped_file.as_os_str().to_string_lossy();
                let filter = [true, true, true, true, true];
                let single_file = return_dfer2(&sd, filter);
                needles.push(single_file.data_set);
            }

            // Clear self.staging
            self.stage.clear();

            //println!("needles => {:#?}", needles);

            // Search through Haystack
            // let sd = "/Users/matthew/tmp/file_types/audio";
            let filter = [true, true, true, true, true];
            let multi_batch = return_dfer2(&self.directory, filter);
            //println!("x => {:#?}", multi_batch);
            let multi = multi_batch.data_set;

            // Empty vec to store matches
            //let mut matched = vec![];
            // Loop through files to find matches
            // for i in 0..needles.len() {
            //     for (key, value) in &needles[i] {
            //         // Loop through multi
            //         for item in multi {
            //             //println!("{:#?}", &item.1.l);
            //             if (*key == item.0)
            //             //&& (value[0].path != item.1[0].path)
            //             {
            //                 //matched.push(item);
            //                 self.staging.push(item.clone());
            //             }
            //         }
            //     }
            // }

            // Matching records here
            for item in multi {
                let (_, list) = item;
                for row in list {
                    for i in 0..needles.len() {
                        for (key, value) in needles[i].clone() {
                            if key == row.checksum && value[0].path != row.path {
                                self.stage.push(row.clone());
                            }
                        }
                    }
                }
            }

            println!("self.stage => {:#?}", &self.stage);

            //self.staging = matched.clone();

            if self.stage.len() == 0 {
                let tmp = egui::Vec2::new(260.0, 260.0);
                _frame.set_window_size(tmp)
            } else {
                let tmp = egui::Vec2::new(860., 260.);
                _frame.set_window_size(tmp);
            }
            //dbg!("{:#?}", &self.staging);
        }

        //********************************************************************************************* */
        // UI
        //********************************************************************************************* */
        let frame_style_1 = egui::containers::Frame {
            margin: egui::style::Margin {
                left: 10.,
                right: 2.,
                top: 5.,
                bottom: 2.,
            },
            rounding: egui::Rounding {
                nw: 1.0,
                ne: 1.0,
                sw: 1.0,
                se: 1.0,
            },
            shadow: eframe::epaint::Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            //fill: Color32::from_rgb(49, 90, 125),
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(255, 255, 255)),
        };

        self::Application::add_top_bar(self, ctx, _frame);

        egui::CentralPanel::default()
            .frame(frame_style_1)
            .show(ctx, |ui| {
                // Add bullseye
                ui.add(toggle2(&mut true));
            });

        //ui.add_sized(ui.available_size(), Image::new(…));

        // Right PANEL
        //fn get_table_fields(dt: &Meta) -> (String, String, String) {
        fn get_table_fields(dt: &Meta) -> String {
            let byte = Byte::from_bytes(dt.file_size.try_into().unwrap());
            let adjusted_byte = byte.get_appropriate_unit(false);

            let mut title: String = String::from("");
            let mut icon: String = String::from("");

            match dt.file_type {
                FileType::Image => {
                    title = format!("{}", dt.path);
                    icon = "🖼".to_string();
                }
                FileType::Audio => {
                    title = format!("{}", dt.path);
                    icon = "🎵".to_string();
                }
                FileType::Video => {
                    title = format!("{}", dt.path);
                    icon = "🎞".to_string();
                }
                FileType::Document => {
                    title = format!("{}", dt.path);
                    icon = "📎".to_string();
                }
                FileType::Other => {
                    title = format!("{}", dt.path);
                    icon = "📝".to_string();
                }
                FileType::None => {}
                FileType::All => {}
            }

            // title
            title = title.to_lowercase();
            if title.len() < 60 {
                title = title[0..title.len()].to_string();
            } else {
                title = title[0..60].to_string();
            }
            let diff = 60 - title.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            title = [title.to_string(), space].join("");

            // adjusted_byte
            let diff = 10 - adjusted_byte.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            let adjusted_byte = [space, adjusted_byte.to_string()].join("");

            let full_title = [icon, title, adjusted_byte].join("  ");
            //(title, adjusted_byte, icon)
            full_title
        }

        egui::SidePanel::right("right_panel")
            .frame(frame_style_1)
            //.min_width(self.test_size2)
            .min_width(630.)
            .resizable(true)
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .id_source("bottom_scroll2")
                    .auto_shrink([false, false])
                    .max_height(220.)
                    .min_scrolled_height(220.)
                    .stick_to_right()
                    .show(ui, |ui| {
                        //
                        ui.vertical(|ui| {
                            for row in &self.stage {
                                egui::Grid::new("grid_main_labels")
                                    .striped(true)
                                    .num_columns(2)
                                    .striped(true)
                                    .spacing(egui::Vec2::new(10.0, 0.0))
                                    .show(ui, |ui| {
                                        let full_title = get_table_fields(row);

                                        ui.add_sized(
                                            [600., 10.0],
                                            egui::Button::new(
                                                egui::RichText::new(full_title)
                                                    .color(egui::Color32::from_rgb(255, 255, 255))
                                                    .monospace(),
                                            )
                                            //.fill(egui::Color32::from_rgb(49, 90, 125)),
                                            .fill(egui::Color32::TRANSPARENT),
                                        );
                                        ui.add_sized(
                                            [20.0, 10.0],
                                            egui::Hyperlink::from_label_and_url("VIEW", &row.path),
                                        );
                                    });
                            }
                        });
                    });

                // let widget_rect =
                //     egui::Rect::from_min_size(ui.min_rect().min + self.test_offset, self.test_size);
                // ui.put(widget_rect, toggle2(&mut true));
            });

        // let tmp = egui::Vec2::new(400., 100.);
        // //_frame.set_window_size(ctx.used_size());
        // _frame.set_window_size(tmp);
    }
}

pub fn toggle_ui2(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(20.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.43 * rect.height();

        let x_center = 120.;
        let y_center = 150.;
        let center = egui::pos2(x_center, y_center);
        let radius_center = 5.32;

        // outer rings
        ui.painter().circle(
            center,
            radius_center * 17.,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(14.0, Color32::from_rgb(123, 167, 204)),
        );

        // middle rings
        ui.painter().circle(
            center,
            radius_center * 11.,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(20.0, Color32::from_rgb(123, 167, 204)),
        );
        // ui.painter().circle(
        //     center,
        //     radius_center * 10.,
        //     egui::Color32::TRANSPARENT,
        //     egui::Stroke::new(2.0, Color32::from_rgb(170, 70, 0)),
        // );
        // ui.painter().circle(
        //     center,
        //     radius_center * 9.,
        //     egui::Color32::TRANSPARENT,
        //     egui::Stroke::new(2.0, Color32::from_rgb(170, 70, 0)),
        // );

        // inner rings
        ui.painter().circle(
            center,
            radius_center * 5.,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(10.0, Color32::from_rgb(123, 167, 204)),
        );
    }

    response
}
pub fn toggle2(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui2(ui, on)
}
pub fn test() {
    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::Pos2::new(0.0, 00.0)),
        resizable: true,
        //initial_window_size: Some(egui::Vec2::new(880.0, 280.0)),
        initial_window_size: Some(egui::Vec2::new(260.0, 260.0)),
        decorated: false,
        transparent: true,
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(Box::new(super::controller::Application::default()), options);
}

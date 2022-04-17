use byte_unit::Byte;
use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, Sense};
use egui_extras::RetainedImage;

use crate::{
    enums::enums::{self, FileAction},
    file::{self, meta::Meta},
    finder::finder,
};

#[derive(Clone, Debug)]
pub struct DupeTable {
    pub name: String,
    pub count: i32,
    pub checksum: String,
    pub list: Vec<Meta>,
    pub file_type: enums::FileType,
    pub visible: bool,
}

pub struct Application {
    pub staging_checksum: String,
    pub staging: Vec<Vec<DupeTable>>,
    pub sub_staging: Vec<file::meta::Meta>,
    pub dupe_table: Vec<DupeTable>,
    pub finder: finder::Finder,
    pub directory: String,
    //
    pub time_elapsed: std::time::Duration,
    pub show_filter_bar: bool,
    //
    pub filter_search_filetype: [bool; 5],
    pub filters_filetype_counters: [i32; 6],
    //
    pub filter_all: bool,
    pub filter_audios: bool,
    pub filter_documents: bool,
    pub filter_images: bool,
    pub filter_videos: bool,
    pub filter_others: bool,
    //
    pub flag_checkbox_audios: bool,
    pub flag_checkbox_documents: bool,
    pub flag_checkbox_images: bool,
    pub flag_checkbox_others: bool,
    pub flag_checkbox_videos: bool,
    //
    pub image_checkbox_audios: RetainedImage,
    pub image_checkbox_documents: RetainedImage,
    pub image_checkbox_others: RetainedImage,
    pub image_checkbox_images: RetainedImage,
    pub image_checkbox_videos: RetainedImage,
    //
    pub image_filter: RetainedImage,
    pub image_directory: RetainedImage,
    pub image_run: RetainedImage,
    pub image_timer: RetainedImage,
    pub image: RetainedImage,
    pub image2: RetainedImage,
    //
    pub sort_by: [String; 3],
    pub sort_by_index: usize,
    pub pager_size: Vec<usize>,
    pub pager_size_index: usize,
    pub selected_staging_index: usize,
    pub fuzzy_search: String,
}

// from_svg_bytes
// from_image_bytes

impl Application {
    pub fn default() -> Self {
        Self {
            staging_checksum: String::from(""),
            sub_staging: vec![file::meta::Meta::new()],
            staging: vec![],               //used in paging
            dupe_table: vec![],            //ui uses this for show and tell
            finder: finder::Finder::new(), //runs the search
            directory: String::from("/Users/matthew/zz/file_types"),
            //
            time_elapsed: std::time::Duration::new(0, 0),
            show_filter_bar: true,
            //
            filter_search_filetype: [true, true, false, false, false],
            filters_filetype_counters: [0; 6],
            //
            filter_all: true,
            filter_audios: true,
            filter_documents: true,
            filter_images: true,
            filter_videos: true,
            filter_others: true,
            //
            flag_checkbox_audios: false,
            flag_checkbox_documents: true,
            flag_checkbox_images: false,
            flag_checkbox_others: false,
            flag_checkbox_videos: false,
            //
            sort_by_index: 0,
            sort_by: [
                "Duplicates".to_string(),
                "Name".to_string(),
                "Size".to_string(),
            ],
            pager_size: [100, 1_000, 10_000, 25_000, 35_000, 50_000, 100_000].to_vec(),
            pager_size_index: 4,
            selected_staging_index: 0,
            fuzzy_search: String::from(""),
            //
            image_checkbox_audios: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/checked.png"),
            )
            .unwrap(),
            image_checkbox_documents: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/checked.png"),
            )
            .unwrap(),
            image_checkbox_images: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            image_checkbox_others: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            image_checkbox_videos: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            //
            image_filter: RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/filter1.png"),
            )
            .unwrap(),
            image_run: RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/play.png"),
            )
            .unwrap(),
            image_directory: RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/folder.png"),
            )
            .unwrap(),
            image_timer: RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/chronometer.png"),
            )
            .unwrap(),
            image: RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/checked.png"),
            )
            .unwrap(),
            image2: RetainedImage::from_svg_bytes(
                "Settings",
                include_bytes!("../../resources/settings.svg"),
            )
            .unwrap(),
        }
    }
}

impl epi::App for Application {
    fn name(&self) -> &str {
        "Sandbox"
        //crate::defines::APP_NAME
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {}

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        //
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
            fill: Color32::from_rgb(49, 90, 125),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(244, 244, 244)),
        };

        // LEFT PANEL
        egui::SidePanel::left("left_panel")
            .frame(frame_style_1)
            .min_width(113.)
            .show(ctx, |ui| {
                ui.add_space(70.);
                self::Application::left_menu(self, ui, ctx);
            });

        // TOP PANEL
        self::Application::top_layout(self, ctx);

        // MAIN
        self::Application::main_layout(self, ctx);

        // DELETION/SUB PANEL
        egui::TopBottomPanel::bottom("bottom_sub_panel")
            .frame(frame_style_1)
            .show(ctx, |ui| {
                // BUTTON DELETE ROW
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Delete Below").color(egui::Color32::LIGHT_RED), //.monospace(),
                    ))
                    .clicked()
                {
                    // EVENT DELETE ITEMS
                    let collection = self
                        .finder
                        .data_set
                        .get_mut(&self.staging_checksum)
                        .unwrap();

                    for mut collection in self.staging[self.selected_staging_index].iter_mut() {
                        if collection.checksum == self.staging_checksum {
                            //println!("..collection => {:#?}", collection);
                            collection.list.retain(|x| {
                                (x.status == FileAction::None)
                                    || (x.status == FileAction::Read)
                                    || (x.status == FileAction::Save)
                            });

                            println!("...collection => {:#?}", collection);
                        }

                    }
 
                    //Remove file from os first
                    // let mut deleted_count = 0;
                    // for row in &self.sub_staging {
                    //     if row.status == FileAction::Delete {
                    //         std::fs::remove_file(&row.path).ok();
                    //         deleted_count += 1;
                    //     }
                    // }

                    //Remove file element from hashmap for gui first
                    self.sub_staging.retain(|x| {
                        (x.status == FileAction::None)
                            || (x.status == FileAction::Read)
                            || (x.status == FileAction::Save)
                    });
 
                    //Remove file element from hashmap for gui second
                    let collection = self
                        .finder
                        .data_set
                        .get_mut(&self.staging_checksum)
                        .unwrap();

                    collection.retain(|x| {
                        (x.status == FileAction::None)
                            || (x.status == FileAction::Read)
                            || (x.status == FileAction::Save)
                    });
                }

                // DELETION/SUB TABLE
                ui.add_space(5.0);
                ScrollArea::vertical()
                    .id_source("bottom_scroll2")
                    .auto_shrink([false, false])
                    .max_height(130.)
                    .min_scrolled_height(130.)
                    .stick_to_right()
                    .show(ui, |ui| {
                        //
                        ui.vertical(|ui| {
                            for row in self.sub_staging[..].iter_mut() {
                                //********************************************************//

                                //Formatting text for gui
                                let date = get_created_date(&row.path);
                                match &date {
                                    Ok(_) => {}
                                    Err(_) => {
                                        break;
                                    }
                                }

                                let byte = Byte::from_bytes(row.file_size.try_into().unwrap());
                                let adjusted_byte = byte.get_appropriate_unit(false);

                                let mut title: String;
                                title = format!("▶ {} ", row.path); //▶
                                title = truncate(&title, 94).to_string();
                                //'attemp to subtract with overflow'
                                let diff = 95 - title.chars().count();
                                let mut space = " ".to_string();
                                for _ in 0..diff {
                                    space.push(' ');
                                }
                                title = [title.to_string(), space].join(" ");

                                ///////////////////////////////////////////////////////////////

                                egui::Grid::new("deletion_grid")
                                    .striped(true)
                                    .num_columns(5)
                                    .min_row_height(20.0)
                                    .spacing(egui::Vec2::new(0.0, 0.0))
                                    .show(ui, |ui| {
                                        if ui.checkbox(&mut row.ui_event_status, " ").clicked() {
                                            //
                                            if row.ui_event_status {
                                                row.status = FileAction::Delete;
                                            } else {
                                                row.status = FileAction::Read;
                                            }

                                            //println!("collection => {:#?}", collection);
                                            /* let collection = self
                                                .finder
                                                .data_set
                                                .get_mut(&self.staging_checksum)
                                                .unwrap();

                                            for mut row2 in collection {
                                                if row2.path == row.path {
                                                    //println!("row2 => {:#?}", row2);
                                                    if row.ui_event_status {
                                                        row2.status = FileAction::Delete;
                                                        row2.ui_event_status = true;
                                                    } else {
                                                        row2.status = FileAction::Read;
                                                        row2.ui_event_status = false;
                                                    }
                                                }
                                            } */
                                        };
                                        ui.add_sized(
                                            [400.0, 15.0],
                                            egui::Label::new(
                                                egui::RichText::new(title)
                                                    .color(egui::Color32::from_rgb(200, 200, 200))
                                                    .monospace(),
                                            ),
                                        );
                                        ui.add_sized(
                                            [100.0, 15.0],
                                            egui::Label::new(
                                                egui::RichText::new(date.unwrap())
                                                    .color(egui::Color32::from_rgb(200, 200, 200))
                                                    .monospace(),
                                            ),
                                        );
                                        ui.add_sized(
                                            [100.0, 15.0],
                                            egui::Label::new(
                                                egui::RichText::new(adjusted_byte.to_string())
                                                    .color(egui::Color32::from_rgb(200, 200, 200))
                                                    .monospace(),
                                            ),
                                        );
                                        ui.add_sized(
                                            [100.0, 15.0],
                                            egui::Hyperlink::from_label_and_url("VIEW", &row.path),
                                        );
                                        ui.end_row();
                                    });
                            }
                        });
                    }); //end of scroll
            });
    }

    fn on_exit(&mut self) {
        println!("exiting...")
    }
}

pub fn get_created_date(path: &str) -> std::io::Result<String> {
    let _metadata = match std::fs::metadata(path) {
        Ok(f) => {
            if let Ok(time) = f.created() {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                let t: String = datetime.format("%m/%d/%Y").to_string();
                return Ok(t);
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "foo"));
            }
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File not Found: 100221",
            ))
        }
    };
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

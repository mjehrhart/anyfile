use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, Sense};
use egui_extras::RetainedImage;

use crate::{finder::finder, enums::enums, file::meta::Meta};

#[derive(Clone, Debug )]
pub struct DupeTable {
    pub name: String,
    pub count: i32,
    pub checksum: String,
    pub list: Vec<Meta>, 
    pub file_type: enums::FileType,
    pub visible: bool,
}


pub struct Application {
    pub dupe_table: Vec<DupeTable>,
    // pub staging: Vec<Vec<DupeTable>>, 
    // pub a: finder::Finder,
    pub finder: finder::Finder,
    // pub c: Vec<file::meta::Meta>,
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
    pub fuzzy_search: String,
}

// from_svg_bytes [true, true, false, false, true]
// from_image_bytes
// hovered
// on_hover_ui
impl Application {
    pub fn default() -> Self {
        Self {
            dupe_table: vec![],             //ui uses this for show and tell
            finder:finder::Finder::new(),   //runs the search
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
            pager_size: [3, 5, 10, 1_000, 10_000, 25_000, 35_000, 50_000, 100_000].to_vec(),
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

        egui::SidePanel::left("left_panel")
            .frame(frame_style_1)
            .show(ctx, |ui| {
                ui.add_space(70.);
                self::Application::left_menu(self, ui, ctx);
            });

        self::Application::top_layout(self, ctx);

        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(frame_style_1)
            .show(ctx, |ui| {
                ui.checkbox(&mut self.filter_audios, "Show subsection");
                ui.add_visible_ui(self.filter_audios, |ui| {
                    ui.label("Maybe you see this, maybe you don't!");

                    ScrollArea::vertical()
                        .id_source("bottom_scroll")
                        .auto_shrink([false, false])
                        .max_height(400.)
                        .stick_to_right()
                        .show(ui, |ui| {
                            self::Application::my_label(self, ui, "bottom".to_string());
                        }); //end of scroll
                });
            });

        self::Application::main_layout(self, ctx);
    }

    fn on_exit(&mut self) {
        println!("exiting...")
    }
}

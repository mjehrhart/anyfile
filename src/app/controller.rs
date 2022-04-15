use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, Sense};
use egui_extras::RetainedImage;

pub struct Application {
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
    pub sort_left_panel: [String; 3],
    pub pager_size: Vec<usize>,
    pub fuzzy_search: String,
}

// from_svg_bytes
// from_image_bytes
// hovered
// on_hover_ui
impl Application {
    pub fn default() -> Self {
        Self {
            filter_all: true,
            filter_audios: true,
            filter_documents: true,
            filter_images: true,
            filter_videos: true,
            filter_others: true,
            //
            flag_checkbox_audios: true,
            flag_checkbox_documents: true,
            flag_checkbox_images: true,
            flag_checkbox_others: true,
            flag_checkbox_videos: true,
            //
            sort_left_panel: [
                "Duplicates".to_string(),
                "Name".to_string(),
                "Size".to_string(),
            ],
            pager_size: [3, 5, 10, 1_000, 10_000, 25_000, 35_000, 50_000, 100_000].to_vec(),
            fuzzy_search: String::from(""),
            //
            image_checkbox_audios: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            image_checkbox_documents: RetainedImage::from_image_bytes(
                "Checkbox1",
                include_bytes!("../../resources/unchecked.png"),
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
        let my_frame0 = egui::containers::Frame {
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
            fill: Color32::from_rgb(244, 244, 244),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(244, 244, 244)),
        };

        let my_frame1 = egui::containers::Frame {
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
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(244, 244, 244)),
        };

        let my_frame2 = egui::containers::Frame {
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
            .frame(my_frame2)
            .show(ctx, |ui| {
                ui.add_space(70.);
                self::Application::left_menu(self, ui, ctx);
            });

        egui::TopBottomPanel::top("top_panel")
            .frame(my_frame2)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(), |ui| {
                        //DIRECTORY IMAGE
                        let image_size = self.image_filter.size_vec2();
                        let image_button = egui::ImageButton::new(
                            self.image_directory.texture_id(ctx),
                            [image_size.x / 2., image_size.y / 2.],
                        )
                        .frame(false);

                        if ui.add(image_button).clicked() {}
                        self::Application::my_label(
                            self,
                            ui,
                            "/Users/matthew/documents".to_string(),
                        );

                        //FILTER IMAGE, RUN IMAGE, TIMER IMAGE
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                            //FILTER IMAGE
                            let image_size = self.image_filter.size_vec2();
                            let image_button = egui::ImageButton::new(
                                self.image_filter.texture_id(ctx),
                                [image_size.x / 2., image_size.y / 2.],
                            )
                            .frame(false);

                            ui.add_space(10.);
                            if ui.add(image_button).clicked() {}

                            //RUN IMAGE
                            let image_size = self.image_filter.size_vec2();
                            let image_button = egui::ImageButton::new(
                                self.image_run.texture_id(ctx),
                                [image_size.x / 2., image_size.y / 2.],
                            )
                            .frame(false);

                            if ui.add(image_button).clicked() {}

                            //Timer IMAGE
                            let image_size = self.image_filter.size_vec2();
                            let image_button = egui::ImageButton::new(
                                self.image_timer.texture_id(ctx),
                                [image_size.x / 2., image_size.y / 2.],
                            )
                            .frame(false);

                            if ui.add(image_button).clicked() {}
                            self::Application::my_label(self, ui, ".0023424s".to_string());
                        });
                    });
                });
            });

        egui::TopBottomPanel::top("top_panel2")
            .frame(my_frame2)
            .show(ctx, |ui| {
                //ui.group(|ui| {
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    self::Application::checkbox_audio(self, ui, ctx);
                    self::Application::checkbox_documents(self, ui, ctx);
                    self::Application::checkbox_images(self, ui, ctx);
                    self::Application::checkbox_others(self, ui, ctx);
                    self::Application::checkbox_videos(self, ui, ctx);

                    // DROPDOWN SORT BY
                    if egui::ComboBox::new("dropdown_sort_by", "")
                        .width(100.0)
                        .show_index(ui, &mut 0, self.sort_left_panel.len(), |i| {
                            self.sort_left_panel[i].to_owned()
                        })
                        .clicked()
                    {};

                    // NUMBER OF ROWS
                    if egui::ComboBox::new("number_of_rows", "")
                        .width(100.0)
                        .show_index(ui, &mut 5, self.pager_size.len(), |i| {
                            self.pager_size[i].to_owned().to_string()
                        })
                        .clicked()
                    {};

                    // FZZY FILTER
                    self::Application::my_label(self, ui, "Filter".to_string());
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fuzzy_search)
                            .code_editor()
                            .desired_width(300.),
                    );
                });
                //});
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(my_frame2)
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .id_source("bottom_scroll")
                    .auto_shrink([false, false])
                    .max_height(400.) 
                    .stick_to_right()
                    .show(ui, |ui| {
                        self::Application::my_label(self, ui, "bottom".to_string());
                    }); //end of scroll
            });

        egui::CentralPanel::default()
            .frame(my_frame2)
            .show(ctx, |ui| {
                //ScrollArea aTable ScrollAreaOutput<()>
                let row_height = 35.0;
                ScrollArea::vertical()
                    .id_source("main_scroll")
                    .auto_shrink([false, false])
                    //.max_height(500.)
                    .stick_to_right()
                    .show_rows(ui, row_height, 100, |ui, row_range| {
                        for row in row_range {
                            // let (title, adjusted_byte, file_count) =
                            //     get_table_fields(vec_table[row].clone());

                            egui::Grid::new("grid_main_labels")
                                .striped(true)
                                .num_columns(3)
                                .spacing(egui::Vec2::new(0.0, 0.0))
                                .show(ui, |ui| {
                                    if ui
                                        .add_sized(
                                            [970.0, 35.0],
                                            egui::Button::new(
                                                egui::RichText::new("filename")
                                                    .color(egui::Color32::from_rgb(255, 255, 255)),
                                            )
                                            .fill(egui::Color32::from_rgb(49, 90, 125)),
                                        )
                                        .clicked()
                                    {}
                                    ui.add_sized(
                                        [100.0, 35.0],
                                        egui::Button::new(
                                            egui::RichText::new("file_count")
                                                .color(egui::Color32::from_rgb(45, 51, 59)),
                                        )
                                        .fill(egui::Color32::from_rgb(49, 90, 125)),
                                    );
                                    ui.add_sized(
                                        [100.0, 35.0],
                                        egui::Button::new(
                                            egui::RichText::new("asjusted_bytes")
                                                .color(egui::Color32::from_rgb(45, 51, 59)),
                                        )
                                        .fill(egui::Color32::from_rgb(49, 90, 125)),
                                    );
                                    ui.end_row();
                                });
                        }
                    }); //end of scroll
            });
    }

    fn on_exit(&mut self) {
        println!("exiting...")
    }
}

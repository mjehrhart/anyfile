use egui::Color32;
use egui_extras::RetainedImage;

use super::controller::Application;

impl Application {
    pub fn checkbox_audio(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //
        let image_size = self.image.size_vec2();
        let image_button = egui::ImageButton::new(
            self.image_checkbox_audios.texture_id(ctx),
            [image_size.x / 16., image_size.y / 16.],
        )
        .frame(false);

        self::Application::my_label(self, ui, "Audio".to_string());

        // .on_hover_ui(|ui| {
        //                 ui.label("Extensions:: | doc | docx | txt | xls | pdf | ppt | vcs | zip |");
        //             })
        
        if ui.add(image_button).clicked() {
            //
            self.flag_checkbox_audios = !self.flag_checkbox_audios;
            self.filter_search_filetype[0] = !self.flag_checkbox_audios;
            //
            if self.flag_checkbox_audios {
                self.image_checkbox_audios = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/unchecked.png"),
                )
                .unwrap();
            } else {
                self.image_checkbox_audios = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/checked.png"),
                )
                .unwrap();
            }
        }
    }

    pub fn checkbox_documents(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //
        let image_size = self.image.size_vec2();
        let image_button = egui::ImageButton::new(
            self.image_checkbox_documents.texture_id(ctx),
            [image_size.x / 16., image_size.y / 16.],
        )
        .frame(false);

        self::Application::my_label(self, ui, "Documemts".to_string());

        if ui.add(image_button).clicked() {
            //
            self.flag_checkbox_documents = !self.flag_checkbox_documents;
            self.filter_search_filetype[1] = !self.flag_checkbox_documents;
            //
            if self.flag_checkbox_documents {
                self.image_checkbox_documents = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/unchecked.png"),
                )
                .unwrap();
            } else {
                self.image_checkbox_documents = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/checked.png"),
                )
                .unwrap();
            }
        }
    }

    pub fn checkbox_images(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //
        let image_size = self.image.size_vec2();
        //
        let image_button = egui::ImageButton::new(
            self.image_checkbox_images.texture_id(ctx),
            [image_size.x / 16., image_size.y / 16.],
        )
        .frame(false);

        self::Application::my_label(self, ui, "Images".to_string());

        if ui.add(image_button).clicked() {
            //
            self.flag_checkbox_images = !self.flag_checkbox_images;
            self.filter_search_filetype[2] = !self.flag_checkbox_images;
            //
            if self.flag_checkbox_images {
                self.image_checkbox_images = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/unchecked.png"),
                )
                .unwrap();
            } else {
                self.image_checkbox_images = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/checked.png"),
                )
                .unwrap();
            }
        }
    }

    pub fn checkbox_others(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //
        let image_size = self.image.size_vec2();
        //
        let image_button = egui::ImageButton::new(
            self.image_checkbox_others.texture_id(ctx),
            [image_size.x / 16., image_size.y / 16.],
        )
        .frame(false);

        self::Application::my_label(self, ui, "Others".to_string());

        if ui.add(image_button).clicked() {
            //
            self.flag_checkbox_others = !self.flag_checkbox_others;
            self.filter_search_filetype[3] = !self.flag_checkbox_others;
            //
            if self.flag_checkbox_others {
                self.image_checkbox_others = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/unchecked.png"),
                )
                .unwrap();
            } else {
                self.image_checkbox_others = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/checked.png"),
                )
                .unwrap();
            }
        }
    }

    pub fn checkbox_videos(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //
        let image_size = self.image.size_vec2();
        //
        let image_button = egui::ImageButton::new(
            self.image_checkbox_videos.texture_id(ctx),
            [image_size.x / 16., image_size.y / 16.],
        )
        .frame(false);

        self::Application::my_label(self, ui, "Videos".to_string());

        if ui.add(image_button).clicked() {
            //
            self.flag_checkbox_videos = !self.flag_checkbox_videos;
            self.filter_search_filetype[4] = !self.flag_checkbox_videos;
            //
            if self.flag_checkbox_videos {
                self.image_checkbox_videos = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/unchecked.png"),
                )
                .unwrap();
            } else {
                self.image_checkbox_videos = RetainedImage::from_image_bytes(
                    "Checkbox2",
                    include_bytes!("../../resources/checked.png"),
                )
                .unwrap();
            }
        }
    }

}

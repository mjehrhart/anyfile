use egui::Color32;
use egui_extras::RetainedImage;

use super::controller::Application;

impl Application {
    pub fn my_label(&mut self, ui: &mut egui::Ui, text: String) {
        ui.add(egui::Label::new(
            egui::RichText::new(text).color(egui::Color32::from_rgb(244, 244, 244)),
        ));
    }
    pub fn left_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let color32_blue = Color32::from_rgb(123, 167, 204);
        let color32_blue_2 = Color32::from_rgb(70, 130, 180);
        let color32_purple = Color32::from_rgb(180, 70, 75);
        let color32_orange = Color32::from_rgb(180, 120, 70);
        let color32_green = Color32::from_rgb(70, 180, 120);
        let color32_green_2 = Color32::from_rgb(180, 175, 70);

        ui.add_space(15.0);
        if ui
            .add(self::Application::toggle(
                &mut self.filter_all,
                color32_blue,
                "All".to_string(),
            ))
            .clicked()
        {
            if self.filter_all {
                self.filter_audios = true;
                self.filter_documents = true;
                self.filter_images = true;
                self.filter_videos = true;
                self.filter_others = true;
            } else {
                self.filter_audios = false;
                self.filter_documents = false;
                self.filter_images = false;
                self.filter_videos = false;
                self.filter_others = false;
            }

            self.image = RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap();
        };

        ui.add_space(10.0);
        if ui
            .add(self::Application::toggle(
                &mut self.filter_audios,
                color32_purple,
                "🎵 Audio".to_string(),
            ))
            .clicked()
        {
            self.image = RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/checked.png"),
            )
            .unwrap();
        }

        ui.add_space(10.0);
        ui.add(self::Application::toggle(
            &mut self.filter_documents,
            color32_orange,
            "📎 Documents".to_string(),
        ));
        ui.add_space(10.0);
        ui.add(self::Application::toggle(
            &mut self.filter_images,
            color32_green,
            "🖼 Images".to_string(),
        ));
        ui.add_space(10.0);
        ui.add(self::Application::toggle(
            &mut self.filter_others,
            color32_blue_2,
            "📝 Other".to_string(),
        ));
        ui.add_space(10.0);
        ui.add(self::Application::toggle(
            &mut self.filter_videos,
            color32_green_2,
            "🎞 Video".to_string(),
        ));
        ui.add_space(30.0); 
    }

    pub fn toggle_ui(
        ui: &mut egui::Ui,
        on: &mut bool,
        color32: Color32,
        text: String,
    ) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(6.0, 0.6);
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

            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::from_white_alpha(100));
                ui.label(text);
            });

            ui.painter().rect(rect, radius, color32, visuals.bg_stroke);

            // Paint the circle, animating it from left to right with `how_on`:
            let circle_x = egui::lerp(
                (rect.left() + radius + 40.0)..=(rect.right() - radius - 20.0),
                how_on,
            );
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter().circle(
                center,
                1.95 * radius,
                color32,
                egui::Stroke::new(0.0, color32),
            );
        }

        response
    }
    pub fn toggle(on: &mut bool, color32: Color32, text: String) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| self::Application::toggle_ui(ui, on, color32, text)
    }

    /*  pub fn toggle_ui2(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
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

            let center = egui::pos2(75., 75.);
            ui.painter().circle(
                center,
                1.95 * 20.,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(2.0, Color32::from_rgb(123, 167, 204)),
            );

            let center = egui::pos2(75., 75.);
            ui.painter().circle(
                center,
                1.95 * 17.,
                egui::Color32::from_rgb(123, 167, 204),
                egui::Stroke::new(2.0, Color32::from_rgb(123, 167, 204)),
            );

            // ui.add_sized(
            //     [100.0, 35.0],
            //     egui::Button::new(egui::RichText::new("5").color(egui::Color32::from_rgb(45, 51, 59)))
            //         .fill(egui::Color32::from_rgb(228, 244, 252)),
            // );
        }

        response
    }
    pub fn toggle2(on: &mut bool) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| toggle_ui2(ui, on)
    } */
}

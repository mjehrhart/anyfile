use egui::Color32;

use crate::requests;

use super::controller::Application;

impl Application {
    pub fn add_top_bar(&mut self, ctx: &egui::Context, frame: &eframe::epi::Frame) {
        //
        let frame_style_2 = egui::containers::Frame {
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
            //fill: Color32::from_rgb(197, 216, 232),
            //fill: Color32::from_rgb(49, 90, 125),
            fill: Color32::from_rgb(70, 130, 180),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(255, 255, 255)),
        };

        egui::TopBottomPanel::top("top_panel")
            .frame(frame_style_2)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    // ************************************************************************************************ //
                    // Combobox Repo List
                    if egui::ComboBox::new("dropdown_sort_by", "")
                        .width(100.0)
                        .show_index(ui, &mut self.repo_list_index, self.repo_list.len(), |i| {
                            self.repo_list[i].to_owned()
                        })
                        .changed()
                    {
                        // Load branches for repo
                        self::Application::get_branches_by_repo(self);

                        // Load staging
                        let results = requests::get_repo_file_list::get_repo_file_list(
                            &self.config,
                            self.repo_list[self.repo_list_index].to_string(),
                            self.branch_list[self.branch_list_index].to_string(),
                        );

                        self.staging.clear();

                        for content in results {
                            for item in content.tree {
                                self.staging.push(item);
                            }
                        }
                    };

                    // ************************************************************************************************ //
                    // Combobox Repo List
                    if egui::ComboBox::new("dropdown_branch_list", "")
                        .width(100.0)
                        .show_index(
                            ui,
                            &mut self.branch_list_index,
                            self.branch_list.len(),
                            |i| self.branch_list[i].to_owned(),
                        )
                        .changed()
                    {     
                    };

                    //
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.add_space(5.);
                        // ************************************************************************************************ //
                        // Exit IMAGE
                        let image_size = self.image_exit.size_vec2();
                        let image_exit = egui::ImageButton::new(
                            self.image_exit.texture_id(ctx),
                            [image_size.x / 2., image_size.y / 2.],
                        )
                        .frame(true);

                        if ui.add(image_exit).clicked() {
                            frame.quit();
                        }

                        // ************************************************************************************************ //
                        //
                        // Reload files
                        let image_size = self.image_reload.size_vec2();
                        let image_reload = egui::ImageButton::new(
                            self.image_reload.texture_id(ctx),
                            [image_size.x / 42., image_size.y / 42.],
                        )
                        .frame(true);

                        if ui.add(image_reload).clicked() {
                            // Load staging
                            let results = requests::get_repo_file_list::get_repo_file_list(
                                &self.config,
                                self.repo_list[self.repo_list_index].to_string(),
                                self.branch_list[self.branch_list_index].to_string(),
                            );

                            self.staging.clear();

                            for content in results {
                                for item in content.tree {
                                    self.staging.push(item);
                                }
                            }
                        }

                        // ************************************************************************************************ //
                    });

                    ui.add_space(5.0);
                }); 
            });
    }
}

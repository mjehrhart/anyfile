use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, Sense, TextStyle};
use egui_extras::RetainedImage;

use std::path::PathBuf;

pub struct Application {}

impl Application {
    pub fn default() -> Self {
        Self {}
    }
}
 
impl epi::App for Application {
    fn name(&self) -> &str {
        "Sandbox 0.0.1"
    }

    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        //
        //********************************************************************************************* */
        // if !ctx.input().raw.hovered_files.is_empty() {
        //     //
        //     let mut text = "Dropping files:\n".to_string();
        //     //
        //     for file in &ctx.input().raw.hovered_files {
        //         //
        //         if let Some(path) = &file.path {
        //             text += &format!("\n{}", path.display());
        //         } else if !file.mime.is_empty() {
        //             text += &format!("\n{}", file.mime);
        //         } else {
        //         }
        //     }
        // }

        if !ctx.input().raw.dropped_files.is_empty() {
            println!("Files dropped");

            let dropped_files = ctx
                .input()
                .raw
                .dropped_files
                .clone()
                .iter()
                .map(|file| file.path.as_ref().unwrap().clone())
                .collect::<Vec<PathBuf>>();

            //dbg!(&dropped_files);
            println!("{:?}", dropped_files);
        }

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
            fill: Color32::from_rgb(49, 90, 125),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(255, 255, 255)),
        };

        egui::CentralPanel::default()
            .frame(frame_style_1)
            .show(ctx, |ui| {
                ui.label("Drop Target");

                ui.add( toggle2(&mut true));
                
            });
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

         
        let x_center = 200.;
        let y_center = 140.;
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
            egui::Stroke::new(20.0, Color32::from_rgb(170, 70, 0)),
        );
        ui.painter().circle(
            center,
            radius_center * 10.,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, Color32::from_rgb(170, 70, 0)),
        );
        ui.painter().circle(
            center,
            radius_center * 9.,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, Color32::from_rgb(170, 70, 0)),
        );

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

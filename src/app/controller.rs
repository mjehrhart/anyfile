 use crate::{
    config::Config,
    requests::{
        self,
        put_file_in_repo::{self, },
    },
};
use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, };
use egui_extras::RetainedImage;
use std::{path::{PathBuf, Path}};
use std::{
    fs::File,
    io::{BufReader, Read},
};
extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

//
pub struct Application {
    pub config: Config, 
    pub pre_staging: Vec<String>, 
    pub staging: Vec<requests::get_repo_file_list::RepoList>, 
    pub branch_list: Vec<String>,
    pub branch_list_index: usize,
    pub repo_list: Vec<String>,
    pub repo_list_index: usize, 
    pub image_exit: RetainedImage,
    pub image_reload: RetainedImage, 
}

#[allow(dead_code)]
impl Application {
    //
    pub fn default(config: Config) -> Self {
        Self { 
            branch_list: vec![],
            branch_list_index: 0,        
            config,
            repo_list: vec![],
            repo_list_index: 0,         //Testing
            pre_staging: vec![],  
            staging: vec![],
            image_exit: RetainedImage::from_svg_bytes(
                "Exit",
                include_bytes!("../../resources/close.svg"),
            )
            .unwrap(), 
            image_reload: RetainedImage::from_image_bytes(  "Settingd",
                include_bytes!("../../resources/refresh.png"),
            )
            .unwrap(),   
        }
    }
    
    // REQUESTS
    // HTTP::Get - Get list of possible repos to use
    pub fn get_list_of_repos_by_user(&mut self) {
        //
        // Load combobox with repo names
        let repo_list = requests::get_repos::get_repo_list_by_user(&self.config);
 
        // Load reult into self.repo_list
        for item in repo_list.unwrap() {
            self.repo_list.push(item.name);
        } 
 
    }
 
    // HTTP::Get - Get list of possible repos to use
    pub fn get_branches_by_repo(&mut self) {
        //
        // Load combobox with repo names
        let branches = requests::get_repo_branches::get_repo_branches(&self.config, self.repo_list[self.repo_list_index].to_string());
 
        // Load result into self.branch_list 
        self.branch_list_index = 0;
        self.branch_list.clear();
        for item in branches.unwrap() {
            self.branch_list.push(item.name);
        } 
 
    }

    // HTTP::Get - Get list of files from repo
    pub fn get_list_of_files_by_repo(&mut self){
        
        let results = requests::get_repo_file_list::get_repo_file_list(
            &self.config,
            self.repo_list[self.repo_list_index].to_string(),
            self.branch_list[self.branch_list_index].to_string(),
        );
        
        // Load staging2
        self.staging.clear();
        for content in results {
            for item in content.tree {
                self.staging.push(item);
            }
        }
    }

    // HTTP::Delete - Remove file from github 
    pub fn delete_file_from_cloud(&mut self, row: requests::get_repo_file_list::RepoList, index: usize){
        
        let x = requests::delete_file_in_repo::delete_file_in_repo(
            self.config.clone(),
            self.repo_list[self.repo_list_index].to_string(),
            row.sha.to_string(),
            row.path,
        ); 

        if let Ok((status_code,_response)) = x {
             if status_code == 200 {
                 // Remove element from ui 
                 self.staging.remove(index);
             }
         } 
    }
 
    // HTTP::Put - Add file to cloud storage GitHub
    pub fn add_file_to_cloud(&mut self, base64: Result<String, Box<dyn std::error::Error>>, file_name: PathBuf) {
        
        let x = requests::put_file_in_repo::put_file_in_repo(
            self.config.clone(),
            self.repo_list[self.repo_list_index].to_string(),
            base64.unwrap(),
            &file_name,
            file_name.extension(),
        );
 
        if let Ok(item) = x {  
            let put_file_in_repo::Content { content: response } = item;
            { 
                self.pre_staging.push(response.path);
            }
        }
    }

    // UI
    pub fn add_label(&mut self, ui: &mut egui::Ui, text: String) {
        ui.add(egui::Label::new(
            egui::RichText::new(text).color(egui::Color32::from_rgb(255, 255, 255)),
        ));
    }

    //
    pub fn add_file_drag_drop_support(&mut self, ctx: &egui::Context){
        //
        // File Drops
        if !ctx.input().raw.dropped_files.is_empty() {
            let dropped_files = ctx
                .input()
                .raw
                .dropped_files
                .clone()
                .iter()
                .map(|file| file.path.as_ref().unwrap().clone())
                .collect::<Vec<PathBuf>>();

            //****************************************************** */
        
            // Collect dropped_files
            for dropped_file in dropped_files {
                //
                // Convert file to base64
                let file_base64 = self::Application::read_file(&dropped_file);

                self::Application::add_file_to_cloud(self, file_base64, dropped_file);
            }  

            // Reload other screen
            self::Application::get_list_of_files_by_repo(self); 
        }
    }
    
    // Main View with two different tables
    pub fn show_main_table(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {

        // Set Background for different tables 
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
            fill: Color32::from_rgb(70, 130, 180),
            //fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(255, 255, 255)),
        };
 
        
        egui::CentralPanel::default()
            .frame(frame_style_2) 
            .show(ctx, |ui| {
                //
                ui.add_space(15.0);
                ScrollArea::vertical()
                    .id_source("main_scroll_area")
                    .auto_shrink([false, false]) 
                    .stick_to_right()
                    .show(ui, |ui| { 
                ui.vertical(|ui| {
                    //
                    let mut i = 0; 
 
                    // Table for repo list of files 
                    for row in self.staging.to_owned(){ 
                    //for row in self.staging2.iter_mut().cloned(){ 

                        let fill_color:egui::Color32  = if self.pre_staging.contains(&row.path) {
                            egui::Color32::from_rgb(49, 90, 125)
                        } else {
                            egui::Color32::from_rgb(123,167,204)
                        };
                            
                        if row.mode == "040000"{ 
                            //do nothing
                        }
                        else {
                            egui::Grid::new("grid_repo_file_list")
                            .striped(true)  
                            .show(ui, |ui| {
                                //
                                // ************************************************************************************************ //
                                if ui
                                .add_sized( 
                                    [500.0, 20.0],
                                    egui::Button::new(
                                        egui::RichText::new(&row.path)
                                            .color(egui::Color32::from_rgb(255, 255, 255))
                                            .monospace(),
                                    )
                                    .fill(fill_color),
                                ).clicked(){
                                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

                                    let url = [
                                        "https://raw.githubusercontent.com/mjehrhart/".to_string(),
                                        self.repo_list[self.repo_list_index].to_string(),
                                        "/main/".to_string(),
                                        row.path.to_string()
                                    ].join("");

                                    // Send http_img to computer Clipboard
                                    let http_img = format!(
                                            "<img width=\"100%\" alt=\"{name}\" src=\"{download_url}\">",
                                            name = row.path,
                                            download_url = url
                                        );

                                    ctx.set_contents(http_img).unwrap();
                                }

                                // ************************************************************************************************ //
                                // REMOVE
                                if ui
                                    .add_sized( 
                                        [35.0, 20.0],
                                        egui::Button::new(
                                            egui::RichText::new("Remove")
                                                .color(egui::Color32::from_rgb(255, 255, 255))
                                                .monospace(),
                                        )
                                        .fill(fill_color),
                                    )
                                    .clicked()
                                {  
                                    // Remove file from cloud and ui
                                    self::Application::delete_file_from_cloud(self, row.clone(), i);
                                }

                                // ************************************************************************************************ //
                                // VIEW 
                                let go_url = [
                                        "https://raw.githubusercontent.com/mjehrhart/".to_string(),
                                        self.repo_list[self.repo_list_index].to_string(),
                                        "/main/".to_string(),
                                        row.path.to_string()
                                    ].join("");

                                ui.add_sized(
                                    [35.0, 20.0],
                                    egui::Hyperlink::from_label_and_url("View".to_string(), &go_url),
                                ); 
                            
                            });  
                                
                        }

                        i += 1;
                    }
                  
                }); 
                 });
        });
    }

    // HELPERS
    // Reads a file and converts to base64
    pub fn read_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer)?;

        let base64_image = base64::encode(&buffer);

        Ok(base64_image)
    }
 
}

impl epi::App for Application {
    fn name(&self) -> &str {
        "AnyFile 1.0.2"
    }

    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
 
        if !self.config.auth_token.is_empty() && !self.config.username.is_empty()  {
            // Load repos
            self::Application::get_list_of_repos_by_user(self);

            // Load branches for repo
            self::Application::get_branches_by_repo(self);

            // Load repo files
            self::Application::get_list_of_files_by_repo(self); 
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) { 
        
        if !self.config.auth_token.is_empty() && !self.config.username.is_empty()  {

            self::Application::add_file_drag_drop_support(self, ctx);
            
            self::Application::add_top_bar(self, ctx, _frame);
    
            self::Application::show_main_table(self, ctx, _frame);

        }  else {

            let home_dir = home::home_dir().unwrap().as_path().display().to_string();
            let path_joined = [home_dir, ".anyfile/config.json".to_string()].join("/");
            let text = ["Please fillout ".to_string(), path_joined.to_string(), " to connect to GitHub.".to_string()].join("");
 
             egui::CentralPanel::default() 
            .show(ctx, |ui| {
                //
                ui.add_space(15.0);

                egui::Grid::new("grid_main_labels")
                .striped(true)
                .num_columns(1)
                .striped(true)
                .spacing(egui::Vec2::new(10.0, 0.0))
                .show(ui, |ui| {
                    ui.add_sized(
                        [550., 35.0],
                        egui::Label::new(
                            egui::RichText::new(text)
                                .color(egui::Color32::from_rgb(10, 10, 10))
                                .monospace(),
                        ) 
                    ); 
                }); 

            });
        }
    }
} 
 
pub fn run(config: crate::config::Config) {
    //
    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::Pos2::new(0.0, 00.0)),
        resizable: true,
        initial_window_size: Some(egui::Vec2::new(625.0, 260.0)),
        decorated: true,
        transparent: true,
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        Box::new(super::controller::Application::default(config)),
        options,
    );
}

 

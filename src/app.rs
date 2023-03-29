use crate::fileio::rdt::RDTHeader;
use crate::fileio::rdt_scd::init_opcode_documentation;
use crate::fileio::rdt_scd::parse_rdt_scd_stream;
use crate::fileio::utils::read_file;
use std::collections::HashMap;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    opcode_documentation: HashMap<String, String>,
    picked_path: String,
    code_string: Vec<String>,
    code_string_init: Vec<String>,
    code_string_main: Vec<String>,
    raw_code: String,
    raw_code_init: String,
    raw_code_main: String,
    button_code_init_enabled: bool,
    button_code_main_enabled: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            opcode_documentation: init_opcode_documentation(),
            picked_path: "".to_owned(),
            code_string: Vec::new(),
            code_string_init: Vec::new(),
            code_string_main: Vec::new(),
            raw_code: "".to_owned(),
            raw_code_init: "".to_owned(),
            raw_code_main: "".to_owned(),
            button_code_init_enabled: true,
            button_code_main_enabled: true,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            opcode_documentation: _,
            picked_path: _,
            code_string: _,
            code_string_init: _,
            code_string_main: _,
            raw_code: _,
            raw_code_init: _,
            raw_code_main: _,
            button_code_init_enabled: _,
            button_code_main_enabled: _,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let filename = path.display().to_string();
                            self.picked_path =
                                path.file_stem().unwrap().to_str().unwrap().to_string();

                            let contents = read_file(filename);
                            let header =
                                RDTHeader::from(&contents).expect("Couldn’t read from RDT file");
                            let init_script_offset = header.offsets[16];
                            let exec_script_offset = header.offsets[17];

                            self.opcode_documentation = init_opcode_documentation();
                            self.code_string = Vec::new();
                            self.raw_code = "".to_owned();
                            (self.code_string_init, self.raw_code_init) =
                                parse_rdt_scd_stream(&contents, init_script_offset);
                            (self.code_string_main, self.raw_code_main) =
                                parse_rdt_scd_stream(&contents, exec_script_offset);
                            self.button_code_init_enabled = true;
                            self.button_code_main_enabled = true;
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("file_list_panel").show(ctx, |ui| {
            ui.heading("Script list");

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                if ui
                    .add_enabled(self.button_code_init_enabled, egui::Button::new("init.scd"))
                    .clicked()
                {
                    self.code_string = self.code_string_init.clone();
                    self.raw_code = self.raw_code_init.clone();
                    self.button_code_init_enabled = false;
                    self.button_code_main_enabled = true;
                }

                if ui
                    .add_enabled(self.button_code_main_enabled, egui::Button::new("main.scd"))
                    .clicked()
                {
                    self.code_string = self.code_string_main.clone();
                    self.raw_code = self.raw_code_main.clone();
                    self.button_code_init_enabled = true;
                    self.button_code_main_enabled = false;
                }

                ui.separator();

                if ui.add(egui::Button::new("Copy code📋")).clicked() {
                    ui.output_mut(|o| o.copied_text = String::from(self.code_string.join("\n")));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(format!("Source code for {}", self.picked_path));
            egui::warn_if_debug_build(ui);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                let keyword_color = egui::Color32::from_rgb(198, 120, 221);
                let function_color = egui::Color32::from_rgb(93, 166, 226);

                let mut function_grouping = Vec::new();
                let mut current_function_code = Vec::new();

                let code_iter = self.code_string.iter();
                for code_line in code_iter {
                    current_function_code.push(code_line.clone());

                    if code_line.contains("End Function") {
                        function_grouping.push(current_function_code);
                        current_function_code = Vec::new();
                    }
                }

                let function_grouping_iter = function_grouping.iter();
                let mut function_num = 0;
                for current_function in function_grouping_iter {
                    egui::CollapsingHeader::new(format!("Function {}", function_num))
                        .default_open(true)
                        .show(ui, |ui| {
                            display_code_function(
                                ui,
                                current_function.clone(),
                                keyword_color,
                                function_color,
                                &self.opcode_documentation,
                            )
                        });
                    function_num += 1;
                }
            });
        });

        egui::SidePanel::right("raw_code_panel").show(ctx, |ui| {
            ui.heading("Raw hex values");
            egui::ScrollArea::both().show(ui, |ui| {
                ui.label(&self.raw_code);
            });
        });
    }
}

fn display_code_function(
    ui: &mut egui::Ui,
    code_string: Vec<String>,
    keyword_color: egui::Color32,
    function_color: egui::Color32,
    opcode_documentation: &HashMap<String, String>,
) {
    let code_iter = code_string.iter();
    let keyword_list = vec![
        "IfStart",
        "ElseStart",
        "EndIf",
        "ForStart",
        "ForEnd",
        "WhileStart",
        "WhileEnd",
        "DoStart",
        "DoEnd",
        "Switch",
        "EndSwitch",
        "Case",
        "Break",
    ];
    for code_line in code_iter {
        let code_line_clone = code_line.clone();
        if code_line_clone.contains("(") {
            let code_line_parts: Vec<&str> = code_line_clone.split("(").collect();
            ui.horizontal(|ui| {
                // Function name should get a different color
                let function_name = code_line_parts[0].clone();
                let mut text_color = function_color;
                if keyword_list.contains(&function_name) {
                    text_color = keyword_color;
                }
                ui.label(egui::RichText::new(function_name).color(text_color))
                    .on_hover_text(match opcode_documentation.get(function_name) {
                        Some(v) => v,
                        None => function_name,
                    });
                // Display function parameters
                ui.label("(".to_owned() + &code_line_parts[1..code_line_parts.len()].join("("));
            });
        } else {
            ui.label(code_line_clone);
        }
    }
}

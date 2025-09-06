use crate::file_handler::{FileHandler, RdtFileData};
use crate::fileio::opcode_data::{
    OPCODE_IF_START, OPCODE_ELSE_START, OPCODE_END_IF,
    OPCODE_FOR_START, OPCODE_FOR_END,
    OPCODE_WHILE_START, OPCODE_WHILE_END,
    OPCODE_DO_START, OPCODE_DO_END,
    OPCODE_SWITCH, OPCODE_END_SWITCH, OPCODE_CASE, OPCODE_BREAK
};
use std::collections::HashMap;

// UI Constants
const KEYWORD_COLOR: egui::Color32 = egui::Color32::from_rgb(198, 120, 221);
const FUNCTION_COLOR: egui::Color32 = egui::Color32::from_rgb(93, 166, 226);

const KEYWORD_LIST: &[&str] = &[
    OPCODE_IF_START,
    OPCODE_ELSE_START, 
    OPCODE_END_IF,
    OPCODE_FOR_START,
    OPCODE_FOR_END,
    OPCODE_WHILE_START,
    OPCODE_WHILE_END,
    OPCODE_DO_START,
    OPCODE_DO_END,
    OPCODE_SWITCH,
    OPCODE_END_SWITCH,
    OPCODE_CASE,
    OPCODE_BREAK,
];

// UI Text Constants
const SCRIPT_LIST_HEADING: &str = "Script list";
const SOURCE_CODE_HEADING_PREFIX: &str = "Source code for";
const RAW_HEX_HEADING: &str = "Raw hex values";
const FUNCTION_HEADING_PREFIX: &str = "Function ";
const COPY_CODE_BUTTON: &str = "Copy code📋";

// Script Names
const INIT_SCRIPT_NAME: &str = "init.scd";
const MAIN_SCRIPT_NAME: &str = "main.scd";

// Function Parsing Constants
const FUNCTION_END_MARKER: &str = "End Function";
const FUNCTION_PARAM_DELIMITER: &str = "(";

/// Contains all UI state and display data
#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct UiState {
    code_string: Vec<String>,
    raw_code: String,
    button_code_init_enabled: bool,
    button_code_main_enabled: bool,
    toast_message: Option<String>,
    toast_timer: f32,
    error_message: Option<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            code_string: Vec::new(),
            raw_code: String::new(),
            button_code_init_enabled: true,
            button_code_main_enabled: true,
            toast_message: None,
            toast_timer: 0.0,
            error_message: None,
        }
    }
}

/// Groups code lines into functions based on "End Function" markers
fn group_code_into_functions(code_string: &[String]) -> Vec<Vec<String>> {
    let mut function_grouping = Vec::new();
    let mut current_function_code = Vec::new();

    for code_line in code_string {
        current_function_code.push(code_line.clone());

        if code_line.contains(FUNCTION_END_MARKER) {
            function_grouping.push(current_function_code);
            current_function_code = Vec::new();
        }
    }

    function_grouping
}

impl TemplateApp {
    /// Shows a toast notification
    fn show_toast(&mut self, message: String) {
        self.ui_state.toast_message = Some(message);
        self.ui_state.toast_timer = 3.0; // Show for 3 seconds
    }

    /// Updates toast timer and clears when expired
    fn update_toast(&mut self, delta_time: f32) {
        if let Some(_) = self.ui_state.toast_message {
            self.ui_state.toast_timer -= delta_time;
            if self.ui_state.toast_timer <= 0.0 {
                self.ui_state.toast_message = None;
            }
        }
    }

    /// Shows an error message that requires manual dismissal
    fn show_error(&mut self, error: String) {
        self.ui_state.error_message = Some(error);
    }

    /// Clears any existing error message
    fn clear_error(&mut self) {
        self.ui_state.error_message = None;
    }

    /// Handles loading and parsing of RDT files
    fn load_rdt_file(&mut self, file_path: &std::path::Path) -> Result<(), String> {
        self.picked_path = file_path.file_stem()
            .ok_or("Invalid file path")?
            .to_str()
            .ok_or("Invalid file name")?
            .to_string();

        // Use the FileHandler to load and parse the file
        let file_data = FileHandler::load_rdt_file(file_path)?;

        // Update the app state
        self.file_data = Some(file_data);
        self.ui_state = UiState::default();

        Ok(())
    }

    /// Renders the top panel with menu bar
    fn render_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("RDT Files", &["rdt"])
                            .pick_file() 
                        {
                            if let Err(e) = self.load_rdt_file(&path) {
                                self.show_error(format!("Error loading file: {}", e));
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });
    }

    /// Renders the left panel with script selection buttons
    fn render_script_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("file_list_panel").show(ctx, |ui| {
            ui.heading(SCRIPT_LIST_HEADING);

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                if ui
                    .add_enabled(self.ui_state.button_code_init_enabled, egui::Button::new(INIT_SCRIPT_NAME))
                    .clicked()
                {
                    self.switch_to_init_script();
                }

                if ui
                    .add_enabled(self.ui_state.button_code_main_enabled, egui::Button::new(MAIN_SCRIPT_NAME))
                    .clicked()
                {
                    self.switch_to_main_script();
                }

                ui.separator();

                if ui.add(egui::Button::new(COPY_CODE_BUTTON)).clicked() {
                    ui.output_mut(|o| o.copied_text = String::from(self.ui_state.code_string.join("\n")));
                    self.show_toast("Code copied to clipboard! 📋".to_string());
                }
            });
        });
    }

    /// Renders the central panel with code display
    fn render_code_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(format!("{} {}", SOURCE_CODE_HEADING_PREFIX, self.picked_path));
            egui::warn_if_debug_build(ui);

            // Render error message (prominent, requires manual dismissal)
            if let Some(ref error) = self.ui_state.error_message {
                ui.add_space(10.0);
                let error_text = error.clone();
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), format!("❌ {}", error_text));
                    if ui.button("×").clicked() {
                        self.clear_error();
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
            }

            // Render toast notification (subtle, auto-dismisses)
            if let Some(ref message) = self.ui_state.toast_message {
                ui.add_space(10.0);
                let message_text = message.clone();
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 100), message_text);
                    if ui.button("×").clicked() {
                        self.ui_state.toast_message = None;
                    }
                });
                ui.add_space(10.0);
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().wrap = Some(false);

                let function_grouping = group_code_into_functions(&self.ui_state.code_string);

                let mut function_num = 0;
                for current_function in function_grouping.iter() {
                    egui::CollapsingHeader::new(format!("{}{}", FUNCTION_HEADING_PREFIX, function_num))
                        .default_open(true)
                        .show(ui, |ui| {
                            display_code_function(
                                ui,
                                current_function.clone(),
                                KEYWORD_COLOR,
                                FUNCTION_COLOR,
                                &self.file_data.as_ref().map(|f| &f.opcode_docs).unwrap_or(&HashMap::new()),
                            )
                        });
                    function_num += 1;
                }
            });
        });
    }

    /// Renders the right panel with raw hex values
    fn render_raw_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("raw_code_panel").show(ctx, |ui| {
            ui.heading(RAW_HEX_HEADING);
            egui::ScrollArea::both().show(ui, |ui| {
                ui.label(&self.ui_state.raw_code);
            });
        });
    }

    /// Switches to the init script and updates button states
    fn switch_to_init_script(&mut self) {
        if let Some(ref file_data) = self.file_data {
            self.ui_state.code_string = file_data.init_script.clone();
            self.ui_state.raw_code = file_data.init_raw.clone();
            self.ui_state.button_code_init_enabled = false;
            self.ui_state.button_code_main_enabled = true;
        }
    }

    /// Switches to the main script and updates button states
    fn switch_to_main_script(&mut self) {
        if let Some(ref file_data) = self.file_data {
            self.ui_state.code_string = file_data.main_script.clone();
            self.ui_state.raw_code = file_data.main_raw.clone();
            self.ui_state.button_code_init_enabled = true;
            self.ui_state.button_code_main_enabled = false;
        }
    }

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

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    file_data: Option<RdtFileData>,
    picked_path: String,
    ui_state: UiState,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            file_data: None,
            picked_path: String::new(),
            ui_state: UiState::default(),
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update toast timer
        self.update_toast(ctx.input(|i| i.unstable_dt));

        self.render_top_panel(ctx, frame);
        self.render_script_panel(ctx);
        self.render_code_panel(ctx);
        self.render_raw_panel(ctx);
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
    for code_line in code_iter {
        if code_line.contains(FUNCTION_PARAM_DELIMITER) {
            let code_line_parts: Vec<&str> = code_line.split(FUNCTION_PARAM_DELIMITER).collect();
            ui.horizontal(|ui| {
                // Function name should get a different color
                let function_name = code_line_parts[0];
                let mut text_color = function_color;
                if KEYWORD_LIST.contains(&function_name) {
                    text_color = keyword_color;
                }
                ui.label(egui::RichText::new(function_name).color(text_color))
                    .on_hover_text(match opcode_documentation.get(function_name) {
                        Some(v) => v,
                        None => function_name,
                    });
                // Display function parameters
                ui.label(FUNCTION_PARAM_DELIMITER.to_owned() + &code_line_parts[1..code_line_parts.len()].join(FUNCTION_PARAM_DELIMITER));
            });
        } else {
            ui.label(code_line);
        }
    }
}

use crate::file_handler::{self, RdtFileData};

mod panels;

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

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    file_data: Option<RdtFileData>,
    picked_path: String,
    ui_state: UiState,
}

impl TemplateApp {
    /// Shows a toast notification
    fn show_toast(&mut self, message: String) {
        self.ui_state.toast_message = Some(message);
        self.ui_state.toast_timer = 3.0; // Show for 3 seconds
    }

    /// Updates toast timer and clears when expired
    fn update_toast(&mut self, delta_time: f32) {
        if self.ui_state.toast_message.is_some() {
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
        self.picked_path = file_path
            .file_stem()
            .ok_or("Invalid file path")?
            .to_str()
            .ok_or("Invalid file name")?
            .to_string();

        let file_data = file_handler::load_rdt_file(file_path)?;

        // Update the app state
        self.file_data = Some(file_data);
        self.ui_state = UiState::default();

        Ok(())
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
        // Always use dark mode, regardless of the OS theme, matching the look users are used to
        // (egui's default is now to follow the system theme, which can otherwise come out light).
        cc.egui_ctx.set_theme(egui::Theme::Dark);

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
    /// Put your widgets into a `Panel`, `CentralPanel`, `Window` or `Area`.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Update toast timer
        self.update_toast(ui.ctx().input(|i| i.unstable_dt));

        // CentralPanel must always be added last: it consumes all remaining space, so any
        // Panel added after it gets none.
        self.render_top_panel(ui);
        self.render_script_panel(ui);
        self.render_raw_panel(ui);
        self.render_code_panel(ui);
    }
}

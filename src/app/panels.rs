//! Rendering for each panel of the main window. State lives on `TemplateApp` (see `super`);
//! this module only draws it and reports user interactions back onto `self`.

use super::TemplateApp;
use crate::fileio::opcode_data::{
    OPCODE_BREAK, OPCODE_CASE, OPCODE_DO_END, OPCODE_DO_START, OPCODE_ELSE_START, OPCODE_END_IF,
    OPCODE_END_SWITCH, OPCODE_FOR_END, OPCODE_FOR_START, OPCODE_IF_START, OPCODE_SWITCH,
    OPCODE_WHILE_END, OPCODE_WHILE_START,
};
use std::collections::HashMap;

// UI Constants
const KEYWORD_COLOR: egui::Color32 = egui::Color32::from_rgb(198, 120, 221);
const FUNCTION_COLOR: egui::Color32 = egui::Color32::from_rgb(93, 166, 226);
const ERROR_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 100, 100);
const TOAST_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 200, 100);

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

impl TemplateApp {
    /// Renders the top panel with menu bar
    // The whole panel is native-only (no File->Quit on web pages!), so `ui` goes unused on wasm32.
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    pub(super) fn render_top_panel(&mut self, ui: &mut egui::Ui) {
        #[cfg(not(target_arch = "wasm32"))]
        egui::Panel::top("top_panel").show(ui, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
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
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    /// Renders the left panel with script selection buttons
    pub(super) fn render_script_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("file_list_panel").show(ui, |ui| {
            ui.heading(SCRIPT_LIST_HEADING);

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                if ui
                    .add_enabled(
                        self.ui_state.button_code_init_enabled,
                        egui::Button::new(INIT_SCRIPT_NAME),
                    )
                    .clicked()
                {
                    self.switch_to_init_script();
                }

                if ui
                    .add_enabled(
                        self.ui_state.button_code_main_enabled,
                        egui::Button::new(MAIN_SCRIPT_NAME),
                    )
                    .clicked()
                {
                    self.switch_to_main_script();
                }

                ui.separator();

                if ui.add(egui::Button::new(COPY_CODE_BUTTON)).clicked() {
                    ui.ctx().copy_text(self.ui_state.code_string.join("\n"));
                    self.show_toast("Code copied to clipboard! 📋".to_string());
                }
            });
        });
    }

    /// Renders the central panel with code display
    pub(super) fn render_code_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(format!(
                "{} {}",
                SOURCE_CODE_HEADING_PREFIX, self.picked_path
            ));
            egui::warn_if_debug_build(ui);

            // Render error message (prominent, requires manual dismissal)
            if let Some(error) = self.ui_state.error_message.clone() {
                if render_dismissible_banner(ui, ERROR_COLOR, &format!("❌ {}", error)) {
                    self.clear_error();
                }
                ui.separator();
                ui.add_space(10.0);
            }

            // Render toast notification (subtle, auto-dismisses)
            if let Some(message) = self.ui_state.toast_message.clone() {
                if render_dismissible_banner(ui, TOAST_COLOR, &message) {
                    self.ui_state.toast_message = None;
                }
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                let function_grouping = group_code_into_functions(&self.ui_state.code_string);

                for (function_num, current_function) in function_grouping.iter().enumerate() {
                    egui::CollapsingHeader::new(format!(
                        "{}{}",
                        FUNCTION_HEADING_PREFIX, function_num
                    ))
                    .default_open(true)
                    .show(ui, |ui| {
                        display_code_function(
                            ui,
                            current_function.clone(),
                            KEYWORD_COLOR,
                            FUNCTION_COLOR,
                            self.file_data
                                .as_ref()
                                .map(|f| &f.opcode_docs)
                                .unwrap_or(&HashMap::new()),
                        )
                    });
                }
            });
        });
    }

    /// Renders the right panel with raw hex values
    pub(super) fn render_raw_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::right("raw_code_panel").show(ui, |ui| {
            ui.heading(RAW_HEX_HEADING);
            egui::ScrollArea::both().show(ui, |ui| {
                ui.label(&self.ui_state.raw_code);
            });
        });
    }
}

/// Renders a colored, dismissible one-line banner (used for both the error and toast
/// notifications, which are otherwise identical apart from color and lifetime).
/// Returns `true` if the dismiss (×) button was clicked.
fn render_dismissible_banner(ui: &mut egui::Ui, color: egui::Color32, text: &str) -> bool {
    let mut dismissed = false;
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.colored_label(color, text);
        if ui.button("×").clicked() {
            dismissed = true;
        }
    });
    ui.add_space(10.0);
    dismissed
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
                ui.label(
                    FUNCTION_PARAM_DELIMITER.to_owned()
                        + &code_line_parts[1..code_line_parts.len()].join(FUNCTION_PARAM_DELIMITER),
                );
            });
        } else {
            ui.label(code_line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn groups_lines_into_separate_functions() {
        let code = lines(&[
            "Start Function 0:",
            "NoOp()",
            "End Function 0",
            "Start Function 1:",
            "EvtEnd()",
            "End Function 1",
        ]);

        let groups = group_code_into_functions(&code);

        assert_eq!(groups.len(), 2);
        assert_eq!(
            groups[0],
            lines(&["Start Function 0:", "NoOp()", "End Function 0"])
        );
        assert_eq!(
            groups[1],
            lines(&["Start Function 1:", "EvtEnd()", "End Function 1"])
        );
    }

    #[test]
    fn empty_input_produces_no_groups() {
        assert!(group_code_into_functions(&[]).is_empty());
    }

    #[test]
    fn lines_without_a_trailing_marker_are_dropped() {
        // group_code_into_functions only closes a group on "End Function", so a trailing
        // partial function with no marker is never pushed into the result.
        let code = lines(&["Start Function 0:", "NoOp()"]);
        assert!(group_code_into_functions(&code).is_empty());
    }
}

use eframe::egui::{self, Color32, Margin, Rounding, Stroke};

pub const MATRIX_NEON_GREEN: Color32 = Color32::from_rgb(0x00, 0xFF, 0x66);
pub const MATRIX_BRIGHT_GREEN: Color32 = Color32::from_rgb(0x00, 0xFF, 0x00);
pub const MATRIX_HOVER_GREEN: Color32 = Color32::from_rgb(0x00, 0xCC, 0x55);
pub const MATRIX_DIM_GREEN: Color32 = Color32::from_rgb(0x1E, 0x5A, 0x1E);
pub const MATRIX_BORDER_GREEN: Color32 = Color32::from_rgb(0x00, 0x64, 0x00);
pub const MATRIX_DEEP_BLACK: Color32 = Color32::from_rgb(0x0A, 0x0A, 0x0A);
pub const MATRIX_PANEL_BG: Color32 = Color32::from_rgb(0x10, 0x1A, 0x10);
pub const MATRIX_INPUT_BG: Color32 = Color32::from_rgb(0x0D, 0x15, 0x0D);
pub const MATRIX_INPUT_HOVER_BG: Color32 = Color32::from_rgb(0x14, 0x28, 0x14);
pub const MATRIX_INPUT_ACTIVE_BG: Color32 = Color32::from_rgb(0x1A, 0x38, 0x1A);

/// Applies the hardened Matrix / Cyberpunk contrast theme to the egui context.
pub fn apply_matrix_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // 1. Backgrounds & Panels
    visuals.dark_mode = true;
    visuals.window_fill = MATRIX_DEEP_BLACK;
    visuals.panel_fill = MATRIX_PANEL_BG;
    visuals.extreme_bg_color = MATRIX_INPUT_BG; // Used for text edit boxes
    visuals.code_bg_color = MATRIX_INPUT_BG;

    visuals.window_rounding = Rounding::same(6.0);
    visuals.window_stroke = Stroke::new(1.0, MATRIX_DIM_GREEN);
    visuals.override_text_color = Some(MATRIX_NEON_GREEN);

    // 2. Text Fields & Input Widgets + Buttons
    let rounding = Rounding::same(4.0);

    // Inactive state: dark background with explicit 1.5px dim Matrix green border
    visuals.widgets.inactive.bg_fill = MATRIX_INPUT_BG;
    visuals.widgets.inactive.weak_bg_fill = MATRIX_INPUT_BG;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.5, MATRIX_DIM_GREEN);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, MATRIX_NEON_GREEN);
    visuals.widgets.inactive.rounding = rounding;

    // Hovered state: glowing neon green stroke with subtle green-tinted background
    visuals.widgets.hovered.bg_fill = MATRIX_INPUT_HOVER_BG;
    visuals.widgets.hovered.weak_bg_fill = MATRIX_INPUT_HOVER_BG;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, MATRIX_NEON_GREEN);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, MATRIX_HOVER_GREEN);
    visuals.widgets.hovered.rounding = rounding;

    // Active / Focused state: solid neon border with dark green fill
    visuals.widgets.active.bg_fill = MATRIX_INPUT_ACTIVE_BG;
    visuals.widgets.active.weak_bg_fill = MATRIX_INPUT_ACTIVE_BG;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, MATRIX_NEON_GREEN);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, MATRIX_BRIGHT_GREEN);
    visuals.widgets.active.rounding = rounding;

    // Open widget state (e.g. open menus/dropdowns)
    visuals.widgets.open.bg_fill = MATRIX_INPUT_HOVER_BG;
    visuals.widgets.open.weak_bg_fill = MATRIX_INPUT_HOVER_BG;
    visuals.widgets.open.bg_stroke = Stroke::new(1.5, MATRIX_NEON_GREEN);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, MATRIX_NEON_GREEN);
    visuals.widgets.open.rounding = rounding;

    // Non-interactive widgets
    visuals.widgets.noninteractive.bg_fill = MATRIX_PANEL_BG;
    visuals.widgets.noninteractive.weak_bg_fill = MATRIX_PANEL_BG;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, MATRIX_BORDER_GREEN);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, MATRIX_NEON_GREEN);
    visuals.widgets.noninteractive.rounding = rounding;

    // Text selection highlight
    visuals.selection.bg_fill = Color32::from_rgb(0x0A, 0x44, 0x18);
    visuals.selection.stroke = Stroke::new(1.0, MATRIX_NEON_GREEN);

    let mut style = (*ctx.style()).clone();
    style.override_font_id = Some(egui::FontId::monospace(13.5));
    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);

    ctx.set_style(style);
    ctx.set_visuals(visuals);
}

/// Custom card frame for dividing UI into distinct visual sections with explicit borders.
pub fn matrix_card_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(MATRIX_PANEL_BG)
        .stroke(Stroke::new(1.0, MATRIX_BORDER_GREEN))
        .rounding(Rounding::same(4.0))
        .inner_margin(Margin::same(12.0))
        .outer_margin(Margin::symmetric(0.0, 4.0))
}

/// Highlighted callout box for documentation and tips.
pub fn callout_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Color32::from_rgb(0x08, 0x14, 0x08))
        .stroke(Stroke::new(1.0, MATRIX_DIM_GREEN))
        .rounding(Rounding::same(4.0))
        .inner_margin(Margin::same(10.0))
        .outer_margin(Margin::symmetric(0.0, 3.0))
}

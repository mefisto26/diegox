use eframe::egui::{self, Color32, Margin, Pos2, Rect, Response, RichText, Rounding, Sense, Stroke, Vec2};

use crate::gui::state::GuiState;
use crate::gui::theme::{MATRIX_DEEP_BLACK, MATRIX_DIM_GREEN, MATRIX_NEON_GREEN};

const LOGO_BYTES: &[u8] = include_bytes!("../../assets/diegox_logo.png");

fn load_logo_texture(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let img = image::load_from_memory(LOGO_BYTES).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        &rgba.into_raw(),
    );
    Some(ctx.load_texture("diegox_logo", color_image, egui::TextureOptions::LINEAR))
}

enum TitlebarButtonType {
    Minimize,
    Maximize(bool), // true = está maximizado (dibujar doble cuadrado)
    Close,
}

fn titlebar_button(
    ui: &mut egui::Ui,
    button_type: TitlebarButtonType,
) -> Response {
    let size = Vec2::new(34.0, 26.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let is_close = matches!(button_type, TitlebarButtonType::Close);

    let (bg_color, stroke_color) = if response.hovered() {
        if is_close {
            (Color32::from_rgb(0x8B, 0x00, 0x00), Color32::from_rgb(0xFF, 0x33, 0x33))
        } else {
            (Color32::from_rgb(0x1E, 0x5A, 0x1E), MATRIX_NEON_GREEN)
        }
    } else {
        (Color32::TRANSPARENT, Color32::from_rgb(0x70, 0xC0, 0x70))
    };

    if response.hovered() {
        ui.painter().rect_filled(rect, Rounding::same(3.0), bg_color);
    }

    let stroke = Stroke::new(1.5, stroke_color);

    match button_type {
        TitlebarButtonType::Close => {
            // "X" perfectamente cuadrada y centrada
            let center = rect.center();
            let half_size = 5.0; // Tamaño de los brazos de la X
            let min = Pos2::new(center.x - half_size, center.y - half_size);
            let max = Pos2::new(center.x + half_size, center.y + half_size);

            ui.painter().line_segment([min, max], stroke);
            ui.painter().line_segment([Pos2::new(max.x, min.y), Pos2::new(min.x, max.y)], stroke);
        }
        TitlebarButtonType::Minimize => {
            // Línea horizontal centrada
            let center = rect.center();
            let half_width = 5.0;
            let y = center.y + 2.0;
            ui.painter().line_segment(
                [Pos2::new(center.x - half_width, y), Pos2::new(center.x + half_width, y)],
                stroke,
            );
        }
        TitlebarButtonType::Maximize(is_maximized) => {
            let center = rect.center();

            if is_maximized {
                // Dibujo de dos cuadrados superpuestos (Restaurar)
                let box_size = 8.0;
                
                // Cuadrado trasero (superior derecho)
                let back_rect = Rect::from_min_size(
                    Pos2::new(center.x - 2.0, center.y - 6.0),
                    Vec2::splat(box_size),
                );
                ui.painter().rect_stroke(back_rect, Rounding::ZERO, stroke);

                // Fondo para tapar el traslape del cuadrado trasero
                let front_rect = Rect::from_min_size(
                    Pos2::new(center.x - 6.0, center.y - 2.0),
                    Vec2::splat(box_size),
                );
                ui.painter().rect_filled(front_rect, Rounding::ZERO, bg_color);
                ui.painter().rect_stroke(front_rect, Rounding::ZERO, stroke);
            } else {
                // Cuadrado único centrado (Maximizar)
                let box_size = 10.0;
                let box_rect = Rect::from_center_size(center, Vec2::splat(box_size));
                ui.painter().rect_stroke(box_rect, Rounding::ZERO, stroke);
            }
        }
    }

    response
}

pub fn draw_custom_titlebar(state: &mut GuiState, ctx: &egui::Context) {
    egui::TopBottomPanel::top("custom_titlebar")
        .frame(
            egui::Frame::none()
                .fill(MATRIX_DEEP_BLACK)
                .stroke(Stroke::new(1.0, MATRIX_DIM_GREEN))
                .inner_margin(Margin::symmetric(10.0, 5.0)),
        )
        .show(ctx, |ui| {
            let bar_height = 28.0;

            ui.horizontal(|ui| {
                if state.logo_texture.is_none() {
                    state.logo_texture = load_logo_texture(ui.ctx());
                }

                // 1. Logo y título
                if let Some(texture) = &state.logo_texture {
                    ui.add(egui::Image::new(texture).max_height(24.0));
                    ui.add_space(6.0);
                }

                ui.label(
                    RichText::new("DIEGOX Suite | Post-Quantum Security")
                        .strong()
                        .color(MATRIX_NEON_GREEN),
                );

                // 2. Botones vectoriales a la derecha
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Botón Cerrar (X)
                    if titlebar_button(ui, TitlebarButtonType::Close)
                        .on_hover_text("Close DIEGOX Suite")
                        .clicked()
                    {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    // Botón Maximizar / Restaurar (□ / ⧉)
                    let is_maximized = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
                    if titlebar_button(ui, TitlebarButtonType::Maximize(is_maximized))
                        .on_hover_text(if is_maximized { "Restore" } else { "Maximize" })
                        .clicked()
                    {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                    }

                    // Botón Minimizar (-)
                    if titlebar_button(ui, TitlebarButtonType::Minimize)
                        .on_hover_text("Minimize")
                        .clicked()
                    {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    }

                    // 3. Área central arrastrable
                    let remaining_size = Vec2::new(ui.available_width().max(0.0), bar_height);
                    let (_drag_rect, drag_resp) = ui.allocate_exact_size(
                        remaining_size,
                        Sense::click_and_drag(),
                    );

                    if drag_resp.dragged() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }

                    if drag_resp.double_clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                    }
                });
            });
        });
}
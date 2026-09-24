use eframe::egui;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

// All core modules are exported from the `diegox` library root (`src/lib.rs`),
// keeping the binary and integration tests (`tests/`) consuming the same API.
use diegox::engine::QuantumEngine;
use diegox::gui;

// Carga los bytes del ícono desde la raíz en tiempo de compilación
const APP_ICON_BYTES: &[u8] = include_bytes!("../app_icon.ico");

fn load_app_icon() -> Option<egui::IconData> {
    let img = image::load_from_memory(APP_ICON_BYTES).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    })
}

fn main() -> eframe::Result<()> {
    let trng = ChaCha20Rng::from_entropy();
    let system_engine = QuantumEngine::new(trng);

    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(true)
            .with_min_inner_size([700.0, 500.0])
            .with_inner_size([920.0, 680.0]),
        ..Default::default()
    };

    if let Some(icon) = load_app_icon() {
        options.viewport = options.viewport.with_icon(std::sync::Arc::new(icon));
    }

    eframe::run_native(
        "DIEGOX Suite",
        options,
        Box::new(|cc| {
            gui::theme::apply_matrix_theme(&cc.egui_ctx);
            Box::new(gui::CsrpGui::new(system_engine))
        }),
    )
}

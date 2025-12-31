use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

#[derive(Resource, Default)]
struct UiState {
    text: String,
    fonts_loaded: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .init_resource::<UiState>()
        .add_systems(Startup, setup_camera_system)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    // ★ フォントは一度だけ登録する
    if !state.fonts_loaded {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "noto".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../assets/fonts/NotoSansCJK-Regular.ttc"
            ))
            .into(), // ★ ここが重要
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "noto".to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "noto".to_owned());

        ctx.set_fonts(fonts);
        state.fonts_loaded = true;
    }


    egui::Window::new("日本語入力テスト")
        .default_size([400.0, 200.0])
        .show(ctx, |ui| {
            ui.label("日本語が表示されるはずです");
            ui.text_edit_multiline(&mut state.text);
        });

    Ok(())
}

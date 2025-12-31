use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

#[derive(Resource, Default)]
struct UiState {
    text: String,
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

// ★ 公式 example と同じ「Result を返す system」
fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
) -> Result {
    let ctx = contexts.ctx_mut()?; // QuerySingleError がそのまま ? で流れる

    egui::Window::new("日本語入力テスト")
        .default_size([400.0, 200.0])
        .show(ctx, |ui| {
            ui.label("ここに日本語を入力してください");
            ui.text_edit_multiline(&mut state.text);
        });

    Ok(())
}

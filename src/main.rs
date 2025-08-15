use bevy::prelude::*;
mod zakurust_editor;
mod zakurust_engine;
#[derive(Component)]
struct CameraMarker;

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    commands.spawn((Camera2d::default(), CameraMarker));
    // zakurust_engine::scene_creation::scene_creation(commands, asset_server, window);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}

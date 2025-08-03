// Calls other loaders to create scene for the game, camera in main
use bevy::prelude::*;

pub fn scene_creation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single().unwrap();
    let background_image = "bg1.jpeg";
    let background_size = Vec2::new(window.width(), window.height());

    let background_sprite = Sprite {
        image: asset_server.load(background_image),
        custom_size: Some(background_size),
        ..default()
    };

    commands.spawn((
        background_sprite,
        Transform::default(),
        GlobalTransform::default(),
    ));
}

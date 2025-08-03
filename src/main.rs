use bevy::prelude::*;

#[derive(Component)]
struct CameraMarker;

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    commands.spawn((Camera2d::default(), CameraMarker));

    // window dimensions
    let window = window.single().unwrap();
    let size = Vec2::new(window.width(), window.height());
    let char_size = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let db_size = Vec2::new(window.width() / 3.0, window.height() / 3.0);

    // paths assume asset folder already so omit them
    let bg_sprite = Sprite {
        image: asset_server.load("bg1.jpeg"),
        custom_size: Some(size),
        ..Default::default()
    };
    commands.spawn((bg_sprite, Transform::default(), GlobalTransform::default()));

    let char_sprite = Sprite {
        image: asset_server.load("girla.jpeg"),
        custom_size: Some(char_size),
        ..Default::default()
    };
    commands.spawn((
        char_sprite,
        Transform::default(),
        GlobalTransform::default(),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}

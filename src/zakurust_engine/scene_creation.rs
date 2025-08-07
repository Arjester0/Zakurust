// Calls other loaders to create scene for the game, camera in main
use crate::zakurust_engine::script_interpreter::{load_script_from_file, ScriptCommand};
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

    let d_box_size = Vec2::new(window.width() * 0.8, window.height() * 0.2);
    let d_box_pos = Vec2::new(0.0, -window.height() * 0.4);
    let d_box_font = TextFont {
        font: asset_server.load("fonts/Roboto-Regular.ttf"),
        font_size: 35.0,
        ..default()
    };
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.1, 0.1, 0.1), d_box_size),
            Transform::from_translation(d_box_pos.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("balls\n"),
                d_box_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
                // TextBounds::from(d_box_size),
                Transform::from_translation(Vec3::Z),
            ));
        });

    script_spawn(commands, asset_server);
}

fn script_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let script = load_script_from_file("assets/scripts/ex.yaml").expect("Failed to load script");
    for cmd in script.iter() {
        match cmd {
            ScriptCommand::CharacterLine {
                name,
                expression,
                line,
            } => {
                println!("{} [{}]: {}", name, expression, line);
                // Pass (name, expression) to your character spawning system
                // let character = name;
                let character_image = name.to_string() + expression + ".jpeg";

                let character_sprite = Sprite {
                    image: asset_server.load(character_image),
                    ..Default::default()
                };

                commands.spawn((
                    character_sprite,
                    Transform::default(),
                    GlobalTransform::default(),
                ));
            }
            ScriptCommand::Choice {
                prompt,
                option1,
                option2,
            } => {
                println!("Choice: {}\nA: {}\nB: {}", prompt, option1, option2);
                // Pass to your choice UI renderer
            }
        }
    }
}

//! Displays a single [`Sprite`], created from an image.

use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;

#[derive(Component)]
struct SpriteMark;

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, moving_sprite.run_if(input_pressed(KeyCode::KeyM)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
        Transform::from_xyz(200.0, 200.0, 0.0),
        SpriteMark,
    ));
}

fn moving_sprite(mut sprite_query: Option<Single<(&mut Transform, &SpriteMark)>>) {
    if let Some(mut sprite) = sprite_query {
        sprite.0.translation.x = 0.0;
        sprite.0.translation.y = 0.0;
    }
}

//! Displays a single [`Sprite`], created from an image.

use std::slice::Windows;
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
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
        .add_systems(FixedUpdate, rotate_sprite)
        .add_systems(Update, moving_sprite.run_if(input_just_pressed(KeyCode::KeyM)))
        .add_systems(Update, moving_window.run_if(input_just_pressed(MouseButton::Left)))
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

fn moving_window(mut windows: Query<&mut Window>,) {
    windows.iter_mut().for_each(|mut w| {
        w.start_drag_move()
    })
}

fn moving_sprite(
    res: Res<Assets<Image>>,
    mut sprite_query: Option<Single<(&mut Transform, &Sprite, &SpriteMark)>>,
) {
    if let Some(mut sprite) = sprite_query {
        sprite.0.translation.x = 0.0;
        sprite.0.translation.y = 0.0;

        if let Some(image) = res.get(&sprite.1.image) {
            info!(
                "width:{}, height:{}",
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height
            );
        }
    }
}

fn rotate_sprite(time: Res<Time>, mut sprite_query: Option<Single<(&mut Transform, &SpriteMark)>>) {
    if let Some(mut sprite) = sprite_query {
        // sprite.0.rotation = Quat::from_rotation_z(time.delta_secs().to_radians())
        sprite.0.rotate_z(time.delta_secs().to_radians() * 10.0)
    }
}

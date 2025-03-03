use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowTheme};

#[derive(Component)]
struct SpriteMark;

#[derive(Component)]
struct SpriteMoving {
    x_velocity: f32,
}

const MOVING_SPEED: f32 = 200.0;

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("bevy_step_by_step"),
                resolution: (600f32, 600f32).into(),
                window_theme: Some(WindowTheme::Light),
                // position: WindowPosition::At((0i32, 0i32).into()),
                // decorations: false,
                // mode: WindowMode::Fullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            show_window_size.run_if(input_just_pressed(KeyCode::KeyP)),
        )
        .add_systems(FixedUpdate, moving_sprite)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
        Transform::from_xyz(200.0, 200.0, 0.0),
        SpriteMark,
        SpriteMoving {
            x_velocity: MOVING_SPEED,
        },
    ));
}

fn show_window_size(windows: Query<&Window>) {
    windows.iter().for_each(|w| {
        let width = w.resolution.size().x;
        let height = w.resolution.size().y;

        info!("window.size: {}, {}", width, height);
    });
}

fn moving_sprite(
    res: Res<Assets<Image>>,
    time_res: Res<Time>,
    mut windows: Query<&Window>,
    mut sprite_query: Query<(&mut Transform, &Sprite, &mut SpriteMoving), With<SpriteMark>>,
) {
    let _ = windows.get_single().map(|w| {
        let width = w.resolution.size().x;

        sprite_query.iter_mut().for_each(|(mut t, s, mut sm)| {
            if let Some(image) = res.get(&s.image) {
                let image_width = image.texture_descriptor.size.width;

                if sm.x_velocity > 0f32 {
                    if ((image_width / 2) as f32 + t.translation.x) > (width / 2f32) {
                        sm.x_velocity = -MOVING_SPEED;
                    }
                } else if sm.x_velocity < 0f32 {
                    if (t.translation.x - ((image_width / 2) as f32)) < (-width / 2f32) {
                        sm.x_velocity = MOVING_SPEED;
                    }
                }

                t.translation.x += time_res.delta_secs() * sm.x_velocity;
            }
        });
    });
}

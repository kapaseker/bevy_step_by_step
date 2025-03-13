use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    let run_sprite = |mut query: Query<&mut SpriteState>| {
        query.iter_mut().for_each(|mut s|{
            s.0 = State::RUN
        })
    };

    let halt_sprite = |mut query: Query<&mut SpriteState>| {
        query.iter_mut().for_each(|mut s|{
            s.0 = State::HALT
        })
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_sprite, spawn_sprite_sheet))
        .add_systems(Update, (move_sprite, animate_sprite))
        .add_systems(Update, run_sprite.run_if(input_just_pressed(KeyCode::KeyR)))
        .add_systems(Update, halt_sprite.run_if(input_just_pressed(KeyCode::KeyH)))
        .run();
}

const SPEED: f32 = 800.0;

fn move_sprite(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite_query: Query<(&mut Transform, &Sprite), With<Sprite>>,
) {
    let delta_time = time.delta_secs();
    let mut x = 0f32;
    let mut y = 0f32;

    if keyboard_input.pressed(KeyCode::KeyW) {
        y = 1f32;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        y = -1f32;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        x = -1f32;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        x = 1f32;
    }

    sprite_query.iter_mut().for_each(|(mut t, s)| {
        // 单位化是有坑的，可能不能正常单位化，所以是用了try_normalize
        Vec3::new(x ,y, 0.0)
            .try_normalize()
            .map(|v| {
                t.translation += v * delta_time * SPEED;
            });
    })
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(-200f32, -200f32, 0f32),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("cloud.png"),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::from_xyz(-200f32, -200f32, -1f32),
    ));
}

#[derive(Component)]
struct SpriteIndex(usize, usize);

#[derive(Component)]
struct SpriteTimer(Timer);

#[derive(Component)]
struct SpriteState(State);

enum State {
    HALT,
    RUN,
}

fn spawn_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = SpriteIndex(1, 6);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        SpriteState(State::HALT),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        SpriteTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &SpriteIndex,
        &mut SpriteTimer,
        &mut Sprite,
        &mut SpriteState,
    )>,
) {
    query
        .iter_mut()
        .for_each(|(index, mut timer, mut sprite, mut state)| match state.0 {
            State::HALT => {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = 0
                }
            }
            State::RUN => {
                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = if atlas.index == index.1 {
                            index.0
                        } else {
                            atlas.index + 1
                        };
                    }
                }
            }
        })
}

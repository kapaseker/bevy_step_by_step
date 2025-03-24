use bevy::input::common_conditions::{input_just_pressed, input_just_released};
use bevy::prelude::*;
use bevy::DefaultPlugins;

const SPEED: f32 = 800.0;

#[derive(Component)]
struct Planet {
    name: String,
    size: u32,
    init_pos: (f32, f32),
    ///移动，左上右下的顺序
    moving: (KeyCode, KeyCode, KeyCode, KeyCode),
}

struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(ob_planet)
            .add_systems(Update, moving_planet);
    }
}

fn moving_planet(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite_query: Query<(&mut Transform, &Sprite, &Planet)>,
) {
    let delta_time = time.delta_secs();

    sprite_query.iter_mut().for_each(|(mut t, s, p)| {

        let mut x = 0f32;
        let mut y = 0f32;

        if keyboard_input.pressed(p.moving.1) {
            y = 1f32;
        } else if keyboard_input.pressed(p.moving.3) {
            y = -1f32;
        }

        if keyboard_input.pressed(p.moving.0) {
            x = -1f32;
        } else if keyboard_input.pressed(p.moving.2) {
            x = 1f32;
        }

        // 单位化是有坑的，可能不能正常单位化，所以是用了try_normalize
        Vec3::new(x, y, 0.0).try_normalize().map(|v| {
            t.translation += v * delta_time * SPEED;
        });
    })
}

fn ob_planet(
    trigger: Trigger<OnAdd, Planet>,
    query: Query<&Planet>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let entity = trigger.entity();

    let _ = query.get(entity).map(|planet| {
        commands.entity(entity).insert((
            Sprite {
                image: asset_server.load(format!("{}.png", planet.name)),
                custom_size: Some(Vec2::new(planet.size as f32, planet.size as f32)),
                ..default()
            },
            Transform::from_xyz(planet.init_pos.0, planet.init_pos.1, 0f32),
        ));
    });
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, add_role.run_if(input_just_pressed(KeyCode::Space)))
        // .add_observer(on_trigger)
        .add_plugins(PlanetPlugin)
        .add_systems(
            Update,
            add_planet.run_if(input_just_released(KeyCode::Digit1)),
        )
        .run();
}

#[derive(Component)]
struct Role(String);

fn add_planet(mut commands: Commands) {
    commands.spawn(Planet {
        name: "alpine".to_string(),
        size: 48u32,
        init_pos: (-100.0, 0.0),
        moving: (KeyCode::KeyA, KeyCode::KeyW, KeyCode::KeyD, KeyCode::KeyS),
    });

    commands.spawn(Planet {
        name: "ocean".to_string(),
        size: 32u32,
        init_pos: (100.0, 0.0),
        moving: (
            KeyCode::ArrowLeft,
            KeyCode::ArrowUp,
            KeyCode::ArrowRight,
            KeyCode::ArrowDown,
        ),
    });
}

fn on_trigger(trigger: Trigger<OnAdd, Role>, query: Query<&Role>, mut role_count: Local<usize>) {
    let _ = query.get(trigger.entity()).map(|role| {
        *role_count += 1;
        info!(
            "entity {} add to app, all count is: {}",
            role.0, *role_count
        );
    });
}

fn add_role(mut id: Local<i32>, mut commands: Commands) {
    commands.spawn(Role(String::from(id.to_string())));
    *id += 10;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

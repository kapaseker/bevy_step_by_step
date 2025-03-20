use bevy::input::common_conditions::{input_just_pressed, input_just_released};
use bevy::prelude::*;
use bevy::DefaultPlugins;

#[derive(Component)]
struct Role(String);

#[derive(Component)]
struct Planet {
    name: String,
    size: u32,
    radius: f32,
}

struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(ob_planet);
    }
}

fn ob_planet(
    trigger: Trigger<OnAdd, Planet>,
    query: Query<&Planet>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _ = query.get(trigger.entity()).map(|planet| {
        commands.spawn(Sprite {
            image: asset_server.load(format!("{}.png", planet.name)),
            custom_size: Some(Vec2::new(planet.size as f32, planet.size as f32)),
            ..default()
        });
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
        .add_systems(Update, add_role.run_if(input_just_pressed(KeyCode::Space)))
        .add_observer(on_trigger)
        .add_plugins(PlanetPlugin)
        .add_systems(Update, add_planet.run_if(input_just_released(KeyCode::Digit1)))
        .run();
}

fn add_planet(mut commands: Commands) {
    commands.spawn(Planet {
        name: "alpine".to_string(),
        size: 48u32,
        radius: 1.0,
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

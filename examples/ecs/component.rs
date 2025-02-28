use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;
use bevy::DefaultPlugins;


#[derive(Component)]
struct Health(f32);

impl Default for Health  {
    fn default() -> Self {
        Self(100.0)
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
#[require(Health)]
struct Player;

#[derive(Component)]
struct Enemy;


fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_entities)
        .add_systems(Update, change_player.run_if(input_just_pressed(KeyCode::KeyP)))
        .run();
}

fn setup_entities(mut commands: Commands) {
    commands.spawn((Player, Position { x: 100f32, y: 200f32 }));
    commands.spawn((Health(72.0), Enemy, Position { x: 20f32, y: 60f32 }));
}

fn change_player(
    mut player_query:Query<(&mut Health, &Player)>
) {
    for (mut health, _) in player_query.iter_mut() {
        health.0 -= 1f32;
        info!("change player {}", health.0);
    }
}

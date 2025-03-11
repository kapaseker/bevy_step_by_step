use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::DefaultPlugins;

#[derive(Event)]
struct PlayerDetected(i32);

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_event::<PlayerDetected>()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            send_player_detected.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, on_player_detected)
        .run();
}

fn send_player_detected(mut events: EventWriter<PlayerDetected>) {
    events.send(PlayerDetected(1));
}

fn on_player_detected(mut events: EventReader<PlayerDetected>) {
    events
        .read()
        .for_each(|ev| info!("detected player: {}", ev.0));
}

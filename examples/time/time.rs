use std::ops::Deref;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::keyboard::KeyboardInput;
use bevy::time::Stopwatch;

#[derive(Component)]
struct OnceTimer(Timer);

#[derive(Component)]
struct RepeatingTimer(Timer);

#[derive(Component)]
struct RunWatch(Stopwatch);

fn main() {

    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, set_up_timer)
        .add_systems(Update, time_info.run_if(input_just_pressed(KeyCode::Space)))
        .add_systems(Update, reset_once_timer.run_if(input_just_pressed(KeyCode::KeyR)))
        .add_systems(Update, update_timer)
        .add_systems(Startup, setup_watch)
        .add_systems(Update, run_watch)
        .run();
}

fn setup_watch(
    mut commands: Commands,
) {
    commands.spawn(RunWatch(Stopwatch::new()));
}

fn run_watch(
    time: Res<Time>,
    mut watch: Query<&mut RunWatch>,
    key_input:Res<ButtonInput<KeyCode>>
) {
    watch.iter_mut().for_each(|mut run| {
        run.0.tick(time.delta());
        if key_input.just_pressed(KeyCode::Enter) {
            info!("time gone: {}", run.0.elapsed_secs());
        }
    })
}


fn set_up_timer(
    mut commands: Commands,
) {
    commands.spawn(OnceTimer(Timer::new(Duration::from_secs(3), TimerMode::Once)));
    commands.spawn(RepeatingTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)));


}

fn reset_once_timer(
    mut once_timer: Query<&mut OnceTimer>,
) {
    once_timer.iter_mut().for_each(|mut timer| {
        timer.0.reset();
    });
}

fn update_timer(
    time: Res<Time>,
    mut once_timer: Query<&mut OnceTimer>,
    mut repeating_timer: Query<&mut RepeatingTimer>,
) {
    let delta = time.delta();
    once_timer.iter_mut().for_each(|mut timer| {
        timer.0.tick(delta);
        if timer.0.just_finished() {
            info!("Once timer just finished");
        }
    });

    repeating_timer.iter_mut().for_each(|mut timer| {
        timer.0.tick(delta);
        if timer.0.just_finished() {
            info!("Repeating timer just finished");
        }
    });
}

fn time_info(
    time: Res<Time>,
) {
    info!("Elapsed Time: {:?}", time.elapsed_secs());
}
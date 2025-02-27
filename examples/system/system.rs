use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use std::slice::Windows;
use bevy::input::common_conditions::input_just_pressed;

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (400f32, 400f32).into(),
                ..default()
            }),
            ..default()
        }))
        // .add_systems(Startup, || {
        //     info!("Startup");
        // })
        // .add_systems(Update, || {
        //     info!("Updating");
        // })
        // .add_systems(FixedUpdate, || {
        //     info!("FixedUpdate");
        // })
        // .add_systems(Update, system_a.after(system_b))
        // .add_systems(Update, system_a.before(system_b))
        // .add_systems(Update, (system_c, system_b, system_a).chain())
        // .add_systems(Update, system_a.run_if(input_just_pressed(KeyCode::Space)))
        .add_systems(Update, system_c.run_if(time_passed(2f32)))
        .run();
}

fn update_system() {
    info!("Updating system");
}

fn system_a() {
    info!("system_a")
}
fn system_b() {
    info!("system_b")
}
fn system_c() {
    info!("system_c")
}

fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        // Tick the timer
        *timer += time.delta_secs();
        // Return true if the timer has passed the time
        *timer >= t
    }
}

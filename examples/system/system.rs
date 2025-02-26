use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use std::slice::Windows;

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
        .add_systems(Startup, || {
            info!("Startup");
        })
        .add_systems(Update, || {
            info!("Updating");
        })
        .add_systems(FixedUpdate, || {
            info!("FixedUpdate");
        })
        .run();
}

fn update_system() {
    info!("Updating system");
}

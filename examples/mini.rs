use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {

    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, || {
            info!("Updating system");
        })
        .run();
}

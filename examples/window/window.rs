use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowTheme};

fn main() {
    // #[cfg(target_os = "windows")]
    // {
    //     std::env::set_var("WGPU_BACKEND", "dx12");
    // }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("bevy_step_by_step"),
                resolution: (600f32, 600f32).into(),
                window_theme: Some(WindowTheme::Light),
                position: WindowPosition::At((0i32, 0i32).into()),
                decorations: false,
                // mode: WindowMode::Fullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0f32, 1f32, 1f32)))
        .add_systems(
            Update,
            moving_window.run_if(input_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            exit_app.run_if(input_just_pressed(KeyCode::Escape)),
        )
        .run();
}

fn moving_window(mut windows: Query<&mut Window>) {
    windows.iter_mut().for_each(|mut w| w.start_drag_move())
}

fn exit_app(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

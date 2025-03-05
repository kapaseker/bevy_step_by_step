use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {

    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (key_combine_press, key_pressed, mouse_press, touch_system, cursor_events))
        .run();
}

fn key_pressed(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        info!("Jump");
    }
}

fn key_combine_press(input: Res<ButtonInput<KeyCode>>) {
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if ctrl && shift && input.just_pressed(KeyCode::KeyC) {
        info!("All select");
    }
}

fn mouse_press(mouse: Res<ButtonInput<MouseButton>>, key: Res<ButtonInput<KeyCode>>) {
    if mouse.just_pressed(MouseButton::Left) && key.pressed(KeyCode::ControlLeft) {
        info!("Back");
    }
}

fn cursor_events(mut cursor_evr: EventReader<CursorMoved>) {
  for ev in cursor_evr.read() {
    println!(
      "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
      ev.position.x, ev.position.y, ev.window
    );
  }
}

fn touch_system(touches: Res<Touches>) {
  for touch in touches.iter_just_pressed() {
    info!(
      "just pressed touch with id: {:?}, at: {:?}",
      touch.id(),
      touch.position()
    );
  }

  for touch in touches.iter_just_released() {
    info!(
      "just released touch with id: {:?}, at: {:?}",
      touch.id(),
      touch.position()
    );
  }

  for touch in touches.iter_just_canceled() {
    info!("canceled touch with id: {:?}", touch.id());
  }

  // you can also iterate all current touches and retrieve their state like this:
  for touch in touches.iter() {
    info!("active touch: {:?}", touch);
    info!("  just_pressed: {}", touches.just_pressed(touch.id()));
  }
}
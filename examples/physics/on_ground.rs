use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::na::Translation;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Ground;

#[derive(Component)]
#[require(MovementAcceleration)]
struct Player;

#[derive(Component)]
struct Gravity(Vec3);

#[derive(Component)]
struct OnGround;

#[derive(Event)]
enum Movement {
    Move(Vec2),
    Jump,
}

#[derive(Component)]
struct MovementAcceleration {
    on_ground: f32,
    jump: f32,
    damping: f32,
}

impl Default for MovementAcceleration {
    fn default() -> Self {
        Self {
            on_ground: 20.0,
            jump: 10.0,
            damping: 0.6,
        }
    }
}

fn main() {
    App::new()
        // Enable physics
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            PanOrbitCameraPlugin,
        ))
        .add_event::<Movement>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, apply_damping, update_grounded),
        )
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Fixed,
        Collider::cylinder(0.05, 36.0),
        Mesh3d(meshes.add(Cylinder::new(36.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Ground,
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    let player_shape = Capsule3d::new(0.4, 1.0);
    let collider = Collider::capsule_y(0.5, 0.4);
    // let shape_cast = collider.clone().set_scale(Vector::ONE * 0.99, 10);
    commands.spawn((
        Mesh3d(meshes.add(player_shape)),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 4.0, 0.0),
        // RigidBody::KinematicVelocityBased,
        collider,
        KinematicCharacterController {
            custom_mass: Some(5.0),
            translation: Some(Vect::ZERO),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: false,
            }),
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: None,
            ..default()
        },
        // Velocity {
        //     linvel: Vec3::ZERO,
        //     angvel: Vec3::ZERO,
        // },
        // ShapeCaster::new(
        //     shape_cast.into(),
        //     Vector::ZERO,
        //     Quaternion::default(),
        //     Dir3::NEG_Y,
        // )
        // .with_max_distance(0.2),
        Gravity(Vec3::NEG_Y * 9.81 * 2.0),
        Player,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        PanOrbitCamera::default(),
        Transform::from_xyz(0.0, 12.0, 18.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<Movement>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as f32, vertical as f32).normalize();

    if direction != Vec2::ZERO {
        movement_event_writer.send(Movement::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(Movement::Jump);
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(mut commands: Commands,
                   rapier_context: ReadRapierContext,
                   mut query: Query<(Entity,&Transform), With<Player>>
) {
    for (entity, transform) in &mut query {

        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let context = rapier_context.single();

        // Then cast the ray.
        let hit = context.cast_ray(
            transform.translation,
            Vect::NEG_Y,
            0.1f32.into(),
            true,
            QueryFilter::only_kinematic(),
        );

        if hit.is_some() {
            commands.entity(entity).insert(OnGround);
        } else {
            commands.entity(entity).remove::<OnGround>();
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut movement_event_reader: EventReader<Movement>,
    mut players: Query<(&MovementAcceleration, &mut KinematicCharacterController, Has<OnGround>, &Gravity)>,
) {
    let delta = time.delta_secs();

    for x in movement_event_reader.read() {
        players.iter_mut().for_each(|(acc, mut controller, on_ground, gravity)| {
            match x {
                Movement::Move(dir) => {
                    let y = if on_ground { 0.0 } else { gravity.0.y * delta };
                    controller.translation = Some(
                        Vect::new(dir.x * delta, y, dir.x * delta)
                    )
                }
                Movement::Jump => {
                    if on_ground {
                        // vel.linvel.y = acc.jump
                    }
                }
            }
        })
    }
}

fn apply_damping(mut query: Query<(&MovementAcceleration, &mut Velocity)>) {
    query.iter_mut().for_each(|(acc, mut vel)| {
        // vel.z *= acc.damping;
        // vel.x *= acc.damping;
        // info!("apply_damping :{}", vel.0);
    })
}

// fn apply_gravity(time: Res<Time>, mut controllers: Query<(&Gravity, &mut KinematicCharacterController)>) {
//     // Precision is adjusted so that the example works with
//     // both the `f32` and `f64` features. Otherwise you don't need this.
//     let delta_time = time.delta_secs();
//
//     for (gravity, mut controller) in &mut controllers {
//         if let Some(mut translation) = controller.translation {
//             translation.y = gravity.0.y * delta_time;
//         }
//     }
// }

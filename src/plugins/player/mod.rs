use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::{color::palettes::css, prelude::*, scene::SceneInstanceReady};
use bevy_tnua::{
    TnuaUserControlsSystemSet,
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::{
    InputManagerBundle,
    prelude::{ActionState, InputMap},
};

use crate::{MyAssets, MyStates};

use super::input::Action;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<Player>()
            .add_systems(OnEnter(MyStates::Next), (setup_player, apply_controls))
            .add_systems(Update, jump)
            .add_systems(
                FixedUpdate,
                apply_controls.in_set(TnuaUserControlsSystemSet),
            )
            .add_observer(
                // log the component from the gltf spawn
                |trigger: Trigger<SceneInstanceReady>,
                 children: Query<&Children>,
                 characters: Query<&Player>| {
                    for entity in children.iter_descendants(trigger.entity()) {
                        let Ok(character) = characters.get(entity) else {
                            continue;
                        };
                        info!(?character);
                    }
                },
            );
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Player {
    name: String,
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    my_assets: Res<MyAssets>,
) {
    let input_map = InputMap::new([
        (Action::Walk, KeyCode::ArrowUp),
        (Action::Jump, KeyCode::KeyK),
    ]);

    commands.spawn((
        // Mesh3d(meshes.add(Capsule3d {
        //     radius: 0.5,
        //     half_length: 0.5,
        // })),
        MeshMaterial3d(materials.add(Color::from(css::DARK_CYAN))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        // The player character needs to be configured as a dynamic rigid body of the physics
        // engine.
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
        // This is Tnua's interface component.
        TnuaController::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED,
        SceneRoot(my_assets.player.clone()),
        InputManagerBundle::with_map(input_map),
    ));
}

fn apply_controls(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut TnuaController>) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    if keyboard.pressed(KeyCode::Space) {}
}

// Query for the `ActionState` component in your game logic systems!
fn jump(
    action_query: Query<&ActionState<Action>, With<SceneRoot>>,
    mut controller_query: Query<&mut TnuaController>,
) {
    let Ok(action_state) = action_query.get_single() else {
        println!("No player");
        return;
    };

    let Ok(mut controller) = controller_query.get_single_mut() else {
        return;
    };

    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&Action::Jump) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}

use avian3d::{
    PhysicsPlugins,
    prelude::{Collider, RigidBody},
};
use bevy::{color::palettes::css::GRAY, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_skein::SkeinPlugin;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use plugins::{camera::CameraPlugin, player::PlayerPlugin};

mod plugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading).load_collection::<UgalaBugala>(),
        )
        .add_plugins((CameraPlugin, PlayerPlugin))
        .add_systems(Startup, (setup_level))
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct UgalaBugala {
    #[asset(path = "demo.gltf")]
    pub player: Handle<Scene>,
}

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_assets: Res<UgalaBugala>,
) {
    // Spawn the ground.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(128.0, 128.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));

    // Spawn a little platform for the player to jump on.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(Color::from(GRAY))),
        Transform::from_xyz(-6.0, 2.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(4.0, 1.0, 4.0),
    ));

    commands.spawn(SceneRoot(mesh_assets.player.clone()));
}

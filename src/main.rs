use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_skein::SkeinPlugin;

fn main() {
    App::new()
        .register_type::<Character>()
        .register_type::<Ground>()
        .add_plugins((DefaultPlugins, SkeinPlugin::default()))
        .add_observer(
            // log the component from the gltf spawn
            |trigger: Trigger<SceneInstanceReady>,
             children: Query<&Children>,
             characters: Query<&Character>| {
                for entity in children.iter_descendants(trigger.entity()) {
                    let Ok(character) = characters.get(entity) else {
                        continue;
                    };
                    info!(?character);
                }
            },
        )
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Character {
    name: String,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Ground;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        SceneRoot(asset_server.load(
            // Change this to your exported gltf file
            GltfAssetLabel::Scene(0).from_asset("demo.gltf"),
        )),
        Transform::from_xyz(0., 1., 0.),
    ));
}

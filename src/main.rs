mod component;
mod resource;
mod player;
mod interaction_flags;



use bevy::{prelude::*, render::mesh::{
        Indices,
        VertexAttributeValues
    }};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::physics::*;
use rapier3d::{
    na::{Point3, UnitQuaternion, Vector3},
    dynamics::{RigidBodyBuilder, RigidBodySet}
};

/*use bevy_rapier3d::{
    na::Point3, physics::*, rapier::dynamics::{
        RigidBodyBuilder,
    }, rapier::geometry::*};*/

use component::BuildSceneCollider;
// use resource::{InputBindingPlugin, MouseEventState};
use resource::{InputBindingPlugin};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        //.init_resource::<MouseEventState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputBindingPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system())
        .add_system(load_collider.system())
        .add_system(player::update_player.system())
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    commands.spawn_scene(asset_server.load("character_controller_playground.gltf#Scene0"));
    /*commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });*/
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(4.0, 5.0, 4.0),
            ..Default::default()
        });
}

// TODO WT: Loading stage.
fn load_collider(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    queries: QuerySet<(
        Query<(Entity, &BuildSceneCollider)>,
        Query<&Handle<Mesh>>,
    )>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    for (tag_entity, build_collider) in queries.q0().iter() {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(build_collider.0) {
            entity_iter.for_each(|entity| {
            eprintln!("hi");
                if let Ok(mesh_handle) = queries.q1().get(entity) {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        let groups = InteractionGroups::all().with_memberships(interaction_flags::STATIC_GEOMETRY.into());
                        let collider = create_collider_for_mesh(mesh)
                            .collision_groups(groups);

                        let rigid = RigidBodyBuilder::new_static();

                        // commands.insert(entity, (collider, rigid));
                    }
                }
                // commands.despawn(tag_entity);

                *done = true;
            });
        }
    }
    *done = true;

    if *done {
        let player_entity = player::create_instance(&mut commands);
        scene_spawner.spawn_as_child(asset_server.load("Animated Characters 2/characterMedium.gltf"), player_entity);
    }
}

fn create_collider_for_mesh(mesh: &Mesh) -> ColliderBuilder {
    let verts = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

    let verts: &Vec<[f32; 3]> = match verts {
        VertexAttributeValues::Float3(vert) => Some(vert),
        _ => None
    }.unwrap();
    let verts: Vec<Point3<f32>> = verts.iter().map(|vert| {
        Point3::new(vert[0], vert[1], vert[2])
    }).collect();

    let indices: Vec<[u32; 3]> = match mesh.indices().unwrap() {
        Indices::U32(i) => Some(i),
        _ => None,
    }.unwrap().chunks(3).map(|tri| {
        [tri[0], tri[1], tri[2]]
    }).collect();

    ColliderBuilder::trimesh(verts.clone(), indices.clone())
}
// Import necessary Bevy modules for GLTF assets, 3D physics, and the main scene resource
use bevy::prelude::*;
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use bevy_rapier3d::prelude::*; // Includes physics components for collision and rigid bodies
use crate::config::MainScene; // Imports the MainScene struct and handle for the main game scene

// Function to set up colliders for each object in the GLTF scene
pub(crate) fn scene_colliders(
    mut commands: Commands, // Allows spawning of new entities and adding components
    mut main_scene: ResMut<MainScene>, // Mutable reference to the main scene resource for tracking load state
    gltf_assets: Res<Assets<Gltf>>, // Resource containing loaded GLTF assets
    gltf_mesh_assets: Res<Assets<GltfMesh>>, // Resource containing GLTF mesh assets
    gltf_node_assets: Res<Assets<GltfNode>>, // Resource for GLTF node assets
    mesh_assets: Res<Assets<Mesh>>, // Resource for standard Bevy meshes
) {
    // Check if the scene has already been loaded and processed; if so, exit the function
    if main_scene.is_loaded {
        return;
    }

    // Access the GLTF scene handle if it has been loaded
    let gltf = gltf_assets.get(&main_scene.handle);

    // If the scene is loaded, spawn the scene entities and set up colliders
    if let Some(gltf) = gltf {
        // Spawn the main scene as a bundle in the Bevy world
        let scene = gltf.scenes.first().unwrap().clone();
        commands.spawn(SceneBundle { scene, ..default() });

        // Iterate over each node in the GLTF scene to find meshes
        for node in &gltf.nodes {
            let node = gltf_node_assets.get(node).unwrap();
            if let Some(gltf_mesh) = node.mesh.clone() {
                let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh).unwrap();
                for mesh_primitive in &gltf_mesh.primitives {
                    let mesh = mesh_assets.get(&mesh_primitive.mesh).unwrap();
                    // For each mesh, create a collider component using the mesh's geometry
                    commands.spawn((
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(), // Creates a mesh collider
                        RigidBody::Fixed, // Sets the object as immovable (e.g., static scene geometry)
                        TransformBundle::from_transform(node.transform), // Apply the transform from the GLTF node to position the collider
                    ));
                }
            }
        }
        // Mark the scene as fully loaded to avoid redundant loading
        main_scene.is_loaded = true;
    }
}

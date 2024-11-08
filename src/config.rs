// Import specific types and traits from the Bevy crate
use bevy::asset::Handle; // `Handle` is used to manage assets in Bevy
use bevy::gltf::Gltf; // GLTF support in Bevy, a 3D model format
use bevy::math::Vec3; // A 3D vector type used for positional coordinates
use bevy::prelude::Resource; // The `Resource` trait allows types to be added to Bevy's world as shared resources

// Define a constant `SPAWN_POINT` representing a fixed starting position
pub(crate) const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.625, 0.0);
// `SPAWN_POINT` is set with coordinates (0.0, 1.625, 0.0), where:
// - X: 0.0 (horizontal position)
// - Y: 1.625 (vertical height, typically placing an object slightly above ground level)
// - Z: 0.0 (depth)

// Define a struct `MainScene` as a resource that stores scene data
// Explanation of Fields in `MainScene`:
// - `handle`: Manages the reference to the GLTF asset, allowing it to be loaded and accessed within Bevy's asset management system.
// - `is_loaded`: Helps track whether the GLTF asset is fully loaded, ensuring it can be used in the scene when ready.
#[derive(Resource)]
pub(crate) struct MainScene {
    pub(crate) handle: Handle<Gltf>, // A handle to a GLTF asset, which represents the main scene model
    pub(crate) is_loaded: bool, // A boolean to check if the GLTF asset has finished loading
}


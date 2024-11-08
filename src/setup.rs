// Import constants and structures from external crates and modules
use std::f32::consts::TAU; // TAU is a mathematical constant, equal to 2π (often useful in angle calculations)
use bevy::asset::AssetServer; // `AssetServer` manages loading and storing game assets in Bevy
use bevy::color::Color; // `Color` is a type to handle RGBA and linear color values
use bevy::math::Vec3; // `Vec3` is a 3D vector, typically used for positions or directional vectors in Bevy
use bevy::pbr::{light_consts, DirectionalLight, DirectionalLightBundle}; // PBR lighting and directional light settings for rendering
use bevy::prelude::{default, Camera3dBundle, Commands, PerspectiveProjection, PositionType, Projection, Query, Res, Style, TextBundle, TextStyle, Transform, TransformBundle, Val, Window}; // Core Bevy types and components for managing entities, rendering, and UI
use bevy::render::camera::Exposure; // `Exposure` allows configuration of brightness for cameras
use bevy_fps_controller::controller::{CameraConfig, FpsController, FpsControllerInput, LogicalPlayer, RenderPlayer}; // Components for FPS camera control from a separate Bevy FPS controller plugin
use bevy_rapier3d::prelude::*; // Bevy Rapier physics plugin for handling collisions and other physics behaviors
use bevy_rapier3d::geometry::{ActiveEvents, Collider, Friction, Restitution}; // Geometry-related types from Bevy Rapier for managing collision and friction properties
use crate::config::{MainScene, SPAWN_POINT}; // Local module for configurations, defining `MainScene` and the spawn point position

// `setup` function initializes various game components and entities, including the lighting, player, and main scene
pub(crate) fn setup(mut commands: Commands, mut window: Query<&mut Window>, assets: Res<AssetServer>) {
    // Set up the window title
    let mut window = window.single_mut();
    window.title = String::from("Contre Frappe");

    // Spawn a directional light for illumination
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT, // Sets brightness similar to natural daylight
            shadows_enabled: true, // Enables shadow rendering for this light source
            ..default()
        },
        transform: Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), // Sets light position and direction
        ..default()
    });

    // Set up the logical and render players
    // A "logical" player manages collision and physics
    // A "render" player controls what the player visually experiences on-screen
    let height = 3.0; // Height of the player model
    let logical_entity = commands
        .spawn((
            Collider::cylinder(height / 2.0, 0.5), // Creates a cylindrical collider for the logical player
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS, // Enables collision events for this entity
            Velocity::zero(), // Initializes with zero velocity
            RigidBody::Dynamic, // Sets the body as dynamic (affected by forces)
            Sleeping::disabled(), // Prevents the entity from sleeping when stationary
            LockedAxes::ROTATION_LOCKED, // Prevents rotation along any axis
            AdditionalMassProperties::Mass(1.0), // Sets a mass of 1.0 for the entity
            GravityScale(0.0), // Sets gravity influence to zero for the player
            Ccd { enabled: true }, // Enables continuous collision detection
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)), // Sets initial position at `SPAWN_POINT`
            LogicalPlayer, // Marks this entity as the logical player
            FpsControllerInput {
                pitch: -TAU / 12.0, // Initial pitch of the camera
                yaw: TAU * 5.0 / 8.0, // Initial yaw orientation
                ..default()
            },
            FpsController {
                air_acceleration: 80.0, // Acceleration factor when player is in the air
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: -0.5, // Sets the camera height offset for first-person perspective
        })
        .id();

    // Spawn the render player entity with a 3D camera
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0, // Field of view for the camera
                ..default()
            }),
            exposure: Exposure::SUNLIGHT, // Sets exposure level to simulate sunlight
            ..default()
        },
        RenderPlayer { logical_entity }, // Links the render player to the logical player entity
    ));

    // Insert the main scene as a resource for tracking and loading
    commands.insert_resource(MainScene {
        handle: assets.load("playground.glb"), // Loads the GLB scene asset and assigns it a handle
        is_loaded: false, // Initial state, indicates asset is not yet loaded
    });

    // Adds a TextBundle to display on-screen information, using absolute positioning
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: assets.load("fonts/FiraSans-Bold.ttf"), // Loads a font for the text display
                font_size: 24.0, // Font size
                color: Color::BLACK, // Text color
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute, // Absolute positioning on the screen
                top: Val::Px(5.0), // 5 pixels from the top
                left: Val::Px(5.0), // 5 pixels from the left
                ..default()
            }),
    );
}

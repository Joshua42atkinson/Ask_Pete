use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_bevy_app(canvas_id: String) {
    // Prevent multiple apps from running if called multiple times (simple check)
    // In a real app, we might need more robust handling or cleanup.
    // For now, we assume it's called once per session or handled by the browser reload.

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(format!("#{}", canvas_id)),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_light)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Add a sprite to prove it's working
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"), // Ensure this asset exists or use a color
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section(
            "Locomotive Cab - Pure Bevy",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn animate_light(time: Res<Time>, mut query: Query<&mut Transform, With<Sprite>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds());
    }
}

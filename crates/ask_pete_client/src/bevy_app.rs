use crate::telemetry::PHYSICS_STATE;
use ask_pete_physics::PhysicsPlugin;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use pete_core::locomotive::{CargoHold, FuelTank, LocomotiveStats, MentalState};
use pete_core::narrative_graph::NarrativeGraph;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct NarrativeGraphAsset {
    #[serde(flatten)]
    pub graph: NarrativeGraph,
}

#[derive(Resource, Default)]
pub struct CurrentStoryState {
    pub current_node_id: Option<String>,
    pub graph_handle: Handle<NarrativeGraphAsset>,
}

#[derive(Component)]
struct NarrativeText;

#[derive(Component)]
struct ChoiceButton(String);

#[derive(Component)]
struct NarrativeUiRoot;

#[wasm_bindgen]
pub fn run_bevy_app(canvas_id: String) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(format!("#{}", canvas_id)),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(JsonAssetPlugin::<NarrativeGraphAsset>::new(&[
            "narrative.json",
        ]))
        .add_plugins(PhysicsPlugin) // [NEW] Physics Engine
        .init_resource::<CurrentStoryState>()
        .add_systems(Startup, (setup, setup_physics)) // [NEW] Spawn Train
        .add_systems(Update, (narrative_driver, button_handler, sync_telemetry)) // [NEW] Sync Data
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut story_state: ResMut<CurrentStoryState>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load the narrative graph
    // Ensure this path matches where you put the file in public/assets or similar
    let graph_handle = asset_server.load("stories/demo.narrative.json");
    story_state.graph_handle = graph_handle;

    // UI Root
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .insert(NarrativeUiRoot)
        .with_children(|parent| {
            // Text Area
            parent
                .spawn(
                    TextBundle::from_section(
                        "Loading Narrative...",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::bottom(Val::Px(50.0)),
                        max_width: Val::Px(800.0),
                        ..default()
                    }),
                )
                .insert(NarrativeText);
        });
}

fn narrative_driver(
    mut commands: Commands,
    mut story_state: ResMut<CurrentStoryState>,
    narrative_assets: Res<Assets<NarrativeGraphAsset>>,
    mut text_query: Query<&mut Text, With<NarrativeText>>,
    choice_buttons: Query<Entity, With<ChoiceButton>>,
    root_query: Query<Entity, With<NarrativeUiRoot>>,
) {
    if let Some(asset) = narrative_assets.get(&story_state.graph_handle) {
        // Initialize if needed
        if story_state.current_node_id.is_none() {
            story_state.current_node_id = Some(asset.graph.start_node_id.clone());
        }

        let current_node_id = story_state.current_node_id.as_ref().unwrap();

        if let Some(node) = asset.graph.nodes.get(current_node_id) {
            // Update main text
            for mut text in &mut text_query {
                text.sections[0].value = format!("{}:\n{}", node.speaker, node.text);
            }

            // Spawn buttons if not already spawned for this node
            // We check if there are NO buttons, which implies we need to spawn them for the current node
            // (This assumes we clear buttons on transition)
            if choice_buttons.is_empty() {
                let root = root_query.single();

                if !node.choices.is_empty() {
                    commands.entity(root).with_children(|parent| {
                        for choice in &node.choices {
                            let next_id = choice.next_node_id.clone().unwrap_or_default();

                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(300.0),
                                        height: Val::Px(65.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                                    ..default()
                                })
                                .insert(ChoiceButton(next_id))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        &choice.text,
                                        TextStyle {
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ));
                                });
                        }
                    });
                } else {
                    // End of conversation?
                    // Maybe spawn a restart button?
                }
            }
        }
    }
}

fn button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ChoiceButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut story_state: ResMut<CurrentStoryState>,
    mut commands: Commands,
    all_buttons: Query<Entity, With<ChoiceButton>>,
) {
    for (interaction, mut color, choice) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                // Transition to next node
                if !choice.0.is_empty() {
                    story_state.current_node_id = Some(choice.0.clone());

                    // Despawn ALL buttons to trigger regeneration
                    for entity in &all_buttons {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

// --- PHYSICS INTEGRATION ---

fn setup_physics(mut commands: Commands) {
    // Spawn the Player's Locomotive
    commands.spawn((
        LocomotiveStats {
            archetype: "Hero".to_string(),
            traction: 10.0,
            velocity: 0.0, // Initial velocity
            efficiency: 1.0,
            analysis: 5.0,
            signaling: 5.0,
            coupling: 5.0,
        },
        FuelTank {
            coal: 100.0,
            steam: 0.0,
            max_coal: 100.0,
            max_steam: 100.0,
        },
        CargoHold {
            items: vec![],
            capacity: 7.0,
        },
        MentalState {
            scale_level: 0.0,
            is_stalled: false,
            is_derailed: false,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        ask_pete_physics::combustion::TrackGradient::default(), // Flat track
    ));
}

fn sync_telemetry(query: Query<(&FuelTank, &LocomotiveStats, &Transform)>) {
    if let Ok((fuel, stats, transform)) = query.get_single() {
        if let Ok(mut state) = PHYSICS_STATE.write() {
            state.coal = fuel.coal;
            state.steam = fuel.steam;
            state.velocity = stats.velocity;
            state.latitude = transform.translation.y as f64; // Using Y as Latitude for now
            state.longitude = transform.translation.x as f64; // Using X as Longitude
        }
    }
}

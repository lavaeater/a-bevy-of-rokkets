use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::hashbrown::{HashMap, HashSet};
use bevy_xpbd_2d::parry::na::SimdRealField;

const X_EXTENT: f32 = 600.;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(FactStore::new())
        .add_event::<FactUpdated>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_layout)
        .add_systems(Update, button_system)
        .add_systems(Update, fact_update_event_broadcaster)
        .run();
}

#[derive(Event)]
pub struct FactUpdated {
    key: String,
    fact: Fact
}

fn fact_update_event_broadcaster(
    mut event_writer: EventWriter<FactUpdated>,
    mut storage: ResMut<FactStore>,
) {
    for (key) in storage.changed_int_facts.drain() {
        event_writer.send(FactUpdated {
            key: key.clone(),
            fact: Fact::Int(*storage.int_facts.get(&key).unwrap())
        });
    }

    for (key) in storage.changed_string_facts.drain() {
        event_writer.send(FactUpdated {
            key: key.clone(),
            fact: Fact::String(storage.string_facts.get(&key).unwrap().clone())
        });
    }

    for (key) in storage.changed_bool_facts.drain() {
        event_writer.send(FactUpdated {
            key: key.clone(),
            fact: Fact::Bool(*storage.bool_facts.get(&key).unwrap())
        });
    }

    for (key) in storage.changed_list_facts.drain() {
        event_writer.send(FactUpdated {
            key: key.clone(),
            fact: Fact::StringList(storage.list_facts.get(&key).unwrap().clone())
        });
    }
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety it's parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                //   - The first column will size to the size of it's contents
                //   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                //  - The first row will size to the size of it's contents
                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                //  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Make this node span two grid columns so that it takes up the entire top tow
                        grid_column: GridPlacement::span(2),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        // Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        // As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(1.0),
                        // Use grid layout for this node
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    item_rect(builder, Color::ORANGE, false, font.clone_weak());
                    item_rect(builder, Color::BISQUE, false, font.clone_weak());
                    item_rect(builder, Color::BLUE, false, font.clone_weak());
                    item_rect(builder, Color::CRIMSON, false, font.clone_weak());

                    item_rect(builder, Color::CYAN, false, font.clone_weak());
                    item_rect(builder, Color::ORANGE_RED, false, font.clone_weak());
                    item_rect(builder, Color::DARK_GREEN, false, font.clone_weak());
                    item_rect(builder, Color::FUCHSIA, false, font.clone_weak());

                    item_rect(builder, Color::TEAL, false, font.clone_weak());
                    item_rect(builder, Color::ALICE_BLUE, false, font.clone_weak());
                    item_rect(builder, Color::CRIMSON, false, font.clone_weak());
                    item_rect(builder, Color::ANTIQUE_WHITE, false, font.clone_weak());

                    item_rect(builder, Color::YELLOW, false, font.clone_weak());
                    item_rect(builder, Color::PINK, false, font.clone_weak());
                    item_rect(builder, Color::YELLOW_GREEN, false, font.clone_weak());
                    item_rect(builder, Color::SALMON, true, font.clone_weak());
                });

            // Right side bar (auto placed in row 2, column 2)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Align content towards the start (top) in the vertical axis
                        align_items: AlignItems::Start,
                        // Align content towards the center in the horizontal axis
                        justify_items: JustifyItems::Center,
                        // Add 10px padding
                        padding: UiRect::all(Val::Px(10.)),
                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                        // Add a 10px gap between rows
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Sidebar",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        "A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                    ));
                    builder.spawn(NodeBundle::default());
                });

            // Footer / status bar
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            });

            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
            builder.spawn(NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Px(100.),
                        bottom: Val::Auto,
                        left: Val::Auto,
                        right: Val::Auto,
                    },
                    width: Val::Percent(60.),
                    height: Val::Px(300.),
                    max_width: Val::Px(600.),
                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 255.0,
                    green: 255.0,
                    blue: 255.0,
                    alpha: 0.8,
                }),
                ..default()
            });
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Color, with_button: bool, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            if with_button {
                builder.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font,
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }

            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
    ));
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut storage: ResMut<FactStore>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                storage.add_to_int("button_pressed".to_string(), 1);
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = storage.get_int("button_pressed").unwrap_or(&0).to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Press to add".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub enum Fact {
    Int(i32),
    String(String),
    Bool(bool),
    StringList(HashSet<String>),
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        // Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        // Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        // Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        // Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        // Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + i as f32 / (num_shapes) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}

#[derive(Resource)]
struct FactStore {
    int_facts: HashMap<String, i32>,
    string_facts: HashMap<String, String>,
    bool_facts: HashMap<String, bool>,
    list_facts: HashMap<String, HashSet<String>>,
    changed_int_facts: HashSet<String>,
    changed_string_facts: HashSet<String>,
    changed_bool_facts: HashSet<String>,
    changed_list_facts: HashSet<String>,
}

impl FactStore {
    // Create a new instance of FactStore
    fn new() -> Self {
        FactStore {
            int_facts: HashMap::new(),
            string_facts: HashMap::new(),
            bool_facts: HashMap::new(),
            list_facts: HashMap::new(),
            changed_int_facts: HashSet::new(),
            changed_string_facts: HashSet::new(),
            changed_bool_facts: HashSet::new(),
            changed_list_facts: HashSet::new(),
        }
    }

    // Store an integer fact
    fn store_int(&mut self, key: String, value: i32) {
        let current_value = self.get_int(&key);
        if current_value.unwrap_or(&0) != &value {
            self.int_facts.insert(key.clone(), value);
            self.changed_int_facts.insert(key.clone());
        }
    }

    fn add_to_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    fn subtract_from_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current - value);
    }

    // Store a string fact
    fn store_string(&mut self, key: String, value: String) {
        let current_value = self.get_string(&key);
        if current_value.unwrap_or(&"".to_string()) != &value {
            self.changed_string_facts.insert(key.clone());
            self.changed_string_facts.insert(key.clone());
        }
    }

    // Store a boolean fact
    fn store_bool(&mut self, key: String, value: bool) {
        let current_value = self.get_bool(&key);
        if current_value.unwrap_or(&false) != &value {
            self.bool_facts.insert(key.clone(), value);
            self.changed_bool_facts.insert(key.clone());
        }
    }

    // Store a list of strings fact
    fn add_to_string_list(&mut self, key: String, value: String) {
        if let Some(list) = self.list_facts.get_mut(&key) {
            if list.insert(value) {
                self.changed_list_facts.insert(key.clone());
            }
        } else {
            let mut new_list = HashSet::new();
            new_list.insert(value);
            self.list_facts.insert(key.clone(), new_list);
            self.changed_list_facts.insert(key.clone());
        }
    }

    fn remove_from_string_list(&mut self, key: String, value: String) {
        if let Some(list) = self.list_facts.get_mut(&key) {
            if list.remove(&value) {
                self.changed_list_facts.insert(key.clone());
            }
        }
    }

    // Retrieve an integer fact
    fn get_int(&self, key: &str) -> Option<&i32> {
        self.int_facts.get(key)
    }

    // Retrieve a string fact
    fn get_string(&self, key: &str) -> Option<&String> {
        self.string_facts.get(key)
    }

    // Retrieve a boolean fact
    fn get_bool(&self, key: &str) -> Option<&bool> {
        self.bool_facts.get(key)
    }

    // Retrieve a list of strings fact
    fn get_list(&self, key: &str) -> Option<&HashSet<String>> {
        self.list_facts.get(key)
    }
}

// Define the FactStore structure (as provided earlier)

// Define a rule structure
struct Rule {
    conditions: Vec<Condition>,
}

// Define a condition enum to represent different types of conditions
enum Condition {
    StringEquals(String, String),
    IntEquals(String, i32),
    IntLargerThan(String, i32),
    IntLessThan(String, i32),
    BoolEquals(String, bool),
    ListContains(String, String),
    Invert(Arc<Condition>),
}

impl Rule {
    // Create a new instance of Rule
    fn new() -> Self {
        Rule {
            conditions: Vec::new(),
        }
    }

    // Add a condition to the rule
    fn add_condition(&mut self, condition: Condition) {
        self.conditions.push(condition);
    }

    // Evaluate the rule based on the FactStore
    fn evaluate(&self, fact_store: &FactStore) -> bool {
        self.conditions.iter().all(|condition| self.evaluate_condition(condition, fact_store))
    }

    fn evaluate_condition(&self, condition: &Condition, fact_store: &FactStore) -> bool {
        match condition {
            Condition::StringEquals(key, value) => {
                fact_store.get_string(key).map_or(false, |fact| fact == value)
            }
            Condition::BoolEquals(key, value) => {
                fact_store.get_bool(key).map_or(false, |fact| fact == value)
            }
            Condition::ListContains(key, value) => {
                fact_store
                    .get_list(key)
                    .map_or(false, |fact| fact.contains(value))
            }
            Condition::IntEquals(key, value) => {
                fact_store.get_int(key).map_or(false, |fact| fact == value)
            }
            Condition::IntLargerThan(key, value) => {
                fact_store.get_int(key).map_or(false, |fact| fact > value)
            }
            Condition::IntLessThan(key, value) => {
                fact_store.get_int(key).map_or(false, |fact| fact < value)
            }
            Condition::Invert(inner_condition) => {
                !self.evaluate_condition(inner_condition, fact_store)
            }
        }
    }
}
//
// fn main() {
//     // Create a new FactStore and populate it with some facts
//     let mut fact_store = FactStore::new();
//     fact_store.store_string_fact("name".to_string(), "Alice".to_string());
//     fact_store.store_bool_fact("is_student".to_string(), true);
//     fact_store.store_string_list_fact(
//         "hobbies".to_string(),
//         vec!["reading".to_string(), "coding".to_string(), "gardening".to_string()],
//     );
//
//     // Create a rule
//     let mut rule = Rule::new();
//     rule.add_condition(Condition::StringEquals("name".to_string(), "Alice".to_string()));
//     rule.add_condition(Condition::BoolEquals("is_student".to_string(), true));
//     rule.add_condition(Condition::ListContains("hobbies".to_string(), "reading".to_string()));
//
//     // Evaluate the rule
//     let rule_result = rule.evaluate(&fact_store);
//     println!("Rule evaluation result: {}", rule_result);
// }


//
// fn main() {
//     // Create a new FactStore
//     let mut fact_store = FactStore::new();
//
//     // Store some sample facts
//     fact_store.store_int_fact("age".to_string(), 30);
//     fact_store.store_string_fact("name".to_string(), "Alice".to_string());
//     fact_store.store_bool_fact("is_student".to_string(), true);
//     fact_store.store_string_list_fact(
//         "hobbies".to_string(),
//         vec!["reading".to_string(), "coding".to_string(), "gardening".to_string()],
//     );
//
//     // Retrieve and print some facts
//     println!("Age: {:?}", fact_store.get_int_fact("age"));
//     println!("Name: {:?}", fact_store.get_string_fact("name"));
//     println!("Is Student: {:?}", fact_store.get_bool_fact("is_student"));
//     println!(
//         "Hobbies: {:?}",
//         fact_store.get_string_list_fact("hobbies")
//     );
// }



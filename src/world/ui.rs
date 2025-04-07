use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Terrain Legend",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Terrain layers
            parent.spawn(TextBundle::from_section(
                "Terrain Layers:",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Surface layer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.6, 0.8, 0.4)),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Surface (Grass)",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Dirt layer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.6, 0.4, 0.2)),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Dirt",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Stone layer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Stone",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Deep layer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Deep",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Bedrock layer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Bedrock",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Features section
            parent.spawn(TextBundle::from_section(
                "Features:",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Caves
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Caves",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Water
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.8)),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Water",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Resources
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Minerals",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

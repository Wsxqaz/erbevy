use crate::{Game, GameState};
use bevy::prelude::*;

#[derive(Component)]
struct OnMenuScreen;

#[derive(Resource)]
struct MenuUpdateTimer(Timer);

#[derive(Resource)]
struct MenuInputTimer(Timer);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(
                Update,
                (menu_input, menu_update)
                    .chain()
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), menu_cleanup);
    }
}

fn menu_setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Welcome to the menu!",
                            TextStyle {
                                font_size: 100.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent.spawn((
                        TextBundle::from_section(
                            "Start",
                            TextStyle {
                                font_size: 50.0,
                                color: Color::BLUE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                        OnMenuScreen,
                    ));

                    parent.spawn((
                        TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                font_size: 50.0,
                                color: Color::BLUE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                        OnMenuScreen,
                    ));
                });
        });

    commands.insert_resource(MenuInputTimer(Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating)));
    commands.insert_resource(MenuUpdateTimer(Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating)));
}

fn menu_input(
    mut timer: ResMut<MenuInputTimer>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if input.just_pressed(KeyCode::ArrowUp) {
            info!("handling KeyCode::ArrowUp");
            game.menu.hover = (game.menu.hover.wrapping_sub(1)) % 2;
        }
        if input.just_pressed(KeyCode::ArrowDown) {
            info!("handling KeyCode::ArrowDown");
            game.menu.hover = (game.menu.hover + 1) % 2;
        }
        if input.just_pressed(KeyCode::Enter) {
            info!("handling KeyCode::Enter");
            match game.menu.hover {
                0 => game_state.set(GameState::Playing),
                1 => game_state.set(GameState::Exit),
                _ => {}
            }
        }
    }
}

fn menu_update(
    mut query: Query<&mut Text, With<OnMenuScreen>>,
    game: Res<Game>,
    mut timer: ResMut<MenuUpdateTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let hover = game.menu.hover;
        for mut text in query.iter_mut() {
            for value in text.sections.iter_mut() {
                if value.value.contains("Start") {
                    if hover == 0 {
                        value.value = "> Start".to_string();
                    } else {
                        value.value = value.value.replace("> ", "");
                    }
                } else if value.value.contains("Exit") {
                    if hover == 1 {
                        value.value = "> Exit".to_string();
                    } else {
                        value.value = value.value.replace("> ", "");
                    }
                }
            }
        }
    }
}

fn menu_cleanup(query: Query<Entity, With<OnMenuScreen>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

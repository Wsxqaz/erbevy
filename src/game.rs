use crate::{Game, GameState};
use bevy::prelude::*;

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource)]
struct GameTimer(Timer);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), game_cleanup);
    }
}

fn game_setup(mut commands: Commands, game: Res<Game>) {
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
            OnGameScreen,
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
                            "Willkommen zum Spiel!",
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

                    parent.spawn(
                        TextBundle::from_section(
                            format!("player: {:?}", game.player),
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
                    );
                });
        });

    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn game(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        game_state.set(GameState::Menu);
    }
}

fn game_cleanup() {}

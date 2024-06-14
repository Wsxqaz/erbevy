use crate::{Game, GameState};
use bevy::prelude::*;
use bevy::sprite::{ Mesh2dHandle, MaterialMesh2dBundle };

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource)]
struct GameGlobalTimer(Timer);

#[derive(Resource)]
struct GameMoveTimer(Timer);

#[derive(Component)]
struct Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), game_cleanup);
    }
}

fn game_setup(mut commands: Commands, game: Res<Game>, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let circle = Mesh2dHandle(meshes.add(Circle {
        radius: 25.0,
        ..Default::default()
    }));
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
                .spawn((
                    MaterialMesh2dBundle {
                        mesh: circle,
                        material: materials.add(Color::WHITE),
                        ..default()
                    }, OnGameScreen, Player
                ));

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

    commands.insert_resource(GameGlobalTimer(Timer::from_seconds(30.0, TimerMode::Once)));
    commands.insert_resource(GameMoveTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn game(
    time: Res<Time>,
    mut global_timer: ResMut<GameGlobalTimer>,
    mut move_timer: ResMut<GameMoveTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Transform, (With<OnGameScreen>, With<Player>)>,
) {
    if global_timer.0.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }

    if move_timer.0.tick(time.delta()).just_finished() {
        println!("Move player");
        for mut transform in query.iter_mut() {
            println!("transform item: {:?}", transform);
            transform.translation.x -= 10.0;
            transform.translation.y -= 10.0;
            transform.scale += 0.1;
        }
    }
}

fn game_cleanup(mut commands: Commands, query: Query<Entity, With<OnGameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

use crate::{Game, GameState};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource)]
struct GameGlobalTimer(Timer);

#[derive(Resource)]
struct GameMoveTimer(Timer);

#[derive(Resource)]
struct GamePlayerTrackerTimer(Timer);

#[derive(Resource)]
struct GamePlayerInputTimer(Timer);

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct PlayerTracker;

#[derive(Resource)]
struct WallSpawnTimer(Timer);

#[derive(Resource)]
struct WallMoveTimer(Timer);

#[derive(Component)]
struct Wall {
    side: u32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(Update, (game, game_player_tracker, game_handle_input, game_wallspawner, game_wallmover).chain().run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), game_cleanup);
    }
}

fn game_wallspawner(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut timer: ResMut<WallSpawnTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let wallspawn_side = game.walls.last;
    game.walls.last = (game.walls.last + 1) % 4;

    let edge = 200.0;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let scale = 1.0;
    if game.walls.last == 0 {
        x = edge;
    } else if game.walls.last == 1 {
        y = edge;
    } else if game.walls.last == 2 {
        x = -1.0 * edge;
    } else if game.walls.last == 3 {
        y = -1.0 * edge;
    }

    let transform = Transform::from_xyz(x, y, z);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, z),
                scale: Vec3::new(60.0, 60.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            ..default()
        },
    Wall { side: wallspawn_side }));
}

fn game_wallmover(
    mut commands: Commands,
    mut move_timer: ResMut<WallMoveTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Wall, Entity), With<Wall>>,
) {
    if !move_timer.0.tick(time.delta()).finished() {
        return;
    }

    for (mut transform, wall, entity)  in query.iter_mut() {
        if transform.translation.x == 0.0 && transform.translation.y == 0.0 {
            commands.entity(entity).despawn_recursive();
        }

        if wall.side == 0 {
            transform.translation.y -= 10.0;
        } else if wall.side == 1 {
            transform.translation.x += 10.0;
        } else if wall.side == 2 {
            transform.translation.y += 10.0;
        } else if wall.side == 3 {
            transform.translation.x -= 10.0;
        }
    }
}

fn game_setup(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

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
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.1).into(),
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

                    parent.spawn((
                        TextBundle::from_section(
                            format!("player: {:?}", game.player),
                            TextStyle {
                                font_size: 50.0,
                                color: Color::BLUE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(100.0)),
                            ..default()
                        }), PlayerTracker, OnGameScreen
                    ));
                });
        });
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(60.0, 60.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            ..default()
        },
        OnGameScreen,
        PlayerSprite,
    ));

    commands.insert_resource(GameGlobalTimer(Timer::from_seconds(30.0, TimerMode::Once)));
    commands.insert_resource(GameMoveTimer(Timer::from_seconds(
        1.0 / 60.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GamePlayerTrackerTimer(Timer::from_seconds(
        1.0 / 60.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GamePlayerInputTimer(Timer::from_seconds(
        1.0 / 60.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(WallSpawnTimer(Timer::from_seconds(
        4.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(WallMoveTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
}

fn game_player_tracker(
    time: Res<Time>,
    mut game_player_tracker_timer: ResMut<GamePlayerTrackerTimer>,
    mut query: Query<&mut Text, (With<OnGameScreen>, With<PlayerTracker>)>,
    game: Res<Game>,
) {
    if game_player_tracker_timer.0.tick(time.delta()).just_finished() {
        for mut text in query.iter_mut() {

            text.sections[0].value = format!("player: {:?}", game.player);
        }
    }
}

fn game_handle_input(
    time: Res<Time>,
    mut input_timer: ResMut<GamePlayerInputTimer>,
    mut input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if input_timer.0.tick(time.delta()).just_finished() {
        info!("Checking for player input...");
        if input.just_pressed(KeyCode::ArrowUp) {
            game.player.pos = 0;
        } else if input.just_pressed(KeyCode::ArrowRight) {
            game.player.pos = 1;
        } else if input.just_pressed(KeyCode::ArrowDown) {
            game.player.pos = 2;
        } else if input.just_pressed(KeyCode::ArrowLeft) {
            game.player.pos = 3;
        }
    }
}

fn game(
    time: Res<Time>,
    mut global_timer: ResMut<GameGlobalTimer>,
    mut move_timer: ResMut<GameMoveTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game: ResMut<Game>,
    mut query: Query<&mut Transform, (With<OnGameScreen>, With<PlayerSprite>)>,
) {
    if global_timer.0.tick(time.delta()).finished() {
        info!("Game over!");
        game_state.set(GameState::Menu);
    }

    if move_timer.0.tick(time.delta()).just_finished() {
        info!("Moving player...");
        for mut transform in query.iter_mut() {
            match game.player.pos {
                0 => {
                    transform.translation.x = 0.0;
                    transform.translation.y = 100.0;
                },
                1 => {
                    transform.translation.x = 100.0;
                    transform.translation.y = 0.0;
                },
                2 => {
                    transform.translation.x = 0.0;
                    transform.translation.y = -100.0;
                },
                3 => {
                    transform.translation.x = -100.0;
                    transform.translation.y = 0.0;
                },
                _ => (),
            }
            // println!("transform item: {:?}", transform);
            // transform.translation.x += -10.0;
            // transform.translation.y += -10.0;
            // println!("transform item: {:?}", transform);
            // game.player.x = transform.translation.x;
            // game.player.y = transform.translation.y;
            // transform.scale += 0.1;
        }
    }
}

fn game_cleanup(mut commands: Commands, query: Query<Entity, With<OnGameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

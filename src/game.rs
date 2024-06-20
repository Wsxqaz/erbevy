use crate::{Game, GameState};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const WALL_SIDES: u32 = 6u32;
const PLAYER_RING_RADIUS: f32 = 500.0;
const PLAYER_MOVE_SPEED: f32 = 5.0;
const INITIAL_RING_RADIUS: f32 = 100.0;
const WALL_SPIN_SPEED: f32 = 1.0;
const WALL_SHRINK_SPEED: f32 = 0.01;
const CENTER_HEX_RADIUS: f32 = 50.0;
const CENTER_HEX_WIDTH: f32 = 2000.0;
const CENTER_HEX_HEIGHT: f32 = 10.0;

// /-\
// \-/

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

#[derive(Component)]
struct BackgroundSlice {
    theta: f32,
}

#[derive(Resource)]
struct BackgroundMoveTimer(Timer);

#[derive(Resource)]
struct WallSpawnTimer(Timer);

#[derive(Resource)]
struct WallMoveTimer(Timer);

#[derive(Component)]
struct Wall {
    theta: f32,
    ring_radius: f32,
}

#[derive(Component)]
struct CenterHex;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(
                Update,
                (
                    game,
                    game_player_tracker,
                    game_handle_input,
                    game_wallspawner,
                    game_wallmover,
                    game_backgroundmover,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), game_cleanup);
    }
}

fn wallside_to_translation(side: u32) -> Vec3 {
    let edge = 500.0;
    let mut pos_x = 0.0;
    let mut pos_y = 0.0;
    let mut pos_z = 0.0;

    if side == 0 {
        pos_y = edge;
    } else if side == 1 {
        pos_x = edge;
    } else if side == 2 {
        pos_y = -1.0 * edge;
    } else if side == 3 {
        pos_x = -1.0 * edge;
    }

    Vec3::new(pos_x, pos_y, pos_z)
}

fn wallside_to_scale(side: u32) -> Vec3 {
    let size = 300.0;
    let mut scale_x = 20.0;
    let mut scale_y = 20.0;

    if side == 0 {
        scale_x = size;
    } else if side == 1 {
        scale_y = size;
    } else if side == 2 {
        scale_x = size;
    } else if side == 3 {
        scale_y = size;
    }

    Vec3::new(scale_x, scale_y, 1.0)
}

fn game_wallspawner(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut timer: ResMut<WallSpawnTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    game.walls.last = (game.walls.last + 1) % 4;

    let wallside = game.walls.last;
    let translation = wallside_to_translation(wallside);
    let scale = wallside_to_scale(wallside);
}

fn game_wallmover(
    mut commands: Commands,
    mut move_timer: ResMut<WallMoveTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Wall, Entity), With<Wall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !move_timer.0.tick(time.delta()).finished() {
        return;
    }

    for (mut transform, mut wall, entity) in query.iter_mut() {
        if transform.translation.x == 0.0 && transform.translation.y == 0.0 {
            commands.entity(entity).despawn_recursive();
        }

        wall.theta = (wall.theta + WALL_SPIN_SPEED) % 360.0;
        wall.ring_radius -= WALL_SHRINK_SPEED;
        let x = wall.theta.to_radians().cos() * wall.ring_radius;
        let y = wall.theta.to_radians().sin() * wall.ring_radius;
        transform.translation.x = x;
        transform.translation.y = y;
        transform.scale.x -= 0.01;
        transform.scale.y -= 0.01;
        transform.rotation = Quat::from_rotation_z(wall.theta.to_radians());
    }
}

fn game_backgroundmover(
    mut commands: Commands,
    mut move_timer: ResMut<BackgroundMoveTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BackgroundSlice, &Mesh2dHandle)>,
) {
    if move_timer.0.tick(time.delta()).finished() {
        info!("moving background slices");
        for (mut transform, mut slice, mut mesh) in query.iter_mut() {
            slice.theta = (slice.theta + 0.5) % 360.0;
            transform.rotation = Quat::from_rotation_z(slice.theta.to_radians());
        }
    }
}

fn game_setup(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
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
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.1).into(),
                        ..default()
                    },
                    OnGameScreen,
                ))
                .with_children(|parent| {
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
                        }),
                        PlayerTracker,
                        OnGameScreen,
                    ));
                });
        });

    for i in 0..WALL_SIDES {
        info!("spawning center hex side {}", i);
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let scale = Vec3::new(CENTER_HEX_WIDTH, CENTER_HEX_HEIGHT, 1.0);
        let rotation = Quat::from_rotation_z((i as f32 * (360.0 / WALL_SIDES as f32)).to_radians());
        info!("rotation: {:?}", rotation);

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: translation,
                    scale: scale,
                    rotation: rotation,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
            CenterHex,
        ));
    }

    for i in 0..WALL_SIDES {
        info!("spawning background slice {}", i);
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let scale = Vec3::new(1.0, 1.0, 1.0);
        let mut color = Color::WHITE;
        if i % 2 == 0 {
            color = Color::GREEN;
        } else {
            color = Color::BLUE;
        }

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(Triangle2d {
                        vertices: [
                            Vec2::new(
                                (i as f32 * 60.0).to_radians().cos() * CENTER_HEX_WIDTH,
                                (i as f32 * 60.0).to_radians().sin() * CENTER_HEX_WIDTH,
                            ),
                            Vec2::new(
                                ((i as f32 + 1.0) * 60.0).to_radians().cos() * CENTER_HEX_WIDTH,
                                ((i as f32 + 1.0) * 60.0).to_radians().sin() * CENTER_HEX_WIDTH,
                            ),
                            Vec2::new(0.0, 0.0),
                        ],
                    }))
                    .into(),
                material: materials.add(color).into(),
                transform: Transform {
                    translation: translation,
                    scale: scale,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
            BackgroundSlice { theta: 0.0 },
        ));
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                scale: Vec3::new(60.0, 60.0, 2.0),
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

    commands.insert_resource(GameGlobalTimer(Timer::from_seconds(
        6000.0,
        TimerMode::Once,
    )));
    commands.insert_resource(GameMoveTimer(Timer::from_seconds(
        1.0 / 60.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GamePlayerTrackerTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GamePlayerInputTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(WallSpawnTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    commands.insert_resource(WallMoveTimer(Timer::from_seconds(
        1.0 / 30.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(BackgroundMoveTimer(Timer::from_seconds(
        1.0 / 30.0,
        TimerMode::Repeating,
    )));
}

fn game_player_tracker(
    time: Res<Time>,
    mut game_player_tracker_timer: ResMut<GamePlayerTrackerTimer>,
    mut query: Query<&mut Text, (With<OnGameScreen>, With<PlayerTracker>)>,
    game: Res<Game>,
) {
    if game_player_tracker_timer
        .0
        .tick(time.delta())
        .just_finished()
    {
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
        if input.pressed(KeyCode::ArrowRight) {
            game.player.theta = (game.player.theta + PLAYER_MOVE_SPEED) % 360.0;
        } else if input.pressed(KeyCode::ArrowLeft) {
            game.player.theta = (game.player.theta - PLAYER_MOVE_SPEED) % 360.0;
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
        let x = game.player.theta.to_radians().cos() * PLAYER_RING_RADIUS;
        let y = game.player.theta.to_radians().sin() * PLAYER_RING_RADIUS;
        game.player.x = x;
        game.player.y = y;
        for mut transform in query.iter_mut() {
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

fn game_cleanup(mut commands: Commands, query: Query<Entity, With<OnGameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

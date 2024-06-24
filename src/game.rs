use crate::{Game, GameState};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const WALL_SIDES: u32 = 6u32;
const PLAYER_MOVE_SPEED: f32 = 5.0;
const INITIAL_RING_RADIUS: f32 = 100.0;
const WALL_SPIN_SPEED: f32 = 1.0;
const WALL_SHRINK_SPEED: f32 = 0.01;
const WALL_RING_RADIUS: f32 = 1000.0;
const WALL_HEIGHT: f32 = 10.0;
const CENTER_HEX_RADIUS: f32 = 100.0;
const CENTER_HEX_HEIGHT: f32 = 10.0;
const PLAYER_RING_RADIUS: f32 = CENTER_HEX_RADIUS + 60.0;
const BORDER_WIDTH: f32 = 3000.0;
const BORDER_HEIGHT: f32 = 10.0;
const BACKGROUD_MOVE_SPEED: f32 = 0.5;

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
    offset_theta: f32,
    index: u32,
}

#[derive(Resource)]
struct BorderMoveTimer(Timer);

#[derive(Resource)]
struct BackgroundMoveTimer(Timer);

#[derive(Resource)]
struct WallSpawnTimer(Timer);

#[derive(Resource)]
struct WallMoveTimer(Timer);

#[derive(Resource)]
struct CenterHexMoveTimer(Timer);

#[derive(Resource)]
struct GameRotateTimer(Timer);

#[derive(Component)]
struct Wall {
    index: u32,
    ring_radius: f32,
}

#[derive(Component)]
struct CenterHex {
    offset_theta: f32,
    index: u32,
}

#[derive(Component)]
struct Borders {
    offset_theta: f32,
    index: u32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                game_setup,
                spawn_background_slices,
                spawn_background_borders,
                spawn_center_hex,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                game,
                game_player_tracker,
                game_handle_input,
                game_wallspawner,
                game_wallmover,
                game_background_mover,
                game_border_mover,
                game_center_hex_mover,
                game_theta_mover,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
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
    asset_server: Res<AssetServer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    println!("spawning wall");
    let wallside = game.walls.last;
    game.walls.last += 1;
    let theta = game.theta;
    let theta = (theta + wallside as f32 * 60.0) % 360.0;
    let theta = (theta + 30.0) % 360.0;
    let x = theta.to_radians().cos() * WALL_RING_RADIUS;
    let y = theta.to_radians().sin() * WALL_RING_RADIUS;
    let translation = Vec3::new(x, y, 10.0);
    let x1 = (1.0 * WALL_RING_RADIUS);
    let x2 = (60.0_f32.to_radians().cos() * WALL_RING_RADIUS);
    let y1 = 0.0;
    let y2 = (60.0_f32.to_radians().sin() * WALL_RING_RADIUS);
    let scale_x = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() + (WALL_RING_RADIUS / 6.25);
    let scale = Vec3::new(scale_x, WALL_HEIGHT, 1.0);
    let rotation = Quat::from_rotation_z(theta.to_radians() + 90.0_f32.to_radians());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: translation,
                scale: scale,
                rotation: rotation,
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        },
        OnGameScreen,
        Wall {
            index: wallside,
            ring_radius: WALL_RING_RADIUS,
        },
    ));
}

fn game_wallmover(
    mut commands: Commands,
    mut move_timer: ResMut<WallMoveTimer>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Wall, Entity), With<Wall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !move_timer.0.tick(time.delta()).finished() {
        return;
    }

    for (mut transform, mut wall, entity) in query.iter_mut() {
        println!("moving wall {}", wall.index);
        let theta = game.theta;
        let theta = (theta + wall.index as f32 * 60.0) % 360.0;
        let theta = (theta + 30.0) % 360.0;
        transform.rotation = Quat::from_rotation_z(theta.to_radians() + 90.0_f32.to_radians());
        wall.ring_radius -= 1.0;
        let x = theta.to_radians().cos() * wall.ring_radius;
        let y = theta.to_radians().sin() * wall.ring_radius;
        let x1 = (1.0 * wall.ring_radius);
        let x2 = (60.0_f32.to_radians().cos() * wall.ring_radius);
        let y1 = 0.0;
        let y2 = (60.0_f32.to_radians().sin() * wall.ring_radius);
        let scale_x = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        transform.translation.x = x;
        transform.translation.y = y;
        info!("scale_x: {}", scale_x);
        transform.scale.x = scale_x + (wall.ring_radius / 6.25);

        if wall.ring_radius < CENTER_HEX_RADIUS {
            commands.entity(entity).despawn_recursive();
        }

    }
}

fn game_theta_mover(
    mut commands: Commands,
    mut move_timer: ResMut<GameRotateTimer>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    if move_timer.0.tick(time.delta()).finished() {
        game.theta = (game.theta + WALL_SPIN_SPEED) % 360.0;
    }
}

fn game_background_mover(
    mut commands: Commands,
    mut move_timer: ResMut<BackgroundMoveTimer>,
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<(&mut Transform, &mut BackgroundSlice, &Mesh2dHandle)>,
) {
    if move_timer.0.tick(time.delta()).finished() {
        for (mut transform, mut slice, mut mesh) in query.iter_mut() {
            transform.rotation = Quat::from_rotation_z(game.theta.to_radians());
        }
    }
}

fn game_border_mover(
    mut commands: Commands,
    mut move_timer: ResMut<BorderMoveTimer>,
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<(&mut Transform, &mut Borders)>,
) {
    if move_timer.0.tick(time.delta()).finished() {
        for (mut transform, mut border) in query.iter_mut() {
            transform.rotation = Quat::from_rotation_z(game.theta.to_radians() + border.index as f32 * 60.0_f32.to_radians());
        }
    }
}

fn spawn_background_slices(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for i in 0..WALL_SIDES {
        let translation = Vec3::new(0.0, 0.0, 1.0);
        let scale = Vec3::new(1.0, 1.0, 1.0);
        let mut color = Color::WHITE;
        if i % 2 == 0 {
            color = Color::OLIVE;
        } else {
            color = Color::ORANGE_RED;
        }

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(Triangle2d {
                        vertices: [
                            Vec2::new(
                                (i as f32 * 60.0).to_radians().cos() * BORDER_WIDTH,
                                (i as f32 * 60.0).to_radians().sin() * BORDER_WIDTH,
                            ),
                            Vec2::new(
                                ((i as f32 + 1.0) * 60.0).to_radians().cos() * BORDER_WIDTH,
                                ((i as f32 + 1.0) * 60.0).to_radians().sin() * BORDER_WIDTH,
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
            BackgroundSlice { offset_theta: 0.0, index: i },
        ));
    }
}

fn spawn_background_borders(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for i in 0..WALL_SIDES {
        let scale = Vec3::new(BORDER_WIDTH, BORDER_HEIGHT, 2.0);
        let theta = (i as f32 * (360.0 / WALL_SIDES as f32));
        let translation = Vec3::new(0.0, 0.0, 2.0);
        let rotation = Quat::from_rotation_z(theta.to_radians());

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
            Borders { offset_theta: theta, index: i },
        ));
    }
}

fn game_center_hex_mover(
    mut commands: Commands,
    mut move_timer: ResMut<CenterHexMoveTimer>,
    game: Res<Game>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CenterHex)>,
) {
    if move_timer.0.tick(time.delta()).finished() {
        for (mut transform, mut center_hex) in query.iter_mut() {
            let theta = game.theta;
            let theta = (theta + center_hex.index as f32 * 60.0) % 360.0;
            let theta = (theta + center_hex.offset_theta) % 360.0;
            transform.rotation = Quat::from_rotation_z(theta.to_radians() + 90.0_f32.to_radians());
            let x = theta.to_radians().cos() * CENTER_HEX_RADIUS;
            let y = theta.to_radians().sin() * CENTER_HEX_RADIUS;
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}


fn spawn_center_hex(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for i in 0..WALL_SIDES {
        let theta = (i as f32 * (360.0 / WALL_SIDES as f32)) + 30.0;
        let translation = Vec3::new(
            theta.to_radians().cos() * CENTER_HEX_RADIUS,
            theta.to_radians().sin() * CENTER_HEX_RADIUS,
            2.0,
        );
        let scale = Vec3::new(
            CENTER_HEX_RADIUS + (CENTER_HEX_RADIUS / 6.25),
            CENTER_HEX_HEIGHT,
            1.0,
        );
        let rotation = Quat::from_rotation_z(theta.to_radians() + 90.0_f32.to_radians());

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
            CenterHex { offset_theta: 30.0, index: i },
        ));
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
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
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
                        color: Color::GREEN,
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

    let mut triangle = Triangle2d {
        vertices: [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
        ],
    };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(triangle))
                .into(),
            material: materials.add(Color::NAVY).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
                scale: Vec3::new(60.0, 60.0, 2.0),
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
        1.0 / 60.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GamePlayerInputTimer(Timer::from_seconds(
        1.0 / 60.0,
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
    commands.insert_resource(BorderMoveTimer(Timer::from_seconds(
        1.0 / 30.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(CenterHexMoveTimer(Timer::from_seconds(
        1.0 / 30.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GameRotateTimer(Timer::from_seconds(
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
            transform.rotation = Quat::from_rotation_z(game.player.theta.to_radians() + 60.0_f32.to_radians());
        }
    }
}

fn game_cleanup(mut commands: Commands, query: Query<Entity, With<OnGameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

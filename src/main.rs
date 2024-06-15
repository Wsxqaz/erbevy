use bevy::prelude::*;

mod game;
mod menu;

use game::GamePlugin;
use menu::MenuPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    Exit,
}

const NUM_RINGS: usize = 10;
const NUM_SECTIONS: usize = 10;

#[derive(Resource)]
struct PhaseTimer(Timer);

#[derive(Default)]
struct Section {
    color: u32,
}

#[derive(Default)]
struct Ring {
    sections: [Section; NUM_SECTIONS],
}

#[derive(Resource, Default)]
struct Game {
    rings: [Ring; NUM_RINGS],
    menu: Menu,
    player: Player,
    walls: Walls,
}

#[derive(Default, Debug)]
struct Player {
    x: f32,
    y: f32,
    pos: u32,
}

#[derive(Default, Debug)]
struct Walls {
    last: u32,
}

#[derive(Component, Default)]
struct MenuItem {
    text: String,
}

#[derive(Default)]
struct Menu {
    hover: u32,
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin { ..default() }),
            MenuPlugin,
            GamePlugin,
        ))
        .init_resource::<Game>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_cameras)
        .add_systems(
            OnEnter(GameState::Exit),
            |mut commands: Commands, query: Query<Entity>| {
                for entity in query.iter() {
                    println!("Exiting: {:?}", entity);
                    commands.entity(entity).despawn_recursive();
                }
            },
        )
        .run();
}

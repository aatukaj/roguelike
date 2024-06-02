use core::panic;

use glam::{ivec2, IVec2};
use hecs::{Entity, World};

use crate::Map;

pub struct Game {
    pub player: Entity,
    pub map: Map,
    pub world: World,
    pub state: ControlState,
}

#[derive(Clone, Copy)]
pub enum ControlState {
    Player,
    Selection(IVec2),
}
impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        let player = world.spawn((
            Position(ivec2(0, 0)),
            Renderable {
                char: '@',
                color: [0, 255, 0],
            },
            Name("Player"),
        ));

        world.spawn((
            Position(ivec2(1, 2)),
            Renderable {
                char: 'A',
                color: [255, 0, 0],
            },
            Name("Enemy A"),
        ));

        Self {
            map: Map::grid(1000, 1000, 4, &mut world),
            world,
            player,
            state: ControlState::Player,
        }
    }
    pub fn player_pos(&mut self) -> IVec2 {
        self.world
            .query_one_mut::<&Position>(self.player)
            .expect("Player has no pos")
            .0
    }
}

pub trait Command {
    fn execute(&self, game: &mut Game);
}

pub struct MovePlayer(pub IVec2);
impl Command for MovePlayer {
    fn execute(&self, game: &mut Game) {
        let Ok(&pos) = game.world.query_one_mut::<&Position>(game.player) else {
            panic!("PLAYER has no position");
        };
        if !game
            .map
            .get_tile(pos.0 + self.0)
            .iter()
            .any(|e| game.world.query_one_mut::<&BlocksMoving>(*e).is_ok())
        {
            if let Ok(pos) = game.world.query_one_mut::<&mut Position>(game.player) {
                pos.0 += self.0;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position(pub IVec2);

#[derive(Debug)]
pub struct Renderable {
    pub char: char,
    pub color: [u8; 3],
}

#[derive(Debug)]
pub struct Name(pub &'static str);

#[derive(Debug)]
pub struct BlocksMoving;

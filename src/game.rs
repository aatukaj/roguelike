use glam::{ivec2, IVec2};
use hecs::{Entity, World};

use crate::{Map, Tile};

pub struct Game {
    pub player: Entity,
    pub map: Map,
    pub world: World,
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
        ));

        world.spawn((
            Position(ivec2(1, 2)),
            Renderable {
                char: 'A',
                color: [255, 0, 0],
            },
        ));

        Self {
            world,
            map: Map::grid(5000, 5000, 4),
            player,
        }
    }
}

pub trait Command {
    fn execute(&self, game: &mut Game);
}

pub struct MovePlayer(pub IVec2);
impl Command for MovePlayer {
    fn execute(&self, game: &mut Game) {
        let Ok(pos) = game.world.query_one_mut::<&mut Position>(game.player) else {
            return;
        };
        if game.map.get_tile(pos.0 + self.0) == &Tile::Air {
            pos.0 += self.0;
        }
    }
}

#[derive(Debug)]
pub struct Position(pub IVec2);

#[derive(Debug)]
pub struct Renderable {
    pub char: char,
    pub color: [u8; 3],
}

#[derive(Debug)]
pub struct Name(pub String);

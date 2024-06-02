use glam::{ivec2, IVec2, UVec2};
use hecs::{Entity, World};
use rand::random;

use crate::{BlocksMoving, Name, Position, Renderable};

pub struct Map {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<Entity>>,
    offset: IVec2,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![]; width * height],
            offset: ivec2(width as i32 / 2, height as i32 / 2),
        }
    }

    pub fn grid(width: usize, height: usize, size: usize, world: &mut World) -> Self {
        let mut map = Self::new(width, height);

        let mut spawn_helper = |x: usize, y: usize| {
            if !map.tiles[x + width * y].is_empty() {
                return;
            }
            let wall = world.spawn((
                Renderable {
                    char: '#',
                    color: [255, 255, 255],
                },
                BlocksMoving,
                Name("Wall"),
                Position(ivec2(x as i32, y as i32) - map.offset),
            ));
            map.tiles[x + width * y].push(wall);
        };

        for y in (0..height).step_by(size * 2 - 0) {
            for x in (0..width).step_by(size * 2 - 0) {
                if random::<f32>() < 0.5 {
                    continue;
                };

                for i in 0..size {
                    spawn_helper(x + i, y);
                    spawn_helper(x, y + i);
                    spawn_helper(x + i, y + size - 1);
                    spawn_helper(x + size - 1, y + i);
                }
            }
        }
        map
    }
    pub fn get_tile(&self, pos: IVec2) -> &[Entity] {
        let Ok(UVec2 { x, y }) = (pos + self.offset).try_into() else {
            return &[];
        };
        let x = x as usize;
        let y = y as usize;
        (x < self.width && y < self.height)
            .then(|| &*self.tiles[x + self.width * y])
            .unwrap_or(&[])
    }
}

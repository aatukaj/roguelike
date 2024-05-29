use std::iter;

use glam::{ivec2, IVec2, UVec2};
use hecs::Entity;
use rand::random;

pub fn parse_map(text: &str) -> Map {
    let width = text.lines().next().unwrap().len();
    let mut height = 0;
    let mut tiles = vec![];
    for line in text.lines() {
        height += 1;
        for c in line.chars() {
            tiles.push(match c {
                ' ' => Tile::Air,
                '1' => Tile::Wall,
                _ => panic!("Invalid map file char {c}"),
            })
        }
        tiles.extend(iter::repeat(Tile::Air).take(width - line.len()))
    }
    Map {
        width,
        height,
        tiles,
        offset: ivec2(width as i32 / 2, height as i32 / 2),
    }
}

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
        for y in (0..height).step_by(size * 2 - 0) {
            for x in (0..width).step_by(size * 2 - 0) {
                if random::<f32>() < 0.5 {
                    continue;
                };

                for i in 0..size {
                    map.tiles[x + i + width * y] = Tile::Wall;
                    map.tiles[x + width * (y + i)] = Tile::Wall;
                    map.tiles[x + i + width * (y + size - 1)] = Tile::Wall;
                    map.tiles[x + size - 1 + width * (y + i)] = Tile::Wall;
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

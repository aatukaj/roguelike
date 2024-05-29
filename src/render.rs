use crate::{Game, Renderable, Tile};
use glam::{ivec2, u16vec2, IVec2, U16Vec2};
use ratatui::prelude::*;

use crate::game::Position;

struct WallChars {
    horizontal: char,
    vertical: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    t_top: char,
    t_bottom: char,
    t_left: char,
    t_right: char,
    intersection: char,
}
const DOUBLE_WALL: WallChars = WallChars {
    horizontal: '═',
    vertical: '║',
    top_left: '╔',
    top_right: '╗',
    bottom_left: '╚',
    bottom_right: '╝',
    t_top: '╩',
    t_bottom: '╦',
    t_left: '╣',
    t_right: '╠',
    intersection: '╬',
};

pub struct RenderWidget;

struct RenderContext<'a> {
    cam: Camera,
    area: Rect,
    buffer: &'a mut Buffer,
}
struct Camera {
    pos: IVec2,
    size: IVec2,
}
impl Camera {
    fn top_left(&self) -> IVec2 {
        self.pos - self.size / 2
    }
    fn bottom_right(&self) -> IVec2 {
        self.pos + (self.size + 1) / 2 - 1
    }
}

impl<'a> RenderContext<'a> {
    pub fn new(area: Rect, buffer: &'a mut Buffer, state: &mut Game) -> Self {
        Self {
            cam: Camera {
                pos: state
                    .world
                    .query_one_mut::<&Position>(state.player)
                    .expect("No player!")
                    .0,
                size: ivec2(area.width as i32, area.height as i32),
            },
            area,
            buffer,
        }
    }
    pub fn render(&mut self, state: &mut Game) {
        eprintln!("safasd");
        for (_, (pos, rend)) in state.world.query_mut::<(&Position, &Renderable)>() {
            let [r, g, b] = rend.color;
            self.set_char(pos.0, rend.char, Style::default().fg(Color::Rgb(r, g, b)));
        }

        self.set_char(self.cam.top_left(), '#', Style::default());
        self.set_char(self.cam.bottom_right(), '#', Style::default());
        self.render_walls(state);
    }
    fn render_walls(&mut self, state: &mut Game) {
        let tl = self.cam.top_left();
        let br = self.cam.bottom_right();
        for y in tl.y..=br.y {
            for x in tl.x..=br.x {
                self.render_wall(ivec2(x, y), state);
            }
        }
    }

    fn render_wall(&mut self, pos: IVec2, state: &mut Game) {
        let tile = state.map.get_tile(pos);
        if tile != &Tile::Wall {
            return;
        };
        let l = state.map.get_tile(pos - ivec2(1, 0)) == &Tile::Wall;
        let r = state.map.get_tile(pos + ivec2(1, 0)) == &Tile::Wall;
        let u = state.map.get_tile(pos - ivec2(0, 1)) == &Tile::Wall;
        let d = state.map.get_tile(pos + ivec2(0, 1)) == &Tile::Wall;

        let c = match (l, r, u, d) {
            (true, true, true, true) => DOUBLE_WALL.intersection,
            (true, true, true, false) => DOUBLE_WALL.t_top,
            (true, true, false, true) => DOUBLE_WALL.t_bottom,
            (true, true, false, false) => DOUBLE_WALL.horizontal,
            (true, false, true, true) => DOUBLE_WALL.t_left,
            (true, false, true, false) => DOUBLE_WALL.bottom_right,
            (true, false, false, true) => DOUBLE_WALL.top_right,
            (true, false, false, false) => DOUBLE_WALL.horizontal,
            (false, true, true, true) => DOUBLE_WALL.t_right,
            (false, true, true, false) => DOUBLE_WALL.bottom_left,
            (false, true, false, true) => DOUBLE_WALL.top_left,
            (false, true, false, false) => DOUBLE_WALL.horizontal,
            (false, false, true, true) => DOUBLE_WALL.vertical,
            (false, false, true, false) => DOUBLE_WALL.vertical,
            (false, false, false, true) => DOUBLE_WALL.vertical,
            (false, false, false, false) => '#',
        };
        self.set_char(
            pos,
            c,
            Style::default()
                .bg(Color::Rgb(100, 100, 100))
                .fg(Color::Rgb(255, 255, 255)),
        );
    }

    fn buffer_pos(&self, pos: IVec2) -> Option<U16Vec2> {
        let pos = pos + self.cam.size / 2 - self.cam.pos;
        if 0 <= pos.x && pos.x < self.cam.size.x && 0 <= pos.y && pos.y < self.cam.size.y {
            pos.try_into()
                .ok()
                .map(|p: U16Vec2| p + u16vec2(self.area.x, self.area.y))
        } else {
            None
        }
    }

    fn set_char(&mut self, pos: IVec2, c: char, style: Style) {
        let Some(U16Vec2 { x, y }) = self.buffer_pos(pos) else {
            return;
        };
        self.buffer.get_mut(x, y).set_char(c).set_style(style);
    }
}

impl StatefulWidget for RenderWidget {
    type State = Game;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut ctx = RenderContext::new(area, buf, state);
        ctx.render(state);
    }
}

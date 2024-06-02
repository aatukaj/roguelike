use crate::{ControlState, Game, Renderable};
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
        for (_, (pos, rend)) in state.world.query_mut::<(&Position, &Renderable)>() {
            let [r, g, b] = rend.color;
            self.set_char(pos.0, rend.char, Style::default().fg(Color::Rgb(r, g, b)));
        }

        self.set_char(self.cam.top_left(), '#', Style::default());
        self.set_char(self.cam.bottom_right(), '#', Style::default());
        if let ControlState::Selection(pos) = state.state {
            self.set_char(
                pos + ivec2(1, 0),
                '<',
                Style::default().fg(Color::Rgb(255, 196, 0)),
            );
            self.set_char(
                pos - ivec2(1, 0),
                '>',
                Style::default().fg(Color::Rgb(255, 196, 0)),
            )
        }
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

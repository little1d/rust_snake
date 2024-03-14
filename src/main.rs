use piston_window::{Button, clear, PistonWindow, PressEvent, UpdateEvent, WindowSettings};
use piston_window::types::Color;
use crate::draw::to_coord_u32;
use crate::game::Game;

mod draw;
mod snake;
mod game;

const  BACKGROUD_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

// 使用Piston游戏引擎创建一个窗口
fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _|{
            clear(BACKGROUD_COLOR, g);
            game.draw(&c, g);
        });
        // 每一帧更新一次，接受arg参数的必爆
        event.update(|arg|{
            game.update(arg.dt);
        });
    }
}

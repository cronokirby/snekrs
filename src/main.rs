extern crate piston_window;
use piston_window::*;

mod snake;
use snake::*;

struct Game {
    input_direction: Option<Key>,
    snake: Snake
}

impl Game {
    fn new() -> Game {
        Game {input_direction: None, snake: Snake::new()}
    }
    // callback for the drawing event
    fn on_draw(&mut self, e: &Input, ren: RenderArgs, win: &mut PistonWindow) {
        const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
        win.draw_2d(e, |c, g| {
           clear(BLACK, g);
           let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
           self.snake.render(g, center);
        });
    }
    fn on_update(&mut self, upd: UpdateArgs) {
        self.snake.mov(&upd.dt, &self.input_direction)
    }
    // no need to handle a release
    fn on_press(&mut self, but: &Button) {
        match *but {
            Button::Keyboard(Key::A) => {
                self.snake.grow();
            },
            Button::Keyboard(k) => {
                self.input_direction = Some(k);
            },
            _ => {}
        }
    }
}

fn main() {
    use Input::*;
    let mut window: PistonWindow = WindowSettings::new("snekrs", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new();
    while let Some(e) = window.next() {
        match e {
            Update(upd) => {
                game.on_update(upd);
            },
            Render(ren) => {
                game.on_draw(&e, ren, &mut window);
            },
            Press(but) => {
                game.on_press(&but);
            }
            _ => {}
        }
    }
}

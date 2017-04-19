use piston_window::*;

struct Point {
    x: f64, y: f64
}

pub struct Snake {
    head: Point,
    tail: Vec<Point>
}

impl Snake {
    pub fn new() -> Snake {
        Snake {head: Point{x: 0.0, y: 0.0}, tail: Vec::new()}
    }
    pub fn grow(&mut self) {
        if self.tail.is_empty() {
            self.tail.push(Point{..self.head});
        } else {
            self.tail.push(Point{x: 1000.0, y: 1000.0});
        }
    }
    pub fn mov(&mut self, dt: &f64, d: &Option<Key>) {
        const SPEED: f64 = 300.0;
        if let Some(d) = *d {
            match self.tail.len() {
                0 => {},
                1 => {
                    self.tail[0] = Point{..self.head};
                },
                _ => {
                    self.tail.pop();
                    self.tail.insert(0, Point{..self.head});
                }
            }
            match d {
                Key::Up => {
                    self.head.y -= SPEED * dt;
                },
                Key::Down => {
                    self.head.y += SPEED * dt;
                },
                Key::Left => {
                    self.head.x -= SPEED * dt;
                },
                Key::Right => {
                    self.head.x += SPEED * dt;
                },
                _ => {}
            }
        }
    }
    pub fn render<G>(&self, g: &mut G, view: math::Matrix2d) where G: Graphics {
        const RED: [f32;4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 20.0);
        let pos = translate(&self.head, &view);
        rectangle(RED, square, pos, g);
        for p in &self.tail {
            let pos = translate(&p, &view);
            rectangle(RED, square, pos, g);
        }
    }
}


fn translate(point: &Point, view: &math::Matrix2d) -> math::Matrix2d {
    view.trans(point.x, point.y).trans(-10.0, -10.0)
}

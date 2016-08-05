use EPSILON;

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
    pub speed: f32,
    pub speed_x: f32,
    pub speed_y: f32,
}

fn speed_projection(spped: f32, direction: f32) -> (f32, f32) {
    let angle = direction.to_radians();
    (spped * angle.cos(), spped * angle.sin())
}

fn position_collision(p1: f32, p2: f32) -> bool {
    (p1 - p2).abs() < EPSILON
}

impl Point {
    pub fn new(x: f32, y: f32, direction: f32, speed: f32) -> Point {
        let (speed_x, speed_y) = speed_projection(speed, direction);
        Point {
            x: x,
            y: y,
            direction: direction,
            speed: speed,
            speed_x: speed_x,
            speed_y: speed_y,
        }
    }

    pub fn step(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if position_collision(self.x, 1.0) {
            self.speed_x = -self.speed_x;
            if self.direction < 90.0 {
                self.direction = 180.0 - self.direction;
            } else {
                self.direction = 540.0 - self.direction;
            }
        }
        if position_collision(self.x, -1.0) {
            self.speed_x = -self.speed_x;
            if self.direction < 180.0 {
                self.direction = 180.0 - self.direction;
            } else {
                self.direction = 540.0 - self.direction;
            }
        }

        if position_collision(self.y, 1.0) || position_collision(self.y, -1.0) {
            self.speed_y = -self.speed_y;
            self.direction = 360.0 - self.direction;
        }

    }
}

#[test]
#[ignore]
fn test_speed_projection() {
    assert!(false);
}

#[test]
#[ignore]
fn test_position_collision() {
    assert!(false);
}

#[test]
#[ignore]
fn test_point() {
    assert!(false);
}

#[test]
#[ignore]
fn test_point_move() {
    assert!(false);
}

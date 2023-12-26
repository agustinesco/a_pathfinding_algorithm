use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }

    pub fn generate_random_position(heigth: usize, width: usize) -> Position {
        let mut rng = rand::thread_rng();
        let random_start_x = rng.gen_range(0..width);
        let random_start_y = rng.gen_range(0..heigth);
        Position::new(random_start_x, random_start_y)
    }

    pub fn distance_to_other_position(self, other_point: Position) -> i32 {
        let x_distance = i32::abs(other_point.x as i32 - self.x as i32);
        let y_distance = i32::abs(other_point.y as i32 - self.y as i32);

        let remaining = i32::abs(x_distance - y_distance);

        return 14 * i32::min(x_distance, y_distance) + 10 * remaining;
    }

}

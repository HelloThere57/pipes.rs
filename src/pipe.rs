use crossterm::style::Color;

use rand::{distributions::Standard, prelude::Distribution, rngs::ThreadRng, Rng, RngCore};

use crate::boilerplate::Direction;

pub struct Pipe {
    pub x: u16,
    pub y: u16,
    direction: Direction,
    pub color: Color,
}
impl Pipe {
    pub fn new(x: u16, y: u16, direction: Direction, color: Color) -> Self {
        Pipe {
            x,
            y,
            direction,
            color,
        }
    }
    pub fn tick(
        &mut self,
        rng: &mut ThreadRng,
        turn_chance: f64,
        term_width: u16,
        term_height: u16,
        char_map: [char; 6],
    ) -> char {
        self.move_forward(term_height, term_width);

        let old_direction = self.direction;
        let _ = self.maybe_turn(rng, turn_chance);

        pipe_connecting(old_direction.opposite(), self.direction, char_map)
    }

    fn move_forward(&mut self, term_height: u16, term_width: u16) {
        match self.direction {
            Direction::Up => self.y = wrap_sub(self.y, 1, 0, term_height),
            Direction::Down => self.y = wrap_add(self.y, 1, 0, term_height),
            Direction::Left => self.x = wrap_sub(self.x, 1, 0, term_width),
            Direction::Right => self.x = wrap_add(self.x, 1, 0, term_width),
        }
    }

    fn maybe_turn(&mut self, rng: &mut ThreadRng, turn_chance: f64) -> bool {
        if rng.gen_bool(turn_chance.clamp(0.0, 1.0)) {
            match rng.gen_bool(0.5) {
                true => self.direction = self.direction.clockwise(),
                false => self.direction = self.direction.counterclockwise(),
            }
            true
        } else {
            false
        }
    }
    pub fn new_random(term_width: u16, term_height: u16, rng: &mut ThreadRng) -> Self {
        let mut s: Pipe = rng.gen();
        s.x = rng.next_u32() as u16 % term_width;
        s.x = rng.next_u32() as u16 % term_height;
        s
    }
}

pub fn pipe_connecting(old_direction: Direction, direction: Direction, map: [char; 6]) -> char {
    #[rustfmt::skip]
    const TABLE: [[usize; 4]; 4] = [
        [0, 0, 4, 2],
        [0, 0, 5, 3],
        [4, 5, 1, 1],
        [2, 3, 1, 1],
    ];

    map[TABLE[direction as usize][old_direction as usize]]
}

impl Distribution<Pipe> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pipe {
        let direction: Direction = match rng.next_u32() % 4 {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            x => unreachable!("mod 4 resulted in {}", x),
        };

        Pipe {
            x: 0,
            y: 0,
            direction,
            color: Color::AnsiValue((rng.next_u32() % 16) as u8),
        }
    }
}

fn wrap_add(p1: u16, p2: u16, min: u16, max: u16) -> u16 {
    let (p1, p2, min, max) = (p1 as u32, p2 as u32, min as u32, max as u32);
    let num = p1 + p2;
    (((((num - min) % (max - min)) + (max - min)) % (max - min)) + min) as u16
}
fn wrap_sub(p1: u16, p2: u16, min: u16, max: u16) -> u16 {
    let (p1, p2, min, max) = (p1 as i32, p2 as i32, min as i32, max as i32);
    let num = p1 - p2;
    (((((num - min) % (max - min)) + (max - min)) % (max - min)) + min) as u16
}

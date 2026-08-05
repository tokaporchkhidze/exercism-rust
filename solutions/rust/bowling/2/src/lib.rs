const FRAMES: usize = 10;
const MAX_ROLLS: usize = 3;

const PINS: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    rolls: [Option<u16>; MAX_ROLLS],
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            rolls: [None; MAX_ROLLS],
        }
    }
}

impl Frame {
    fn pins_left(&mut self) -> u16 {
        match self.rolls {
            [None, None, None] => PINS,
            [Some(first), None, None] if first == PINS => PINS,
            [Some(first), None, None] => PINS - first,
            [Some(first), Some(second), None] => {
                if first + second == PINS || second == PINS {
                    PINS
                } else {
                    PINS - second
                }
            }
            [_, _, _] => PINS,
        }
    }

    fn is_complete(&self, is_last_frame: bool) -> bool {
        let (sum, count) = self
            .rolls
            .iter()
            .flatten()
            .fold((0, 0), |(sum, count), &roll| (sum + roll, count + 1));
        if is_last_frame {
            match count {
                0..2 => false,
                c if c == 2 && sum < PINS => true,
                c if c == 2 && sum == PINS => false,
                c if c == 2 && sum > PINS => false,
                _ => true,
            }
        } else {
            sum == PINS || count == 2
        }
    }
}

pub struct BowlingGame {
    frames: [Frame; FRAMES],
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: std::array::from_fn(|_| Default::default()),
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.current_frame == FRAMES {
            return Err(Error::GameComplete);
        }

        let frame = &mut self.frames[self.current_frame];
        let pins_left = frame.pins_left();
        if pins > pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }
        let slot = frame.rolls.iter_mut().find(|el| el.is_none());
        *slot.expect("incomplete frame always has a free slot") = Some(pins);
        if frame.is_complete(self.current_frame == 9) {
            self.current_frame += 1;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.current_frame < FRAMES {
            return None;
        }

        let rolls: Vec<u16> = self
            .frames
            .iter()
            .flat_map(|frame| frame.rolls.iter().flatten().copied())
            .collect();

        let mut total = 0;
        let mut i = 0;
        for _ in 0..FRAMES {
            if rolls[i] == PINS {
                total += PINS + rolls[i + 1] + rolls[i + 2];
                i += 1;
            } else if rolls[i] + rolls[i + 1] == PINS {
                total += PINS + rolls[i + 2];
                i += 2;
            } else {
                total += rolls[i] + rolls[i + 1];
                i += 2;
            }
        }

        Some(total)
    }
}

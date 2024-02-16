use core::fmt;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::Debug;

mod ascii;

pub struct Board {
    board: [Option<TicTac>; 9],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TicTac {
    X,
    O,
}

pub struct Wins {
    x: u8,
    o: u8,
}

impl Wins {
    pub fn new() -> Wins {
        Wins {
            x: 0,
            o: 0,
        }
    }
}

impl Wins {
    pub fn add(&mut self, tic_tac: TicTac) {
        if tic_tac == TicTac::X {
            self.x += 1;
        } else {
            self.o += 1;
        }
    }
}

impl Wins {
    pub fn get(&self, tic_tac: TicTac) -> u8 {
        if tic_tac == TicTac::X {
            self.x
        } else {
            self.o
        }
    }
}

impl Wins {
    pub fn winner(&self) -> TicTac {
        if self.x > self.o {
            TicTac::X
        } else {
            TicTac::O
        }
    }
}

impl Wins {
    pub fn loser(&self) -> TicTac {
        if self.x < self.o {
            TicTac::X
        } else {
            TicTac::O
        }
    }
}

impl fmt::Display for TicTac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TicTac::X => write!(f, "X"),
            TicTac::O => write!(f, "O"),
        }
    }
}

impl Distribution<TicTac> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TicTac {
        let index: u8 = rng.gen_range(0..2);
        match index {
            0 => TicTac::X,
            1 => TicTac::O,
            _ => unreachable!(),
        }
    }
}

pub enum Size {
    Small,
    Large,
}

impl Board {
    pub fn new() -> Board {
        Board { board: [None; 9] }
    }
}

impl Board {
    pub fn try_mark(&mut self, index: usize, mark: &TicTac) -> Result<(), &'static str> {
        if self.board[index].is_none() {
            self.board[index] = Some(*mark);
            Ok(())
        } else {
            Err("slot already occupied")
        }
    }
}

impl Board {
    pub fn winner(&self) -> Option<TicTac> {
        if self.has_won(TicTac::X) {
            Some(TicTac::X)
        } else if self.has_won(TicTac::O) {
            Some(TicTac::O)
        } else {
            None
        }
    }
}

impl Board {
    fn has_won(&self, tic_tac: TicTac) -> bool {
        for i in 0..3 {
            if ((self.board[i % 3] == Some(tic_tac)) // check for vertical wins
                && (self.board[i % 3] == self.board[(i % 3) + 3])
                && (self.board[i % 3] == self.board[(i % 3) + 6]))
                || ((self.board[i * 3] == Some(tic_tac)) // check for horizontal wins
                    && (self.board[i * 3] == self.board[(i * 3) + 1])
                    && (self.board[i * 3] == self.board[(i * 3) + 2]))
            {
                return true;
            }
        }

        // there isn't any good pattern between the two diagonal wins for a 3x3 grid,
        // so the easiest thing to do is to check manually
        ((self.board[0] == Some(tic_tac)) // check for diagonal top-left--bottom-right wins
            && (self.board[0] == self.board[4])
            && (self.board[0] == self.board[8]))
            || ((self.board[2] == Some(tic_tac)) // check for diagonal top-right--bottom-left wins
                && (self.board[2] == self.board[4])
                && (self.board[2] == self.board[6]))
    }
}

impl Board {
    pub fn print(&self, size: &Size) {
        println!("-------------------------------------------------------------");

        for i in 0..3 {
            for j in 0..8 {
                println!(
                    "|{}|{}|{}|",
                    ascii::get_frame(self.board[i * 3], j),
                    ascii::get_frame(self.board[(i * 3) + 1], j),
                    ascii::get_frame(self.board[(i * 3) + 2], j)
                );
            }
            if i != 2 {
                println!("|-------------------|-------------------|-------------------|");
            } else {
                println!("-------------------------------------------------------------");
            }
        }
    }
}

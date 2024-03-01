mod board;
mod command;

use {
    board::{Board, Size, TicTac, Wins},
    clap::{Parser, ValueEnum},
    core::fmt,
    rand::Rng,
    std::{cmp, fmt::Debug, io, process},
};

//TODO: move all command related stuff into command

/// Tic-tac-toe game with 1 and 2 player modes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Use a small board, instead of the default large one
    #[arg(short = 's', long, default_value_t = false)]
    small_board: bool,

    /// Start in 2-player mode
    #[arg(short = 'p', long, default_value_t = true)]
    second_player: bool,

    /// How many wins to play until
    #[arg(short = 'o', long, default_value_t = 1)]
    out_of: u8,

    /// Makes the player go first, regardless of first-turn setting. If false, first turn is decided by first-turn
    #[arg(short = 'f', long)]
    human_first: bool,

    /// Which player should go first when beginning a game
    #[arg(short = 't', long, default_value_t = FirstTurn::Random)]
    first_turn: FirstTurn,

    /// Difficulty of the AI
    #[arg(short = 'd', long, default_value_t = Difficulty::Easy)]
    difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FirstTurn {
    Winner,
    Loser,
    Random,
    X,
    O,
}

impl FirstTurn {
    fn get_tic_tac(&self) -> TicTac {
        //TODO: implement a match for all first_turn's
        rand::thread_rng().gen()
    }
}

impl fmt::Display for FirstTurn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FirstTurn::Winner => write!(f, "winner"),
            FirstTurn::Loser => write!(f, "loser"),
            FirstTurn::Random => write!(f, "random"),
            FirstTurn::X => write!(f, "x"),
            FirstTurn::O => write!(f, "o"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Difficulty {
    Easy,
    Medium,
    Impossible,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Difficulty::Easy => write!(f, "easy"),
            Difficulty::Medium => write!(f, "medium"),
            Difficulty::Impossible => write!(f, "impossible"),
        }
    }
}

pub fn run(args: Args) {
    print_intro(&args);

    let mut input = String::new();
    wait_for_enter(&mut input);
    Board::new().print(&Size::Large);

    let mut wins = Wins::new();
    let mut scratches = 0;

    #[allow(clippy::never_loop)]
    while cmp::max(wins.get(TicTac::X), wins.get(TicTac::O)) < args.out_of {
        let mut turn = args.first_turn.get_tic_tac();
        let mut board = Board::new();
        let mut num_turns = 0;
        loop {
            println!("{}'s turn: ", turn);
            input.clear();
            get_input(&mut input);

            // "7" | "q" => { if board.try_mark(0, &turn).is_err() { continue;}},
            // "8" | "w" => { if board.try_mark(1, &turn).is_err() { continue;}},
            // "9" | "e" => { if board.try_mark(2, &turn).is_err() { continue;}},
            // "4" | "a" => { if board.try_mark(3, &turn).is_err() { continue;}},
            // "5" | "s" => { if board.try_mark(4, &turn).is_err() { continue;}},
            // "6" | "d" => { if board.try_mark(5, &turn).is_err() { continue;}},
            // "1" | "z" => { if board.try_mark(6, &turn).is_err() { continue;}},
            // "2" | "x" => { if board.try_mark(7, &turn).is_err() { continue;}},
            // "3" | "c" => { if board.try_mark(8, &turn).is_err() { continue;}},

            match input.trim() {
                //TODO: make this more concise with a match
                "7" | "q" => {
                    if board.try_mark(0, &turn).is_err() {
                        continue;
                    }
                }
                "8" | "w" => {
                    if board.try_mark(1, &turn).is_err() {
                        continue;
                    }
                }
                "9" | "e" => {
                    if board.try_mark(2, &turn).is_err() {
                        continue;
                    }
                }
                "4" | "a" => {
                    if board.try_mark(3, &turn).is_err() {
                        continue;
                    }
                }
                "5" | "s" => {
                    if board.try_mark(4, &turn).is_err() {
                        continue;
                    }
                }
                "6" | "d" => {
                    if board.try_mark(5, &turn).is_err() {
                        continue;
                    }
                }
                "1" | "z" => {
                    if board.try_mark(6, &turn).is_err() {
                        continue;
                    }
                }
                "2" | "x" => {
                    if board.try_mark(7, &turn).is_err() {
                        continue;
                    }
                }
                "3" | "c" => {
                    if board.try_mark(8, &turn).is_err() {
                        continue;
                    }
                }
                "Q" => {
                    println!("Game exited");
                    return;
                }
                "h" => {
                    print_help();
                    continue;
                }
                "" => {
                    board.print(&Size::Large);
                    continue; //don't change turn
                }
                unexpected => {
                    println!("'{}' is not a command", unexpected);
                    println!("Enter 'h' to see help.");
                    continue;
                }
            }

            board.print(&Size::Large);
            turn = if turn == TicTac::X {
                TicTac::O
            } else {
                TicTac::X
            };
            num_turns += 1;

            if let Some(winner) = board.winner() {
                wins.add(winner);

                board.print(&Size::Large);                
                println!("{} won the match in {} turns!", winner, num_turns);
                wait_for_enter(&mut input);
                
                if cmp::max(wins.get(TicTac::X), wins.get(TicTac::O)) < args.out_of {
                    Board::new().print(&Size::Large);
                }

                break;
            } else if num_turns >= 9 {
                println!("The match was a scratch, and there was no winner.");
                wait_for_enter(&mut input);

                scratches += 1;

                Board::new().print(&Size::Large);

                break;
            }
        }
    }

    if args.out_of > 1 {
        println!(
            "\
The overall winner is {}, with {} wins, while {} had {} wins;
while there were {} scratches.",
            wins.winner(),
            wins.get(wins.winner()),
            wins.loser(),
            wins.get(wins.loser()),
            scratches
        );
    }
}

fn get_input(buf: &mut String) {
    io::stdin().read_line(buf).unwrap_or_else(|err| {
        println!("Unexpected input error: {}", err);
        process::exit(1);
    });
}

fn wait_for_enter(input: &mut String) {
    println!("Press enter to continue...");
    get_input(input);
}

fn print_intro(args: &Args) {
    println!("Welcome to Tic-Tac-Toe, Written by Iris!");
    print_help();
    print_settings(args);
}

fn print_help() {
    println!(
        "\
Run this program with '--help' to see options.
To place marks, use the numpad:
7 8 9
4 5 6
1 2 3
or if you don't have one, use:
q w e 
 a s d
  z x c
other shit
Press enter at any time to see the board.

And pretend it isn't all staggered and wonky"
    );
    //TODO: add comprehensive help
}

fn print_settings(args: &Args) {
    println!(
        "\
\nSettings:"
    );
    dbg!(args);
    //TODO: implement print_settings
}

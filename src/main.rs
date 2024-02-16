use clap::Parser;

fn main() {
    let args = tic_tac_toe::Args::parse();

    //dbg!(&args);

    tic_tac_toe::run(args);
}

//TODO: fix the switch in lib.rs :sob:

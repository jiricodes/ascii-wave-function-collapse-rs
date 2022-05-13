mod grid;
mod tile;

use grid::Grid;
use tile::SymbolMap;

use rand::prelude::*;

use clap::Parser;
use ron::ser::{to_string_pretty, PrettyConfig};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the map
    #[clap(short, long, default_value_t = 80)]
    width: usize,

    /// Height of the map
    #[clap(short, long, default_value_t = 10)]
    height: usize,

    /// Random seed
    #[clap(short, long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let symmap = SymbolMap::new();

    // let pretty = PrettyConfig::new()
    //     .depth_limit(3)
    //     .separate_tuple_members(true)
    //     .enumerate_arrays(true);
    // let s = to_string_pretty(&symmap, pretty).expect("Serialization failed");
    // println!("{}", s);

    let mut grid = Grid::new(args.width, args.height);

    let seed: u64 = if args.seed.is_none() {
        rand::thread_rng().gen()
    } else {
        args.seed.unwrap()
    };
    dbg!(seed);
    grid.collapse(seed, &symmap);
    println!("{}", grid);
}

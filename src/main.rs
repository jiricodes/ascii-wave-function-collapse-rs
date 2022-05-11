mod grid;
mod tile;

use grid::Grid;
use tile::SymbolMap;

use rand::prelude::*;

use ron::ser::{to_string_pretty, PrettyConfig};

fn main() {
    let symmap = SymbolMap::new();

    let pretty = PrettyConfig::new()
        .depth_limit(3)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    let s = to_string_pretty(&symmap, pretty).expect("Serialization failed");
    println!("{}", s);

    let mut grid = Grid::new(120, 20);
    // println!("{}", grid);

    let seed: u64 = rand::thread_rng().gen();
    grid.collapse(seed, &symmap);
    println!("{}", grid);
    dbg!(seed);
}

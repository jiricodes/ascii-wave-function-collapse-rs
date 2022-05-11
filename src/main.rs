mod grid;
mod tile;

use grid::Grid;
use tile::SymbolMap;

use rand::prelude::*;

fn main() {
    let symmap = SymbolMap::new();
    let mut grid = Grid::new(120, 20);
    println!("{}", grid);

    let seed: u64 = rand::thread_rng().gen();
    grid.collapse(seed, &symmap);
    println!("{}", grid);
    dbg!(seed);
}

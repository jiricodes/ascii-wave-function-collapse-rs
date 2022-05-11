mod grid;
mod tile;

use grid::Grid;
use tile::SymbolMap;

fn main() {
    let symmap = SymbolMap::new();
    let mut grid = Grid::new(10, 10);
    grid.collapse(2, &symmap);
    println!("{}", grid);
}

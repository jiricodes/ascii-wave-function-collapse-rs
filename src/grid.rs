use crate::tile::{Direction, SymbolMap, Tile};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::VecDeque;
use std::fmt;

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    min_opts_index: Option<usize>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::with_capacity(width * height);
        for l in 0..height {
            for c in 0..width {
                let mut tile = Tile::default();
                if l == 0 {
                    tile.prune(SymbolMap::TOP_EDGE);
                } else if l == height - 1 {
                    tile.prune(SymbolMap::BOTTOM_EDGE);
                }
                if c == 0 {
                    tile.prune(SymbolMap::LEFT_EDGE);
                } else if c == width - 1 {
                    tile.prune(SymbolMap::RIGHT_EDGE);
                }
                tiles.push(tile);
            }
        }
        Self {
            width,
            height,
            tiles,
            min_opts_index: Some((width * height) / 2 + width / 2),
        }
    }

    pub fn collapse(&mut self, seed: u64, symmap: &SymbolMap) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        self.min_opts_index = Some(rng.gen_range(0..self.width * self.height));
        while self.min_opts_index.is_some() {
            let curri = self.min_opts_index.unwrap();
            let mut t = self.tiles.get_mut(curri).unwrap();
            t.value = symmap.rng_pick(&t.value, &mut rng);
            self.prune(curri, symmap);
        }
    }

    pub fn prune(&mut self, start: usize, symmap: &SymbolMap) {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<usize> = Vec::new();
        q.push_back(start);
        while !q.is_empty() {
            let current = q.pop_front().unwrap();
            visited.push(current);
            let current_tile_clone = self.tiles[current].clone();
            let all_n = self.neighbors(current);
            for (n_idx, dir) in all_n.iter() {
                let n = self.tiles.get_mut(*n_idx).unwrap();
                // if neighbor has solution, then ignore it
                if n.is_set() || visited.contains(n_idx) {
                    continue;
                }
                // else prune its options and append it to the queue
                if n.prune_with_other_in_dir(&current_tile_clone, dir, symmap) {
                    q.push_back(*n_idx);
                }
            }
        }
        self.find_min();
    }

    fn find_min(&mut self) {
        self.min_opts_index = None;
        for i in 0..self.width * self.height {
            let l = self.tiles[i].value.len();
            if l > 1 {
                match self.min_opts_index {
                    Some(val) => {
                        if l < self.tiles[val].value.len() {
                            self.min_opts_index = Some(i);
                        }
                    }
                    None => {
                        self.min_opts_index = Some(i);
                    }
                }
            } else if l == 0 {
                println!("{}", self);
                panic!("Tile {}, ran out of options", i);
            }
        }
    }

    fn neighbors(&self, i: usize) -> Vec<(usize, Direction)> {
        let mut ret = Vec::new();
        // top
        if i >= self.width {
            ret.push((i - self.width, Direction::Top));
        }
        // right
        if i % self.width != self.width - 1 {
            ret.push((i + 1, Direction::Right));
        }
        // botom
        if i < (self.height - 1) * self.width {
            ret.push((i + self.width, Direction::Bottom));
        }
        // left
        if i % self.width != 0 {
            ret.push((i - 1, Direction::Left));
        }
        ret
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut g = String::with_capacity(self.width * self.height);
        for i in 0..self.width * self.height {
            if i % self.width == self.width - 1 {
                g += &format!("{}\n", self.tiles[i]);
            } else {
                g += &format!("{}", self.tiles[i]);
            }
        }
        write!(f, "Dimensions: {}x{}\n{}", self.width, self.height, g)
    }
}

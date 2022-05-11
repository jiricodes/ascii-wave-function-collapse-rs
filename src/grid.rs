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
        for _ in 0..width * height {
            tiles.push(Tile::default());
        }
        Self {
            width,
            height,
            tiles,
            min_opts_index: Some((width * height) / 2 + width / 2),
        }
    }

    fn user_break() {
        use std::io::{stdin, stdout, Write};
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
    }

    pub fn collapse(&mut self, seed: u64, symmap: &SymbolMap) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        while self.min_opts_index.is_some() {
            let mut t = self.tiles.get_mut(self.min_opts_index.unwrap()).unwrap();
            let i: usize = rng.gen_range(0..t.value.len() - 1);
            t.value = t.value.chars().nth(i).unwrap().to_string();
            self.prune(self.min_opts_index.unwrap(), symmap);
            println!("{}", self);
            Grid::user_break();
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
                if n.is_set() || visited.contains(&n_idx) {
                    continue;
                }
                // else prune its options and append it to the queue
                if n.prune_with_other_in_dir(&current_tile_clone, &dir, symmap) {
                    q.push_back(*n_idx);
                }
            }
        }
        self.find_min();
        dbg!(&self.min_opts_index);
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
        write!(f, "{}", g)
    }
}

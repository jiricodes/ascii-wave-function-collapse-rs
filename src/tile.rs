use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Top,
        Direction::Right,
        Direction::Bottom,
        Direction::Left,
    ];
    pub const COUNT: usize = 4;
    pub fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Left => Self::Right,
            Self::Bottom => Self::Top,
            Self::Right => Self::Left,
        }
    }
}

pub struct SymbolMap {
    map: HashMap<char, HashMap<Direction, String>>,
}

impl SymbolMap {
    pub const ALLPIECES: &'static str = " /\\_#";
    // pub const STARTPIECES: &'static str = " /\\";
    const HILLPIECES: &'static str = "/\\_#";

    pub const TOP_EDGE: &'static str = " _";
    pub const RIGHT_EDGE: &'static str = " \\_#";
    pub const BOTTOM_EDGE: &'static str = " /\\#";
    pub const LEFT_EDGE: &'static str = " /_#";

    pub fn new() -> Self {
        let mut map = HashMap::new();
        let (c, n) = Self::empty();
        map.insert(c, n);
        let (c, n) = Self::right_slope();
        map.insert(c, n);
        let (c, n) = Self::left_slope();
        map.insert(c, n);
        let (c, n) = Self::hill_top();
        map.insert(c, n);
        let (c, n) = Self::hill_rock();
        map.insert(c, n);
        Self { map }
    }

    fn get(&self, c: char, d: &Direction) -> Option<&str> {
        if let Some(n) = self.map.get(&c) {
            if let Some(opts) = n.get(d) {
                return Some(opts.as_str());
            }
        }
        None
    }

    pub fn get_dirs(&self, c: char) -> Option<&HashMap<Direction, String>> {
        self.map.get(&c)
    }

    fn empty() -> (char, HashMap<Direction, String>) {
        let value = ' ';
        let mut neighbors = HashMap::with_capacity(Direction::COUNT);
        neighbors.insert(Direction::Top, " /\\#".to_string());
        neighbors.insert(Direction::Left, " \\_".to_string());
        neighbors.insert(Direction::Bottom, " /\\_".to_string());
        neighbors.insert(Direction::Right, " /_".to_string());
        (value, neighbors)
    }

    fn right_slope() -> (char, HashMap<Direction, String>) {
        let value = '/';
        let mut neighbors = HashMap::with_capacity(Direction::COUNT);
        neighbors.insert(Direction::Top, " \\".to_string());
        neighbors.insert(Direction::Left, " _".to_string());
        neighbors.insert(Direction::Bottom, " \\_#".to_string());
        neighbors.insert(Direction::Right, "#\\".to_string());
        (value, neighbors)
    }

    fn left_slope() -> (char, HashMap<Direction, String>) {
        let value = '\\';
        let mut neighbors = HashMap::with_capacity(Direction::COUNT);
        neighbors.insert(Direction::Top, " /".to_string());
        neighbors.insert(Direction::Left, "/#".to_string());
        neighbors.insert(Direction::Bottom, " /_#".to_string());
        neighbors.insert(Direction::Right, " _".to_string());
        (value, neighbors)
    }

    fn hill_top() -> (char, HashMap<Direction, String>) {
        let value = '_';
        let mut neighbors = HashMap::with_capacity(Direction::COUNT);
        neighbors.insert(Direction::Top, " /\\#".to_string());
        neighbors.insert(Direction::Left, " \\_".to_string());
        neighbors.insert(Direction::Bottom, "#".to_string());
        neighbors.insert(Direction::Right, " /_".to_string());
        (value, neighbors)
    }

    fn hill_rock() -> (char, HashMap<Direction, String>) {
        let value = '#';
        let mut neighbors = HashMap::with_capacity(Direction::COUNT);
        neighbors.insert(Direction::Top, Self::HILLPIECES.to_string());
        neighbors.insert(Direction::Left, "/#".to_string());
        neighbors.insert(Direction::Bottom, " _#".to_string());
        neighbors.insert(Direction::Right, "#\\".to_string());
        (value, neighbors)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Tile {
    pub value: String,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            value: SymbolMap::ALLPIECES.to_string(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.len() != 1 {
            write!(f, "{}", self.value.len())
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl Tile {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn prune(&mut self, options: &str) {
        let combo: String = self
            .value
            .chars()
            .filter(|c| options.contains(&c.to_string()))
            .collect();
        self.value = combo;
        assert!(
            self.value.len() != 0,
            "Tile cannot have less than 1 char after pruning"
        );
    }

    fn pruned_opts(&self, options: &str) -> String {
        let combo: String = self
            .value
            .chars()
            .filter(|c| options.contains(&c.to_string()))
            .collect();
        combo
    }

    pub fn prune_with_other_in_dir(
        &mut self,
        other: &Self,
        dir: &Direction,
        symmap: &SymbolMap,
    ) -> bool {
        let mut combo = String::new();
        for c in other.value.chars() {
            let opts = symmap.get(c, dir);
            if opts.is_some() {
                combo += self.pruned_opts(opts.unwrap()).as_str();
            }
        }
        combo = combo.chars().unique().collect();
        let ret = self.value.len() != combo.len();
        self.value = combo;
        ret
    }

    pub fn get_value(&self) -> Option<char> {
        if self.value.len() == 1 {
            return self.value.chars().next();
        }
        None
    }

    pub fn is_set(&self) -> bool {
        self.value.len() == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tile_pruning() {
        let mut t = Tile::default();
        t.prune(SymbolMap::HILLPIECES);
        assert_eq!(t.value, SymbolMap::HILLPIECES.to_string());
        let symmap = SymbolMap::new();
        t.value = "#".to_string();
        let mut t_top = Tile::default();
        t_top.prune_with_other_in_dir(&t, &Direction::Top, &symmap);
        assert_eq!(t_top.value, SymbolMap::HILLPIECES.to_string())
    }

    #[test]
    fn tile_pruning_with_other() {
        let symmap = SymbolMap::new();
        let t = Tile::new("#".to_string());
        let mut t_top = Tile::default();
        t_top.prune_with_other_in_dir(&t, &Direction::Top, &symmap);
        assert_eq!(t_top.value, SymbolMap::HILLPIECES.to_string())
    }
}

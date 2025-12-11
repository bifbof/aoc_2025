use std::ops::{Deref, DerefMut};

pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

const N3X3: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl<T: Copy> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data.first().map_or(0, Vec::len);
        Self { rows, cols, data }
    }

    pub fn get(&self, (row, col): (usize, usize)) -> Option<T> {
        self.data.get(row).and_then(|r| r.get(col)).copied()
    }

    pub fn neighbors(&self, (row, col): (usize, usize)) -> impl Iterator<Item = T> {
        N3X3.into_iter().filter_map(move |(r, c)| {
            let r = row.checked_add_signed(r)?;
            let c = col.checked_add_signed(c)?;
            self.get((r, c))
        })
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn cols(&self) -> usize {
        self.cols
    }
}

impl<T> Deref for Grid<T> {
    type Target = [Vec<T>];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

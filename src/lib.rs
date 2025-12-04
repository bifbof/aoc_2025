use std::ops::{Deref, DerefMut};

use itertools::iproduct;

pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data.first().map_or(0, |row| row.len());
        Grid { rows, cols, data }
    }

    pub fn get(&self, (row, col): (usize, usize)) -> Option<T> {
        self.data.get(row).and_then(|r| r.get(col)).copied()
    }

    pub fn neighbors(&self, (row, col): (usize, usize)) -> impl Iterator<Item = T> {
        iproduct!(-1..=1, -1..=1)
            .filter(|&(r, c)| !(r == 0 && c == 0))
            .filter_map(move |(r, c)| {
                let r = row.checked_add_signed(r)?;
                let c = col.checked_add_signed(c)?;
                self.get((r, c))
            })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
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

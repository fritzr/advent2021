use std::error::Error;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    data: Vec<T>,
    shape: (usize, usize), // (rows, cols)
}

#[allow(dead_code)]
impl<T> Vec2d<T> {
    /// Create a `Vec2d<T>` using a generator function.
    pub fn generate<F>(rows: usize, cols: usize, f: F) -> Self
        where F: Fn(usize, usize) -> T
    {
        Vec2d {
            data: (0..rows*cols).map(|index| f(index / cols, index % cols)).collect(),
            shape: (rows, cols)
        }
    }

    pub fn shape(&self) -> (usize, usize) { self.shape }
    pub fn nrows(&self) -> usize { self.shape.0 }
    pub fn ncols(&self) -> usize { self.shape.1 }
    pub fn len(&self) -> usize { self.data.len() }

    fn _index(&self, row: usize, col: usize) -> usize {
        row * self.shape.1 + col
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn rows(&self) -> std::slice::Chunks<'_, T> {
        self.data.chunks(self.shape.1)
    }

    pub fn rows_mut(&mut self) -> std::slice::ChunksMut<'_, T> {
        self.data.chunks_mut(self.shape.1)
    }

    pub fn reshape(&mut self, rows: usize, cols: usize) -> Result<(), Box<dyn Error>> {
        if rows * cols != self.data.len() {
            Err(format!("cannot reshape {} elements to ({}, {})", self.len(), rows, cols).into())
        } else {
            self.shape = (rows, cols);
            Ok(())
        }
    }

    pub fn reshaped(mut self, rows: usize, cols: usize) -> Result<Self, Box<dyn Error>> {
        self.reshape(rows, cols)?;
        Ok(self)
    }

    pub fn reshaped_from<F>(self, f: F) -> Result<Self, Box<dyn Error>>
        where F: FnOnce(usize, usize) -> (usize, usize)
    {
        let new_shape = f(self.shape.0, self.shape.1);
        self.reshaped(new_shape.0, new_shape.1)
    }

    pub fn at(&self, index: (usize, usize)) -> Option<&T> {
        if index.0 < self.nrows() && index.1 < self.ncols() {
            Some(&self[index])
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
        if index.0 < self.nrows() && index.1 < self.ncols() {
            Some(&mut self[index])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.data[self._index(index.0, index.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self._index(index.0, index.1);
        &mut self.data[index]
    }
}

impl<T> FromIterator<T> for Vec2d<T> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        // Assume 1xN, caller must reshape.
        let data = Vec::<T>::from_iter(iter);
        let len = data.len();
        Self { data, shape: (1, len) }
    }
}

impl<T> Display for Vec2d<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (row_index, row) in self.rows().enumerate() {
            write!(f, "{:>3} |", row_index)?;
            for value in row {
                write!(f, " {:2}", value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

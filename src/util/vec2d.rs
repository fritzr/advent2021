use std::error::Error;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut, Range};
use std::fmt::{Display, Formatter};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Vec2d<T=i32> {
    data: Vec<T>,
    shape: (usize, usize), // (rows, cols)
}

#[allow(dead_code)]
impl<T> Vec2d<T> {
    pub fn from<U>(value: U, shape: (usize, usize)) -> Self
        where Vec<T>: FromIterator<U>, U: Copy
    {
        Self { data: std::iter::repeat(value).take(shape.0 * shape.1).collect(), shape }
    }

    /// Create a `Vec2d<T>` using a generator function.
    pub fn generate<F>(rows: usize, cols: usize, f: F) -> Self
        where F: Fn(usize, usize) -> T
    {
        Vec2d {
            data: (0..rows*cols).map(|index| f(index / cols, index % cols)).collect(),
            shape: (rows, cols)
        }
    }

    /// Iterate over 2D indexes in a "box".
    ///
    /// For example:
    ///
    /// assert_eq!(
    ///     Vec2d::iter_range((1,1)..(3,3)).collect(),
    ///     vec![(1,1), (1,2), (2,1), (2,2)]
    /// )
    pub fn enumerate_box(&self, range: Range<(usize, usize)>)
        -> impl Iterator<Item = (usize, usize)>
    {
        (min(range.start.0, self.nrows())..min(range.end.0, self.ncols())).flat_map(move |row| {
            (range.start.1..range.end.1).map(move |col| (row, col))
        })
    }

    pub fn indexes(&self) -> impl Iterator<Item = (usize, usize)> {
        self.enumerate_box((0, 0)..self.shape)
    }

    /// Iterate over 2D indexes in sequence.
    ///
    /// For example:
    pub fn enumerate_range(&self, range: Range<(usize, usize)>)
        -> impl Iterator<Item = (usize, usize)>
    {
        let shape = self.shape;
        (self._index(range.start)..self._index(range.end)).map(move |index| {
            (index / shape.0, index % shape.1)
        })
    }

    pub fn shape(&self) -> (usize, usize) { self.shape }
    pub fn nrows(&self) -> usize { self.shape.0 }
    pub fn ncols(&self) -> usize { self.shape.1 }
    pub fn len(&self) -> usize { self.data.len() }

    fn _index(&self, (row, col): (usize, usize)) -> usize {
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

    pub fn reshape(&mut self, (rows, cols): (usize, usize)) -> Result<(), Box<dyn Error>> {
        if rows * cols != self.data.len() {
            Err(format!("cannot reshape {} elements to ({}, {})", self.len(), rows, cols).into())
        } else {
            self.shape = (rows, cols);
            Ok(())
        }
    }

    pub fn reshaped(mut self, shape: (usize, usize)) -> Result<Self, Box<dyn Error>> {
        self.reshape(shape)?;
        Ok(self)
    }

    pub fn reshaped_from<F>(self, f: F) -> Result<Self, Box<dyn Error>>
        where F: FnOnce((usize, usize)) -> (usize, usize)
    {
        let new_shape = f((self.shape.0, self.shape.1));
        self.reshaped(new_shape)
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
        &self.data[self._index(index)]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self._index(index);
        &mut self.data[index]
    }
}

/// Create Vec2d from an iterator.
///
/// Enables code like:
///
///   let v: Vec2d<u8> = (0..16).collect().reshaped(4, 4);
///
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

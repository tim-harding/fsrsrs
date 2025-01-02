use std::ops::{Index, IndexMut};

use crate::Rating::{self, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cards<T: Copy> {
    pub again: T,
    pub hard: T,
    pub good: T,
    pub easy: T,
}

impl<T: Copy> Cards<T> {
    pub fn new(again: T, hard: T, good: T, easy: T) -> Self {
        Self {
            again,
            hard,
            good,
            easy,
        }
    }

    pub fn splat(t: T) -> Self {
        Self::new(t, t, t, t)
    }

    pub fn from_fn(f: impl Fn(Rating) -> T) -> Self {
        Self::new(f(Again), f(Hard), f(Good), f(Easy))
    }

    pub fn into_array(self) -> [T; 4] {
        [self.again, self.hard, self.good, self.easy]
    }

    pub fn as_array(&self) -> [&T; 4] {
        [&self.again, &self.hard, &self.good, &self.easy]
    }

    pub fn as_array_mut(&mut self) -> [&mut T; 4] {
        [
            &mut self.again,
            &mut self.hard,
            &mut self.good,
            &mut self.easy,
        ]
    }

    pub fn map<R: Copy>(self, f: impl Fn((Rating, T)) -> R) -> Cards<R> {
        Cards::new(
            f((Again, self.again)),
            f((Hard, self.hard)),
            f((Good, self.good)),
            f((Easy, self.easy)),
        )
    }

    pub fn update(&mut self, f: impl Fn((Rating, &mut T))) {
        f((Again, &mut self.again));
        f((Hard, &mut self.hard));
        f((Good, &mut self.good));
        f((Easy, &mut self.easy));
    }
}

impl<T: Copy> IndexMut<usize> for Cards<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut()[index]
    }
}

impl<T: Copy> IndexMut<Rating> for Cards<T> {
    fn index_mut(&mut self, index: Rating) -> &mut Self::Output {
        &mut self[index as usize - 1]
    }
}

impl<T: Copy> Index<usize> for Cards<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_array()[index]
    }
}

impl<T: Copy> Index<Rating> for Cards<T> {
    type Output = T;

    fn index(&self, index: Rating) -> &Self::Output {
        &self[index as usize - 1]
    }
}

impl<T: Copy> IntoIterator for Cards<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<'a, T: Copy> IntoIterator for &'a Cards<T> {
    type Item = &'a T;

    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array().into_iter()
    }
}

impl<'a, T: Copy> IntoIterator for &'a mut Cards<T> {
    type Item = &'a mut T;

    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array_mut().into_iter()
    }
}

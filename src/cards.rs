use std::ops::{Index, IndexMut};

use crate::Rating::{self, *};

pub struct Cards<T: Copy>([T; 4]);

impl<T: Copy> Cards<T> {
    pub fn new(again: T, hard: T, good: T, easy: T) -> Self {
        Self([again, hard, good, easy])
    }

    pub fn splat(t: T) -> Self {
        Self([t; 4])
    }

    pub fn from_fn(f: impl Fn(Rating) -> T) -> Self {
        Self([f(Again), f(Hard), f(Good), f(Easy)])
    }

    pub fn into_array(self) -> [T; 4] {
        self.0
    }

    pub fn as_array(&self) -> [&T; 4] {
        self.0.each_ref()
    }

    pub fn as_array_mut(&mut self) -> [&mut T; 4] {
        self.0.each_mut()
    }

    pub fn again(self) -> T {
        self[Again]
    }

    pub fn hard(self) -> T {
        self[Hard]
    }

    pub fn good(self) -> T {
        self[Good]
    }

    pub fn easy(self) -> T {
        self[Easy]
    }

    pub fn map(self, f: impl Fn((Rating, T)) -> T) -> Self {
        Self::new(
            f((Again, self.0[0])),
            f((Hard, self.0[1])),
            f((Good, self.0[2])),
            f((Easy, self.0[3])),
        )
    }

    pub fn update(&mut self, f: impl Fn((Rating, &mut T))) {
        f((Again, &mut self.0[0]));
        f((Hard, &mut self.0[1]));
        f((Good, &mut self.0[2]));
        f((Easy, &mut self.0[3]));
    }
}

impl<T: Copy> IndexMut<usize> for Cards<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Copy> IndexMut<Rating> for Cards<T> {
    fn index_mut(&mut self, index: Rating) -> &mut Self::Output {
        &mut self.0[index as usize - 1]
    }
}

impl<T: Copy> Index<usize> for Cards<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
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

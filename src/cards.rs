use crate::Rating::{self, *};

pub struct Cards<T: Copy> {
    pub again: T,
    pub hard: T,
    pub good: T,
    pub easy: T,
}

impl<T: Copy> Cards<T> {
    pub fn new(t: T) -> Self {
        Self {
            again: t,
            hard: t,
            good: t,
            easy: t,
        }
    }

    pub fn from_fn(f: impl Fn(Rating) -> T) -> Self {
        Self {
            again: f(Again),
            hard: f(Hard),
            good: f(Good),
            easy: f(Easy),
        }
    }

    pub fn get(self, rating: Rating) -> T {
        match rating {
            Again => self.again,
            Hard => self.hard,
            Good => self.good,
            Easy => self.easy,
        }
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

    pub fn map(self, f: impl Fn((Rating, T)) -> T) -> Self {
        Self {
            again: f((Again, self.again)),
            hard: f((Hard, self.hard)),
            good: f((Good, self.good)),
            easy: f((Easy, self.easy)),
        }
    }

    pub fn update(&mut self, f: impl Fn((Rating, &mut T))) {
        f((Again, &mut self.again));
        f((Hard, &mut self.hard));
        f((Good, &mut self.good));
        f((Easy, &mut self.easy));
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

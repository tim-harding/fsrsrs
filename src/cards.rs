use crate::{
    Card,
    Rating::{self, *},
};

pub struct Cards {
    pub again: Card,
    pub hard: Card,
    pub good: Card,
    pub easy: Card,
}

impl Cards {
    pub fn new(card: Card) -> Self {
        Self {
            again: card,
            hard: card,
            good: card,
            easy: card,
        }
    }

    pub fn get(self, rating: Rating) -> Card {
        match rating {
            Again => self.again,
            Hard => self.hard,
            Good => self.good,
            Easy => self.easy,
        }
    }

    pub fn into_array(self) -> [Card; 4] {
        [self.again, self.hard, self.good, self.easy]
    }

    pub fn as_array_ref(&self) -> [&Card; 4] {
        [&self.again, &self.hard, &self.good, &self.easy]
    }

    pub fn as_array_mut(&mut self) -> [&mut Card; 4] {
        [
            &mut self.again,
            &mut self.hard,
            &mut self.good,
            &mut self.easy,
        ]
    }

    pub fn map(self, f: impl Fn((Rating, Card)) -> Card) -> Self {
        Self {
            again: f((Again, self.again)),
            hard: f((Hard, self.hard)),
            good: f((Good, self.good)),
            easy: f((Easy, self.easy)),
        }
    }

    pub fn update(&mut self, f: impl Fn((Rating, &mut Card))) {
        f((Again, &mut self.again));
        f((Hard, &mut self.hard));
        f((Good, &mut self.good));
        f((Easy, &mut self.easy));
    }
}

impl IntoIterator for Cards {
    type Item = Card;

    type IntoIter = std::array::IntoIter<Card, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<'a> IntoIterator for &'a Cards {
    type Item = &'a Card;

    type IntoIter = std::array::IntoIter<&'a Card, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array_ref().into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Cards {
    type Item = &'a mut Card;

    type IntoIter = std::array::IntoIter<&'a mut Card, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array_mut().into_iter()
    }
}

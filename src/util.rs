#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn from_xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn from_row_col(row: usize, col: usize) -> Self {
        Self::from_xy(col, row)
    }
    pub fn random(max_x: usize, max_y: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..max_x),
            y: rng.gen_range(0..max_y),
        }
    }
    pub fn iter_2d(max_x: usize, max_y: usize) -> PositionIterator {
        PositionIterator::new(max_x, max_y)
    }
    pub fn iter_around(&self, radius: usize) -> AroundIterator {
        AroundIterator::new(*self, radius)
    }
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone)]
pub struct PositionIterator {
    current: Position,
    max_x: usize,
    max_y: usize,
}

impl PositionIterator {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        Self {
            current: Position::zero(),
            max_x,
            max_y,
        }
    }
}

impl std::iter::Iterator for PositionIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.max_y {
            return None;
        }
        let ret = self.current;
        self.current.x += 1;
        if self.current.x >= self.max_x {
            self.current.x = 0;
            self.current.y += 1;
        }
        Some(ret)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_in_row = self.max_x - self.current.x;
        let rows_remaining = self.max_y - self.current.y - 1;
        let total_remaining = remaining_in_row + (self.max_x * rows_remaining);
        (total_remaining, Some(total_remaining))
    }
    fn count(self) -> usize {
        self.size_hint().0
    }
    fn last(self) -> Option<Self::Item> {
        Some(Self::Item::from_xy(self.max_x - 1, self.max_y - 1))
    }
}

#[derive(Clone)]
pub struct AroundIterator {
    current: Position,
    center: Position,
    radius: usize,
}

impl AroundIterator {
    pub fn new(center: Position, radius: usize) -> Self {
        let current = Position::from_xy(
            center.x.saturating_sub(radius),
            center.y.saturating_sub(radius),
        );
        Self {
            center,
            radius,
            current,
        }
    }
}

impl std::iter::Iterator for AroundIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y > self.center.y + self.radius {
            return None;
        }
        let ret = self.current;
        self.current.x += 1;
        if self.current.x > self.center.x + self.radius {
            self.current.x = self.center.x.saturating_sub(self.radius);
            self.current.y += 1;
        }
        if ret == self.center {
            self.next()
        } else {
            Some(ret)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Position;
    #[test]
    fn constructors() {
        assert_eq!(Position::from_row_col(3, 2), Position::from_xy(2, 3));
    }
    #[test]
    fn iter_2d() {
        let iter = Position::iter_2d(3, 3);
        assert_eq!(iter.size_hint(), (9, Some(9)));
        let visited: Vec<_> = iter.collect();
        assert_eq!(
            &visited,
            &[
                Position::from_xy(0, 0),
                Position::from_xy(1, 0),
                Position::from_xy(2, 0),
                Position::from_xy(0, 1),
                Position::from_xy(1, 1),
                Position::from_xy(2, 1),
                Position::from_xy(0, 2),
                Position::from_xy(1, 2),
                Position::from_xy(2, 2),
            ]
        );
        assert_eq!(
            Position::iter_2d(7, 5).last(),
            Some(Position::from_xy(7 - 1, 5 - 1)),
        );
    }
    #[test]
    fn iter_around() {
        let visited: Vec<_> = Position::from_xy(2, 2).iter_around(1).collect();
        assert_eq!(
            &visited,
            &[
                Position::from_xy(1, 1),
                Position::from_xy(2, 1),
                Position::from_xy(3, 1),
                Position::from_xy(1, 2),
                // not (2, 2)
                Position::from_xy(3, 2),
                Position::from_xy(1, 3),
                Position::from_xy(2, 3),
                Position::from_xy(3, 3),
            ]
        );
    }
}

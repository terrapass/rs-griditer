use crate::Coord;

//
// pub struct PerimeterIter
//

/// Iterator that yields points at the perimeter of a rect, clockwise.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct PerimeterIter<C> {
    left:    C,
    top:     C,
    width:   C,
    height:  C,
    current_point: Option<(C, C)>
}

impl<C> Iterator for PerimeterIter<C>
    where C: Coord
{
    type Item = (C, C);

    fn next(&mut self) -> Option<Self::Item> {
        self.current_point.map(|current_point_copy| {
            let next_point = self.next_point(current_point_copy);

            self.current_point = if !self.is_initial_point(next_point) {
                Some(next_point)
            } else {
                None
            };

            current_point_copy
        })
    }
}

impl<C> PerimeterIter<C>
    where C: Coord
{
    //
    // Interface
    //

    pub fn with_dimensions((left, top): (C, C), (width, height): (C, C)) -> Self {
        assert!(width >= C::ZERO && height >= C::ZERO, "width and height must be non-negative");

        let is_empty = width == C::ZERO || height == C::ZERO;

        Self{
            left,
            top,
            width,
            height,
            current_point: if !is_empty { Some((left, top)) } else { None }
        }
    }

    pub fn with_corners((left, top): (C, C), (right, bottom): (C, C)) -> Self {
        assert!(left <= right && top <= bottom);

        Self{
            left,
            top,
            width:  right - left + C::ONE,
            height: bottom - top + C::ONE,
            current_point: Some((left, top))
        }
    }

    //
    // Service
    //

    fn is_initial_point(&self, (x, y): (C, C)) -> bool {
        x == self.left && y == self.top
    }

    fn next_point(&self, current_point: (C, C)) -> (C, C) {
        assert!(self.width > C::ZERO && self.height > C::ZERO, "assuming non-empty rect at this point");

        assert!(
            current_point.0 == self.left
                || current_point.0 == self.left + self.width - C::ONE
                || current_point.1 == self.top
                || current_point.1 == self.top + self.height - C::ONE,
            "current point must always be on perimeter"
        );

        self.try_next_point_degenerate(current_point)
            .unwrap_or_else(|| self.next_point_regular(current_point))
    }

    fn try_next_point_degenerate(&self, current_point: (C, C)) -> Option<(C, C)> {
        if self.width == C::ONE { // Single column case
            assert_eq!(current_point.0, self.left);

            Some((self.left, self.top + ((current_point.1 - self.top + C::ONE) % self.height)))
        } else if self.height == C::ONE { // Single row case
            assert_eq!(current_point.1, self.top);

            Some((self.left + ((current_point.0 - self.left + C::ONE) % self.width), self.top))
        } else { // Non-degenerate case, must be handled in a regular manner
            None
        }
    }

    fn next_point_regular(&self, current_point: (C, C)) -> (C, C) {
        assert!(self.width  > C::ONE, "regular case assumes more than one column");
        assert!(self.height > C::ONE, "regular case assumes more than one row");

        let right  = self.left + self.width - C::ONE;
        let bottom = self.top + self.height - C::ONE;

        let mut point = current_point;

        if point.0 > self.left && point.0 < right {
            if point.1 == self.top {
                point.0 += C::ONE; // Top side, go right
            } else {
                point.0 -= C::ONE; // Bottom side, go left
            }
        } else if point.1 > self.top && point.1 < bottom {
            if point.0 == self.left {
                point.1 -= C::ONE; // Left side, go up
            } else {
                point.1 += C::ONE; // Right side, go down
            }
        } else if point.0 == self.left && point.1 == self.top {
            point.0 += C::ONE; // Top left corner, go right
        } else if point.0 == right && point.1 == self.top {
            point.1 += C::ONE; // Top right corner, go down
        } else if point.0 == right && point.1 == bottom {
            point.0 -= C::ONE; // Bottom right corner, go left
        } else {
            assert!(point.0 == self.left && point.1 == bottom, "all options must have been exhausted");

            point.1 -= C::ONE; // Bottom left corner, go up
        }

        point
    }
}

//
// Unit tests
//

#[cfg(test)]
mod tests {
    use super::{
        super::test_utils::assert_expected_sequence,
        *
    };

    #[test]
    fn always_none_for_empty() {
        let mut it_0 = PerimeterIter::with_dimensions((8, -20), (0, 0));
        let mut it_1 = PerimeterIter::with_dimensions((-5, 91), (9, 0));
        let mut it_2 = PerimeterIter::with_dimensions((88, 72), (0, 34));

        for _ in 0..10 {
            assert!(it_0.next().is_none());
            assert!(it_1.next().is_none());
            assert!(it_2.next().is_none());
        }
    }

    #[test]
    fn only_initial_for_unit() {
        let mut it = PerimeterIter::with_corners((5, 9), (5, 9));

        let initial_point = it.next()
            .expect("expected PerimeterIter to return a point on first iteration");

        assert_eq!(initial_point.0, 5);
        assert_eq!(initial_point.1, 9);

        for _ in 0..10 {
            assert!(it.next().is_none());
        }
    }

    #[test]
    fn row() {
        let mut it = PerimeterIter::with_dimensions((9u32, 1u32), (5u32, 1u32));

        let expected_coords = [(9, 1), (10, 1), (11, 1), (12, 1), (13, 1)];

        assert_expected_sequence(&mut it, &expected_coords);
    }

    #[test]
    fn column() {
        let mut it = PerimeterIter::with_corners((5i8, 20i8), (5i8, 25i8));

        let expected_coords = [(5, 20), (5, 21), (5, 22), (5, 23), (5, 24), (5, 25)];

        assert_expected_sequence(&mut it, &expected_coords);
    }

    #[test]
    fn rect() {
        let mut it = PerimeterIter::with_dimensions((0, 0), (2, 3));

        let expected_coords = [(0, 0), (1, 0), (1, 1), (1, 2), (0, 2), (0, 1)];

        assert_expected_sequence(&mut it, &expected_coords);
    }

    #[test]
    fn only_perimeter_points() {
        let left = -98i16;
        let top = 55i16;
        let right = 95i16;
        let bottom = 101i16;

        let it = PerimeterIter::with_corners((left, top), (right, bottom));

        for point in it {
            assert!(
                point.0 == left || point.0 == right || point.1 == top || point.1 == bottom,
                "iterator must only yield points on perimeter"
            );
        }
    }
}

use crate::Coord;

//
// pub struct BresenhamIter
//

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct BresenhamIter<C = isize>
    where C: Coord
{
    // To avoid code duplication due to symmetry, this implementation
    // internally uses coordinate names `a` and `b`, which are mapped
    // to actual coordinates depending on the slope of the line.
    // `a` represents the coordinate we iterate on, so it means `y`
    // if the line is steep (|delta x| < |delta y|), otherwise - `x`.
    // `b` represents the coordinate we calculate, so it means `x`
    // if the line is steep (|delta x| < |delta y|), otherwise - `y`.

    has_finished:  bool,
    is_line_steep: bool,
    step_a:        C::Diff,
    step_b:        f32,
    current_a:     C,
    current_b:     f32,
    end_a:         C
}

impl<C> Iterator for BresenhamIter<C>
    where C: Coord
{
    type Item = (C, C);

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_finished {
            return None;
        }

        let current_b_rounded = C::from_f32(self.current_b.round());

        let current_point = if self.is_line_steep {
            (current_b_rounded, self.current_a)
        } else {
            (self.current_a, current_b_rounded)
        };

        if self.current_a == self.end_a {
            self.has_finished = true;
        } else {
            self.current_a = C::from_diff(self.current_a.into_diff() + self.step_a);
            self.current_b += self.step_b;
        }

        Some(current_point)
    }
}

impl<C> BresenhamIter<C>
    where C: Coord
{
    //
    // Interface
    //

    pub fn new(start_point: (C, C), end_point: (C, C)) -> Self {
        let delta_x = end_point.0.into_diff() - start_point.0.into_diff();
        let delta_y = end_point.1.into_diff() - start_point.1.into_diff();

        let is_line_steep = C::abs_diff(delta_y) > C::abs_diff(delta_x);

        let (start_a, end_a, delta_a, start_b, delta_b) = if is_line_steep {
            (start_point.1, end_point.1, delta_y, start_point.0, delta_x)
        } else {
            (start_point.0, end_point.0, delta_x, start_point.1, delta_y)
        };

        let step_a = C::signum(delta_a);
        let step_b = C::diff_into_f32(delta_b) / C::diff_into_f32(C::abs_diff(delta_a));

        Self {
            has_finished: false,
            is_line_steep,
            step_a,
            step_b,
            current_a: start_a,
            current_b: start_b.into_f32(),
            end_a
        }
    }
}

//
// Unit tests
//

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::{
        super::test_utils::assert_expected_sequence,
        *
    };

    #[test]
    fn single_point() {
        test_coords_sequence(&[(4, 6)]);
    }

    #[test]
    fn horizontal_right() {
        test_coords_sequence(&[(-12, 2), (-11, 2), (-10, 2), (-9, 2), (-8, 2), (-7, 2)]);
    }

    #[test]
    fn horizontal_left() {
        test_coords_sequence(&[(20, 8), (19, 8), (18, 8), (17, 8), (16, 8)]);
    }

    #[test]
    fn vertical_up() {
        test_coords_sequence(&[(5, 4), (5, 3), (5, 2)]);
    }

    #[test]
    fn vertical_down() {
        test_coords_sequence(&[(-9, 1), (-9,2), (-9, 3)]);
    }

    #[test]
    fn diagonal_right_up() {
        test_coords_sequence(&[(1, 5), (2, 4), (3, 3), (4, 2), (5, 1)]);
    }

    #[test]
    fn diagonal_right_down() {
        test_coords_sequence(&[(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn diagonal_left_up() {
        test_coords_sequence(&[(5, 5), (4, 4), (3, 3), (2, 2), (1, 1)]);
    }

    #[test]
    fn diagonal_left_down() {
        test_coords_sequence(&[(5, 1), (4, 2), (3, 3), (2, 4), (1, 5)]);
    }

    #[test]
    fn steep_increase() {
        test_slope((2, 5), (10, 20));
    }

    #[test]
    fn steep_increase_reversed() {
        test_slope((10, 20), (2, 5));
    }

    #[test]
    fn steep_decrease() {
        test_slope((11, 30), (15, -12));
    }

    #[test]
    fn steep_decrease_reversed() {
        test_slope((15, -12), (11, 30));
    }

    #[test]
    fn gentle_increase() {
        test_slope((-20, 5), (-2, 6));
    }

    #[test]
    fn gentle_increase_reversed() {
        test_slope((-2, 6), (-20, 5));
    }

    #[test]
    fn gentle_decrease() {
        test_slope((15, 2), (30, 0));
    }

    #[test]
    fn gentle_decrease_reversed() {
        test_slope((30, 0), (15, 2));
    }

    #[test]
    #[should_panic(expected = "unsigned coord values must be small enough to be convertible to the corresponding signed type")]
    fn large_unsigned_coord_panic() {
        BresenhamIter::<u8>::new((250, 108), (0, 9));
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn large_signed_coord_diff_panic() {
        BresenhamIter::<i8>::new((-120, 5), (120, 25));
    }

    //
    // Test service
    //

    fn test_coords_sequence<C>(coords_sequence: &[(C, C)])
        where C: Coord + Debug
    {
        assert!(!coords_sequence.is_empty());

        let start_point = *coords_sequence.first().unwrap();
        let end_point = *coords_sequence.last().unwrap();

        let mut it = BresenhamIter::new(
            start_point,
            end_point
        );

        assert_expected_sequence(&mut it, &coords_sequence);
    }

    fn test_slope(start_point: (isize, isize), end_point: (isize, isize)) {
        assert!(
            start_point.0 != end_point.0 && start_point.1 != end_point.1,
            "expected endpoints for a line that is neither horizontal nor vertical"
        );

        let mut it = BresenhamIter::new(
            start_point,
            end_point
        );

        let mut last_point = it.next().expect("expected at least one point");
        println!("initial point is {:?}", last_point);
        assert_eq!(last_point, start_point);

        let delta_x = end_point.0 - start_point.0;
        let delta_y = end_point.1 - start_point.1;
        let is_line_steep = isize::abs(delta_y) > isize::abs(delta_x);

        for point in it {
            println!("last point was {:?}, current point is {:?}", last_point, point);

            let (slower_coord, last_slower_coord, slower_delta, faster_coord, last_faster_coord, faster_delta) = if is_line_steep {
                (point.0, last_point.0, delta_x, point.1, last_point.1, delta_y)
            } else {
                (point.1, last_point.1, delta_y, point.0, last_point.0, delta_x)
            };

            assert!(
                (slower_delta > 0 && slower_coord >= last_slower_coord)
                    || (slower_delta < 0 && slower_coord <= last_slower_coord),
                "coordinates must change monotonically between points"
            );

            assert_eq!(
                faster_coord - last_faster_coord,
                isize::signum(faster_delta),
                "faster changing coordinate must change monotonically and continuosly between points"
            );

            last_point = point;
        }
    }
}

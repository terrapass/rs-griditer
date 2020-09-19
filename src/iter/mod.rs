pub mod bresenham;
pub mod perimeter;

#[cfg(test)]
mod test_utils {
    use std::fmt::Debug;

    //
    // Test interface
    //

    pub fn assert_expected_sequence<It, C>(it: &mut It, expected_coords: &[(C, C)])
        where It: Iterator<Item = (C, C)>,
              C:  Copy + Debug + PartialEq<C>
    {
        for &coords in expected_coords {
            assert!(
                is_some_with_coords(it.next(), coords),
                "iterator must yield expected points in correct order"
            );
        }

        assert_eq!(it.next(), None, "iterator must not yield any extraneous points");
    }

    //
    // Test service
    //

    fn is_some_with_coords<C>(actual_point: Option<(C, C)>, expected_point: (C, C)) -> bool
        where C: Debug + PartialEq<C>
    {
        if let Some(actual_point) = actual_point {
            println!("expected Some({:?}), got Some({:?})", expected_point, actual_point);

            actual_point.0 == expected_point.0 && actual_point.1 == expected_point.1
        } else {
            println!("expected Some({:?}), got None", expected_point);

            false
        }
    }
}

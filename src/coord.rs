use std::{
    any::TypeId,
    ops::{
        Add,
        Sub
    },
    convert::TryInto
};

//
// pub trait Coord
//

pub trait Coord: Clone + Copy + Add + PartialEq + Eq {
    type Diff: Copy + Add<Output = Self::Diff> + Sub<Output = Self::Diff> + PartialOrd<Self::Diff>;

    const ZERO: Self;
    const ONE:  Self;

    fn from_f32(value: f32) -> Self;

    fn from_diff(value: Self::Diff) -> Self;

    fn into_f32(self) -> f32;

    fn into_diff(self) -> Self::Diff;

    fn diff_into_f32(value: Self::Diff) -> f32;

    fn signum(value: Self::Diff) -> Self::Diff;

    fn abs_diff(value: Self::Diff) -> Self::Diff;
}

//
// Foreign impls
//

macro_rules! impl_coord {
    ($self: ty, $diff: ty) => {
        impl Coord for $self {
            type Diff = $diff;

            const ZERO: Self = 0;
            const ONE:  Self = 1;

            fn from_f32(value: f32) -> Self {
                value as Self
            }

            fn from_diff(value: Self::Diff) -> Self {
                value.try_into()
                    .expect("expected difference between unsigned coord values to fall in range of the unsigned coord type")
            }

            fn into_f32(self) -> f32 {
                self as f32
            }

            fn into_diff(self) -> Self::Diff {
                self.try_into()
                    .unwrap_or_else(|_| {
                        assert_ne!(TypeId::of::<Self>(), TypeId::of::<Self::Diff>());

                        panic!("unsigned coord values must be small enough to be convertible to the corresponding signed type");
                    })
            }

            fn diff_into_f32(value: Self::Diff) -> f32 {
                value as f32
            }

            fn signum(value: Self::Diff) -> Self::Diff {
                <$diff>::signum(value)
            }

            fn abs_diff(value: Self::Diff) -> Self::Diff {
                <$diff>::abs(value)
            }
        }
    };
}

impl_coord!(u8, i8);
impl_coord!(i8, i8);

impl_coord!(u16, i16);
impl_coord!(i16, i16);

impl_coord!(u32, i32);
impl_coord!(i32, i32);

impl_coord!(u64, i64);
impl_coord!(i64, i64);

impl_coord!(u128, i128);
impl_coord!(i128, i128);

impl_coord!(usize, isize);
impl_coord!(isize, isize);

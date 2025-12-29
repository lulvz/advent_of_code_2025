use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
pub mod utils {
    use std::array;

    use super::*;

    pub fn read_input(filename: &str) -> io::Result<String> {
        let path = Path::new("./inputs/").join(filename);
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents.to_string())
    }

    pub trait Scalar:
        Sized
        + Default
        + Copy
        + Clone
        + Debug
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
    {
    }

    // this can be interpreted as: implement the trait Scalar for all the types (T) that ipmlement
    // all those other traits down there
    impl<T> Scalar for T where
        T: Sized
            + Default
            + Copy
            + Clone
            + Debug
            + Add<Output = Self>
            + Sub<Output = Self>
            + Mul<Output = Self>
            + Div<Output = Self>
            + AddAssign
            + SubAssign
            + MulAssign
            + DivAssign
    {
    }

    // ////////////// VECTOR STRUCT /////////////////////
    #[derive(Debug, Copy, Clone)]
    pub struct Vector<T: Scalar, const N: usize> {
        data: [T; N],
    }

    impl<T: Scalar, const N: usize> From<[T; N]> for Vector<T, N> {
        fn from(data: [T; N]) -> Self {
            Vector { data }
        }
    }

    // ////////////// OPERATOR IMPLEMENTATIONS /////////////////////
    impl<T: Scalar, const N: usize> Add for Vector<T, N> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            array::from_fn(|i| self.data[i] + rhs.data[i]).into()
        }
    }

    impl<T: Scalar, const N: usize> AddAssign for Vector<T, N> {
        fn add_assign(&mut self, rhs: Self) {
            for i in 0..N {
                self.data[i] += rhs.data[i];
            }
        }
    }

    impl<T: Scalar, const N: usize> Sub for Vector<T, N> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            array::from_fn(|i| self.data[i] - rhs.data[i]).into()
        }
    }

    impl<T: Scalar, const N: usize> SubAssign for Vector<T, N> {
        fn sub_assign(&mut self, rhs: Self) {
            for i in 0..N {
                self.data[i] -= rhs.data[i];
            }
        }
    }

    impl<T: Scalar, const N: usize> Mul for Vector<T, N> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            array::from_fn(|i| self.data[i] * rhs.data[i]).into()
        }
    }

    impl<T: Scalar, const N: usize> MulAssign for Vector<T, N> {
        fn mul_assign(&mut self, rhs: Self) {
            for i in 0..N {
                self.data[i] *= rhs.data[i];
            }
        }
    }

    impl<T: Scalar, const N: usize> Div for Vector<T, N> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            array::from_fn(|i| self.data[i] / rhs.data[i]).into()
        }
    }

    impl<T: Scalar, const N: usize> DivAssign for Vector<T, N> {
        fn div_assign(&mut self, rhs: Self) {
            for i in 0..N {
                self.data[i] /= rhs.data[i];
            }
        }
    }
    // ////////////// CUSTOM OPERATOR TRAITS /////////////////////
    pub trait Dot {
        type Scalar;
        fn dot(self, rhs: Self) -> Self::Scalar;
    }

    // ////////////// CUSTOM OPERATOR IMPLEMENTATIONS /////////////////////
    impl<T: Scalar, const N: usize> Dot for Vector<T, N> {
        type Scalar = T;

        fn dot(self, rhs: Self) -> Self::Scalar {
            let mut acc: Self::Scalar = T::default();
            for i in 0..N {
                acc += self.data[i] * rhs.data[i];
            }
            acc
        }
    }

    // ////////////// COORDINATE NAME TRAITS /////////////////////
    pub trait HasX {
        type Scalar;
        fn x(&self) -> &Self::Scalar;
        fn x_mut(&mut self) -> &mut Self::Scalar;
    }

    pub trait HasY {
        type Scalar;
        fn y(&self) -> &Self::Scalar;
        fn y_mut(&mut self) -> &mut Self::Scalar;
    }

    pub trait HasZ {
        type Scalar;
        fn z(&self) -> &Self::Scalar;
        fn z_mut(&mut self) -> &mut Self::Scalar;
    }

    // ////////////// COORDINATE IMPLEMENTATIONS /////////////////////
    impl<T: Scalar> HasX for Vector<T, 2> {
        type Scalar = T;
        fn x(&self) -> &Self::Scalar {
            &self.data[0]
        }
        fn x_mut(&mut self) -> &mut Self::Scalar {
            &mut self.data[0]
        }
    }

    impl<T: Scalar> HasY for Vector<T, 2> {
        type Scalar = T;
        fn y(&self) -> &Self::Scalar {
            &self.data[1]
        }
        fn y_mut(&mut self) -> &mut Self::Scalar {
            &mut self.data[1]
        }
    }

    impl<T: Scalar> HasX for Vector<T, 3> {
        type Scalar = T;
        fn x(&self) -> &Self::Scalar {
            &self.data[0]
        }
        fn x_mut(&mut self) -> &mut Self::Scalar {
            &mut self.data[0]
        }
    }

    impl<T: Scalar> HasY for Vector<T, 3> {
        type Scalar = T;
        fn y(&self) -> &Self::Scalar {
            &self.data[1]
        }
        fn y_mut(&mut self) -> &mut Self::Scalar {
            &mut self.data[1]
        }
    }

    impl<T: Scalar> HasZ for Vector<T, 3> {
        type Scalar = T;
        fn z(&self) -> &Self::Scalar {
            &self.data[2]
        }
        fn z_mut(&mut self) -> &mut Self::Scalar {
            &mut self.data[2]
        }
    }
}

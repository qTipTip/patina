#[macro_export]
macro_rules! implement_geometry_ops {
    ($t:ident) => {
        // Unary Operators (-Self)
        impl<T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Neg for $t<T> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output {
                $t(-self.0)
            }
        }

        // Binary Operators (Self + Self, Self - Self, Self * Self, Self / Self)
        impl<T: Num + CheckNan + Copy> std::ops::Add for $t<T> {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                $t(self.0 + rhs.0)
            }
        }
        impl<T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Sub for $t<T> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                $t(self.0 - rhs.0)
            }
        }
        impl<T: Num + CheckNan + Copy> std::ops::Mul for $t<T> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                $t(self.0 * rhs.0)
            }
        }
        impl<T: Num + CheckNan + Copy> std::ops::Div for $t<T> {
            type Output = Self;
            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                $t(self.0 / rhs.0)
            }
        }

        // Assignment Operators (Self += Self, Self -= Self, etc.)
        impl<T: std::ops::AddAssign> std::ops::AddAssign for $t<T> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        impl<T: std::ops::SubAssign> std::ops::SubAssign for $t<T> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
        impl<T> std::ops::MulAssign for $t<T>
        where
            Tuple3<T>: std::ops::MulAssign<Tuple3<T>> + CheckNan,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }
        impl<T> std::ops::DivAssign for $t<T>
        where
            Tuple3<T>: std::ops::DivAssign<Tuple3<T>> + CheckNan,
        {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
            }
        }

        // Scalar Operators (Self * T, Self / T)
        impl<T: Num + CheckNan + Copy> std::ops::Mul<T> for $t<T> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                $t(self.0 * rhs)
            }
        }
        impl<T: Num + CheckNan + Copy> std::ops::Div<T> for $t<T> {
            type Output = Self;
            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                $t(self.0 / rhs)
            }
        }
        impl<T: std::ops::MulAssign<T> + Copy + CheckNan> std::ops::MulAssign<T> for $t<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs;
            }
        }
        impl<T: std::ops::DivAssign<T> + Copy + CheckNan> std::ops::DivAssign<T> for $t<T> {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                self.0 /= rhs;
            }
        }
    };
}

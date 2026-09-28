#[macro_export]
macro_rules! implement_geometry_ops {
    ($t:ident, $tuple_type:ident) => {
        // Unary Operators (-Self)
        impl<T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Neg for $t<T> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output {
                $t(-self.0)
            }
        }
        impl<'a, T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Neg for &'a $t<T> {
            type Output = $t<T>;
            #[inline]
            fn neg(self) -> Self::Output {
                $t(-self.0)
            }
        }

        // 1. Value + Value
        impl<T: Num + CheckNan + Copy> std::ops::Add for $t<T> {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                $t(self.0 + rhs.0)
            }
        }
        // 2. Ref + Ref
        impl<'a, 'b, T: Num + CheckNan + Copy> std::ops::Add<&'b $t<T>> for &'a $t<T> {
            type Output = $t<T>;
            #[inline]
            fn add(self, rhs: &'b $t<T>) -> Self::Output {
                $t(self.0 + rhs.0)
            }
        }
        // 3. Value + Ref
        impl<'b, T: Num + CheckNan + Copy> std::ops::Add<&'b $t<T>> for $t<T> {
            type Output = Self;
            #[inline]
            fn add(self, rhs: &'b $t<T>) -> Self::Output {
                $t(self.0 + rhs.0)
            }
        }
        // 4. Ref + Value
        impl<'a, T: Num + CheckNan + Copy> std::ops::Add<$t<T>> for &'a $t<T> {
            type Output = $t<T>;
            #[inline]
            fn add(self, rhs: $t<T>) -> Self::Output {
                $t(self.0 + rhs.0)
            }
        }

        // 1. Value - Value
        impl<T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Sub for $t<T> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                $t(self.0 - rhs.0)
            }
        }
        // 2. Ref - Ref
        impl<'a, 'b, T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Sub<&'b $t<T>>
            for &'a $t<T>
        {
            type Output = $t<T>;
            #[inline]
            fn sub(self, rhs: &'b $t<T>) -> Self::Output {
                $t(self.0 - rhs.0)
            }
        }
        // 3. Value - Ref
        impl<'b, T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Sub<&'b $t<T>>
            for $t<T>
        {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: &'b $t<T>) -> Self::Output {
                $t(self.0 - rhs.0)
            }
        }
        // 4. Ref - Value
        impl<'a, T: Num + std::ops::Neg<Output = T> + CheckNan + Copy> std::ops::Sub<$t<T>>
            for &'a $t<T>
        {
            type Output = $t<T>;
            #[inline]
            fn sub(self, rhs: $t<T>) -> Self::Output {
                $t(self.0 - rhs.0)
            }
        }

        // Binary Operators (Mul, Div)
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
            $tuple_type<T>: std::ops::MulAssign<$tuple_type<T>> + CheckNan,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }
        impl<T> std::ops::DivAssign for $t<T>
        where
            $tuple_type<T>: std::ops::DivAssign<$tuple_type<T>> + CheckNan,
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

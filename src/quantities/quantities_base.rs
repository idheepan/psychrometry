#[macro_export]
macro_rules! NewQuantity {
    ($quantity:ident, $units:ident, $tolerance:expr) => {
        #[derive(Debug)]
        pub struct $quantity<T: $units> {
            base_unit: i64,
            unit: PhantomData<T>,
        }

        macro_rules! ImplQuantityFromNumber {
            ($N:ty) => {
                impl<T> From<$N> for $quantity<T>
                where
                    T: $units,
                {
                    fn from(n: $N) -> Self {
                        $quantity {
                            base_unit: (n as f64 * T::conv_factor_base_unit() as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> From<$quantity<T>> for $N
                where
                    T: $units,
                {
                    fn from(t: $quantity<T>) -> $N {
                        (t.base_unit as f64 / (T::conv_factor_base_unit() as f64)) as $N
                    }
                }
            };
        }

        macro_rules! ImplOpsForNumber {
            ($N:ty) => {
                impl<T> ops::Add<$N> for $quantity<T>
                where
                    T: $units,
                {
                    type Output = Self;
                    fn add(self, rhs: $N) -> Self::Output {
                        $quantity {
                            base_unit: self.base_unit
                                + (rhs as f64 * T::conv_factor_base_unit() as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> ops::Add<$quantity<T>> for $N
                where
                    T: $units,
                {
                    type Output = $quantity<T>;
                    fn add(self, rhs: $quantity<T>) -> Self::Output {
                        $quantity {
                            base_unit: rhs.base_unit
                                + (self as f64 * T::conv_factor_base_unit() as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> ops::Sub<$N> for $quantity<T>
                where
                    T: $units,
                {
                    type Output = Self;
                    fn sub(self, rhs: $N) -> Self::Output {
                        $quantity {
                            base_unit: self.base_unit
                                - (rhs as f64 * T::conv_factor_base_unit() as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> ops::Mul<$N> for $quantity<T>
                where
                    T: $units,
                {
                    type Output = Self;
                    fn mul(self, rhs: $N) -> Self::Output {
                        $quantity {
                            base_unit: (rhs as f64 * self.base_unit as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> ops::Mul<$quantity<T>> for $N
                where
                    T: $units,
                {
                    type Output = $quantity<T>;
                    fn mul(self, rhs: $quantity<T>) -> Self::Output {
                        $quantity {
                            base_unit: (self as f64 * rhs.base_unit as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> ops::Div<$N> for $quantity<T>
                where
                    T: $units,
                {
                    type Output = Self;
                    fn div(self, rhs: $N) -> Self::Output {
                        $quantity {
                            base_unit: ((self.base_unit as f64) / rhs as f64) as i64,
                            unit: PhantomData,
                        }
                    }
                }

                impl<T> ops::Div<$quantity<T>> for $N
                where
                    T: $units,
                {
                    type Output = $N;
                    fn div(self, rhs: $quantity<T>) -> Self::Output {
                        ((rhs.base_unit as f64) / (T::conv_factor_base_unit() as f64 * self as f64))
                            as $N
                    }
                }
            };
        }

        impl<'a, T1, T2> From<&'a $quantity<T1>> for $quantity<T2>
        where
            T1: $units,
            T2: $units,
        {
            fn from(t1: &'a $quantity<T1>) -> Self {
                $quantity {
                    base_unit: (t1.base_unit),
                    unit: (PhantomData),
                }
            }
        }

        impl<T1, T2> PartialEq<$quantity<T1>> for $quantity<T2>
        where
            T1: $units,
            T2: $units,
        {
            fn eq(&self, other: &$quantity<T1>) -> bool {
                (self.base_unit - other.base_unit).abs() < $tolerance
            }
        }

        ImplQuantityFromNumber!(i64);
        ImplQuantityFromNumber!(f64);
        ImplOpsForNumber!(f64);
        ImplOpsForNumber!(i64);
    };
}

use core::cmp;
use core::marker::PhantomData;
use core::ops;

const TEMP_TOLERANCE: i64 = 200; //Microkelvins
#[derive(Debug)]
pub struct Temperature<T: crate::units::TemperatureUnit> {
    micro_kelvin: i64,
    unit: PhantomData<T>,
}
macro_rules! ImplTemperatureFromNumber {
    ($N:ty) => {
        impl<T> From<$N> for Temperature<T>
        where
            T: crate::units::TemperatureUnit,
        {
            fn from(n: $N) -> Self {
                Temperature {
                    micro_kelvin: (n as f64 * T::conv_factor_micro_kelvin() as f64
                        + T::conv_offset_micro_kelvin() as f64)
                        as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> From<Temperature<T>> for $N
        where
            T: crate::units::TemperatureUnit,
        {
            fn from(t: Temperature<T>) -> $N {
                (((t.micro_kelvin - T::conv_offset_micro_kelvin()) as f64)
                    / (T::conv_factor_micro_kelvin() as f64)) as $N
            }
        }
    };
}
ImplTemperatureFromNumber!(i64);
ImplTemperatureFromNumber!(f64);

impl<'a, T1, T2> From<&'a Temperature<T1>> for Temperature<T2>
where
    T1: crate::units::TemperatureUnit,
    T2: crate::units::TemperatureUnit,
{
    fn from(t1: &'a Temperature<T1>) -> Self {
        Temperature {
            micro_kelvin: (t1.micro_kelvin),
            unit: (PhantomData),
        }
    }
}

macro_rules! ImplOpsForNumber {
    ($N:ty) => {
        impl<T> ops::Add<$N> for Temperature<T>
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = Self;
            fn add(self, rhs: $N) -> Self::Output {
                Temperature {
                    micro_kelvin: self.micro_kelvin
                        + (rhs as f64 * T::conv_factor_micro_kelvin() as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Add<Temperature<T>> for $N
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = Temperature<T>;
            fn add(self, rhs: Temperature<T>) -> Self::Output {
                Temperature {
                    micro_kelvin: rhs.micro_kelvin
                        + (self as f64 * T::conv_factor_micro_kelvin() as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Sub<$N> for Temperature<T>
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = Self;
            fn sub(self, rhs: $N) -> Self::Output {
                Temperature {
                    micro_kelvin: self.micro_kelvin
                        - (rhs as f64 * T::conv_factor_micro_kelvin() as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Mul<$N> for Temperature<T>
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = Self;
            fn mul(self, rhs: $N) -> Self::Output {
                Temperature {
                    micro_kelvin: (rhs as f64 * self.micro_kelvin as f64
                        + (1.0 - rhs as f64) * T::conv_offset_micro_kelvin() as f64)
                        as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Mul<Temperature<T>> for $N
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = Temperature<T>;
            fn mul(self, rhs: Temperature<T>) -> Self::Output {
                Temperature {
                    micro_kelvin: (self as f64 * rhs.micro_kelvin as f64
                        + (1.0 - self as f64) * T::conv_offset_micro_kelvin() as f64)
                        as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Div<$N> for Temperature<T>
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = Self;
            fn div(self, rhs: $N) -> Self::Output {
                Temperature {
                    micro_kelvin: ((self.micro_kelvin as f64
                        + (rhs as f64 - 1.0) * T::conv_offset_micro_kelvin() as f64)
                        / rhs as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Div<Temperature<T>> for $N
        where
            T: crate::units::TemperatureUnit,
        {
            type Output = $N;
            fn div(self, rhs: Temperature<T>) -> Self::Output {
                (((rhs.micro_kelvin - T::conv_offset_micro_kelvin()) as f64)
                    / (T::conv_factor_micro_kelvin() as f64 * self as f64)) as $N
            }
        }
    };
}
ImplOpsForNumber!(f64);
ImplOpsForNumber!(i64);
impl<T> PartialEq for Temperature<T>
where
    T: crate::units::TemperatureUnit,
{
    fn eq(&self, other: &Self) -> bool {
        (self.micro_kelvin - other.micro_kelvin).abs() < TEMP_TOLERANCE
    }
}
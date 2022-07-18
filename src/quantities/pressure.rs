use core::cmp;
use core::marker::PhantomData;
use core::ops;

const PRES_TOLERANCE: i64 = 200; //millipascals
#[derive(Debug)]
pub struct Pressure<T: crate::units::PressureUnit> {
    milli_pascal: i64,
    unit: PhantomData<T>,
}

macro_rules! ImplPressureFromNumber {
    ($N:ty) => {
        impl<T> From<$N> for Pressure<T>
        where
            T: crate::units::PressureUnit,
        {
            fn from(n: $N) -> Self {
                Pressure {
                    milli_pascal: (n as f64 * T::conv_factor_milli_pascal() as f64)
                        as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> From<Pressure<T>> for $N
        where
            T: crate::units::PressureUnit,
        {
            fn from(t: Pressure<T>) -> $N {
                (t.milli_pascal as f64
                    / (T::conv_factor_milli_pascal() as f64)) as $N
            }
        }
    };
}

ImplPressureFromNumber!(i64);
ImplPressureFromNumber!(f64);

impl<'a, T1, T2> From<&'a Pressure<T1>> for Pressure<T2>
where
    T1: crate::units::PressureUnit,
    T2: crate::units::PressureUnit,
{
    fn from(t1: &'a Pressure<T1>) -> Self {
        Pressure {
            milli_pascal: (t1.milli_pascal),
            unit: (PhantomData),
        }
    }
}

macro_rules! ImplOpsForNumber {
    ($N:ty) => {
        impl<T> ops::Add<$N> for Pressure<T>
        where
            T: crate::units::PressureUnit,
        {
            type Output = Self;
            fn add(self, rhs: $N) -> Self::Output {
                Pressure {
                    milli_pascal: self.milli_pascal
                        + (rhs as f64 * T::conv_factor_milli_pascal() as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Add<Pressure<T>> for $N
        where
            T: crate::units::PressureUnit,
        {
            type Output = Pressure<T>;
            fn add(self, rhs: Pressure<T>) -> Self::Output {
                Pressure {
                    milli_pascal: rhs.milli_pascal
                        + (self as f64 * T::conv_factor_milli_pascal() as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Sub<$N> for Pressure<T>
        where
            T: crate::units::PressureUnit,
        {
            type Output = Self;
            fn sub(self, rhs: $N) -> Self::Output {
                Pressure {
                    milli_pascal: self.milli_pascal
                        - (rhs as f64 * T::conv_factor_milli_pascal() as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Mul<$N> for Pressure<T>
        where
            T: crate::units::PressureUnit,
        {
            type Output = Self;
            fn mul(self, rhs: $N) -> Self::Output {
                Pressure {
                    milli_pascal: (rhs as f64 * self.milli_pascal as f64)
                        as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Mul<Pressure<T>> for $N
        where
            T: crate::units::PressureUnit,
        {
            type Output = Pressure<T>;
            fn mul(self, rhs: Pressure<T>) -> Self::Output {
                Pressure {
                    milli_pascal: (self as f64 * rhs.milli_pascal as f64)
                        as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Div<$N> for Pressure<T>
        where
            T: crate::units::PressureUnit,
        {
            type Output = Self;
            fn div(self, rhs: $N) -> Self::Output {
                Pressure {
                    milli_pascal: ((self.milli_pascal as f64)
                        / rhs as f64) as i64,
                    unit: PhantomData,
                }
            }
        }

        impl<T> ops::Div<Pressure<T>> for $N
        where
            T: crate::units::PressureUnit,
        {
            type Output = $N;
            fn div(self, rhs: Pressure<T>) -> Self::Output {
                ((rhs.milli_pascal as f64)
                    / (T::conv_factor_milli_pascal() as f64 * self as f64)) as $N
            }
        }
    };
}

ImplOpsForNumber!(f64);
ImplOpsForNumber!(i64);
impl<T1, T2> PartialEq<Pressure<T1>> for Pressure<T2>
where
    T1: crate::units::PressureUnit,
    T2: crate::units::PressureUnit
{
    fn eq(&self, other: &Pressure<T1>) -> bool {
        (self.milli_pascal - other.milli_pascal).abs() < PRES_TOLERANCE
    }
}

#[cfg(test)]
mod pressure_tests {
    use super::*;
    use crate::units::{Atmosphere, Pascal, Psi};
    #[test]
    fn create(){
        let a = 1.2; //atm
        let b = 121590; //Pa
        let c = 17.6351385; //psi
        let pa = Pressure::<Atmosphere>::from(a);
        let pb = Pressure::<Pascal>::from(b);
        let pc = Pressure::<Psi>::from(c);
        assert_eq!(pa, pc);
        assert_eq!(f64::from(pa), a);
        assert_eq!(pb, pc);
    }
}
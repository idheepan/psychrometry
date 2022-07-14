//! Psychrometry is derived from PsychroLib <https://github.com/psychrometrics/psychrolib>.
//!
//! This library should make it easy to integrate temperature and humidity sensors with
//! your rust based dashboards. Versions of PsychroLib for other languages are available
//! from the above repository. The names are as close to the original as possible. The one
//! major difference is that the function calls in this library is in snake_case while the original
//! repository uses CamelCase. This library will update when it merges upstream.
//!
//! # Quick Start
//! The following example lets you get the enthalpy of moist air with dry bulb temperature
//! and relative humidty.

//! # Functions implemented so far
//! - `get_trankine_from_tfahrenheit`
//! - `get_tfahrenheit_from_trankine`
//! - `get_tkelvin_from_tcelsius`
//! - `get_tcelsius_from_tkelvin`
//! - `get_sat_vap_pres`
//! - `get_moist_air_enthalpy`
//! - `get_vap_pres_from_hum_ratio`
//! - `get_rel_hum_from_vap_pres`
//! - `get_vap_pres_from_rel_hum`
//! - `get_hum_ratio_from_vap_pres`
//! - `get_hum_ratio_from_rel_hum`
//!
//!
#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Fix documentation formating for units with underscore
//TODO: Documentation for Result errors. The pedantic warning can be enabled after that.
// Too many false positives for now.
#![warn(clippy::pedantic)]
#![warn(clippy::must_use_candidate)]
// #![warn(missing_docs)]
#![allow(unused)]

// TODO: Implement display and formatting
// TODO: Implement standard math operations.
/// Funtions for psychrometric calculations.

pub mod quantities {
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
}
pub mod units {

    pub trait TemperatureUnit {
        fn singular_name() -> String;
        fn abbreviation() -> String;
        fn conv_factor_micro_kelvin() -> i64;
        fn conv_offset_micro_kelvin() -> i64;
    }

    macro_rules! NewTemperatureUnit {
        ($unit_name:ident, $singular_name:expr, $abbreviation:expr, $conv_factor:expr, $conv_offset:expr) => {
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct $unit_name;

            impl TemperatureUnit for $unit_name {
                #[inline(always)]
                fn singular_name() -> String {
                    $singular_name.to_string()
                }
                #[inline(always)]
                fn abbreviation() -> String {
                    $abbreviation.to_string()
                }
                #[inline(always)]
                fn conv_factor_micro_kelvin() -> i64 {
                    $conv_factor
                }
                #[inline(always)]
                fn conv_offset_micro_kelvin() -> i64 {
                    $conv_offset
                }
            }
        };
    }
    NewTemperatureUnit!(Kelvin, "kelvin", "K", 1_000_000, 0);
    NewTemperatureUnit!(Celcius, "celcius", "C", 1_000_000, 273_150_000);
    NewTemperatureUnit!(
        Fahrenheit,
        "fahrenheit",
        "F",
        (1_000_000.0 / 1.8) as i64,
        (459_670_000.0 / 1.8) as i64
    );
}

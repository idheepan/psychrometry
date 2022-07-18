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
    (1_000_000.0_f32 / 1.8_f32) as i64,
    (459_670_000.0 / 1.8) as i64
);

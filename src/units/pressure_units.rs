pub trait PressureUnit {
    fn singular_name() -> String;
    fn abbreviation() -> String;
    fn conv_factor_milli_pascal() -> i64;
}

macro_rules! NewPressureUnit {
    ($unit_name:ident, $singular_name:expr, $abbreviation:expr, $conv_factor:expr) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $unit_name;

        impl PressureUnit for $unit_name {
            #[inline(always)]
            fn singular_name() -> String {
                $singular_name.to_string()
            }
            #[inline(always)]
            fn abbreviation() -> String {
                $abbreviation.to_string()
            }
            #[inline(always)]
            fn conv_factor_milli_pascal() -> i64 {
                $conv_factor
            }
        }
    };
}
NewPressureUnit!(Pascal, "pascal", "Pa", 1_000);
NewPressureUnit!(Atmosphere, "atmosphere", "atm", 101_325_000);
NewPressureUnit!(Psi, "psi", "psi", 6_894_760);
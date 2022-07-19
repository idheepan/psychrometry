#[macro_export]
macro_rules! NewUnitType {
    ($unit_type:ident) => {
        pub trait $unit_type {
            fn singular_name() -> String;
            fn abbreviation() -> String;
            fn conv_factor_base_unit() -> i64;
        }
    };
}

#[macro_export]
macro_rules! NewUnit {
    ($unit_type:ident, $unit_name:ident, $singular_name:expr, $abbreviation:expr, $conv_factor:expr) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $unit_name;

        impl $unit_type for $unit_name {
            #[inline(always)]
            fn singular_name() -> String {
                $singular_name.to_string()
            }
            #[inline(always)]
            fn abbreviation() -> String {
                $abbreviation.to_string()
            }
            #[inline(always)]
            fn conv_factor_base_unit() -> i64 {
                $conv_factor
            }
        }
    };
}

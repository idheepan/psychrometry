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
//! ```
//! use psychrometry::psychrolib;
//! let mut psychrolib = psychrolib::SI {};
//! let rel_hum = 0.25_f64; //Relative humidity from 0 to 1
//! let t_dry_bulb = 30_f64; //Dry bulb temperature in Celcius for SI
//! let pres_ambient = 101325_f64; //Ambient pressure in Pa for SI
//! let hum_ratio = psychrolib.get_hum_ratio_from_rel_hum(t_dry_bulb, rel_hum, pres_ambient).unwrap();
//! let enth = psychrolib.get_moist_air_enthalpy(t_dry_bulb, hum_ratio).unwrap();
//! assert_eq!(47015.61,
//! (enth*100.0).trunc()/100.0); //Truncating to two decimal points.
//! ```
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
// #![warn(clippy::pedantic)]
// #![warn(clippy::must_use_candidate)]
#![warn(missing_docs)]
#![allow(unused)]
//TODO: Feature to select SI unit library or IP or both. Compile only what is necessary.
//TODO: Is a class even necessary just to track unit system?

/// Funtions for psychrometric calculations.
pub mod psychrolib;
use psychrolib::*;
#[cfg(test)]
/// Module for testing the straightforward functions of the library
mod tests {
    use super::*;

    #[test]
    /// Simple tests. Compared with psychrolib packages
    fn get_sat_vap_pres_normal() {
        let mut psychrolib = psychrolib::SI {};
        assert_eq!(
            259.9028,
            (psychrolib.get_sat_vap_pres(-10.0).unwrap_or(0.0) * 1E4).trunc() / 1E4
        );
        let mut psychrolib = psychrolib::IP {};
        assert_eq!(
            0.0108,
            (psychrolib.get_sat_vap_pres(-10.0).unwrap_or(0.0) * 1E4).trunc() / 1E4
        );
    }
    #[test]
    fn get_moist_air_enthalpy_normal() {
        let mut psychrolib = psychrolib::SI {};
        assert_eq!(
            55748.0,
            (psychrolib
                .get_moist_air_enthalpy(30.0, 0.010)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
        let mut psychrolib = psychrolib::IP {};
        assert_eq!(
            17.9431,
            (psychrolib
                .get_moist_air_enthalpy(30.0, 0.010)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
    }

    #[test]
    fn get_vap_pres_from_hum_ratio_normal() {
        let psychrolib = psychrolib::SI {};
        assert_eq!(
            808.0852,
            (psychrolib
                .get_vap_pres_from_hum_ratio(0.005, 101325.0)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
        let psychrolib = psychrolib::IP {};
        assert_eq!(
            0.1172,
            (psychrolib
                .get_vap_pres_from_hum_ratio(0.005, 14.6959488)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
    }

    #[test]
    fn get_rel_hum_from_vap_pres_normal() {
        let mut psychrolib = psychrolib::SI {};
        assert_eq!(
            0.3420,
            (psychrolib
                .get_rel_hum_from_vap_pres(20.0, 800.0)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
    }

    #[test]
    fn get_vap_pres_from_rel_hum_normal() {
        let mut psychrolib = psychrolib::SI {};
        assert_eq!(
            935.5214,
            (psychrolib
                .get_vap_pres_from_rel_hum(20.0, 0.4)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
    }

    #[test]
    fn get_hum_ratio_from_vap_pres_normal() {
        let mut psychrolib = psychrolib::SI {};
        assert_eq!(
            0.0055,
            (psychrolib
                .get_hum_ratio_from_vap_pres(900.0, 101325.0)
                .unwrap_or(0.0)
                * 1E4)
                .trunc()
                / 1E4
        );
    }

    #[test]
    fn get_hum_ratio_from_rel_hum_normal() {
        let mut psychrolib = psychrolib::SI;
        assert_eq!(
            0.007165,
            (psychrolib
                .get_hum_ratio_from_rel_hum(24.870, 0.36699, 101325.0)
                .unwrap_or(0.0)
                * 1E6)
                .trunc()
                / 1E6
        );
    }
}

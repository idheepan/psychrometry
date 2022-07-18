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
// Too many false positives for now. Especially for cast_possible_truncation
// #![warn(clippy::pedantic)]
#![warn(clippy::must_use_candidate)]
// #![warn(missing_docs)]
#![allow(unused)]

// TODO: Implement display and formatting for various quantities
// TODO: Implement pressure, relative humidity, humidity ratio, specific enthalpy
/// Funtions for psychrometric calculations.
pub mod quantities;
pub mod units;

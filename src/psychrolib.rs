#[derive(Default)]
/// Supports to unit systems. This will soon deprecated in favor of a trait type
pub enum UnitSystem {
    /// Imperial Units
    IP,
    /// SI Units (default)
    #[default]
    SI,
}

// TODO: Add docs for function returning Errors.
/******************************************************************************************************
 * Global constants
 *****************************************************************************************************/

const ZERO_FAHRENHEIT_AS_RANKINE: f64 = 459.67; // Zero degree Fahrenheit (°F) expressed as degree Rankine (°R).
                                                // Reference: ASHRAE Handbook - Fundamentals (2017) ch. 39.

const ZERO_CELSIUS_AS_KELVIN: f64 = 273.15; // Zero degree Celsius (°C) expressed as Kelvin (K).
                                            // Reference: ASHRAE Handbook - Fundamentals (2017) ch. 39.

// const R_DA_IP: f64 = 53.350; // Universal gas constant for dry air (IP version) in ft∙lbf/lb_da/R.
// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1.

// const R_DA_SI: f64 = 287.042; // Universal gas constant for dry air (SI version) in J/kg_da/K.
// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1.

const INVALID: f64 = -99999.99; // Invalid value.

// const MAX_ITER_COUNT: i32 = 100; // Maximum number of iterations before exiting while loops.

const MIN_HUM_RATIO: f64 = 1e-7; // Minimum acceptable humidity ratio used/returned by any functions.
                                 // Any value above 0 or below the MIN_HUM_RATIO will be reset to this value.

// const FREEZING_POINT_WATER_IP: f64 = 32.0; // Freezing point of water in Fahrenheit.

// const FREEZING_POINT_WATER_SI: f64 = 0.0; // Freezing point of water in Celsius.

const TRIPLE_POINT_WATER_IP: f64 = 32.018; // Triple point of water in Fahrenheit.

const TRIPLE_POINT_WATER_SI: f64 = 0.01; // Triple point of water in Celsius.

#[derive(Debug)]
/// All types of errors possible within psychrometry crate.
pub enum PsychroLibErr<'a> {
    /// When one of the values in param is not valid
    ValueError(&'a str),
    /// When one of the values in params is not within acceptable limits.
    RangeError(&'a str),
    /// When the solution doesn't converge for given conditions.
    ConvergenceError(&'a str),
}

// TODO: Implement a fmt::Display trait for PsychroLibErr

/// A simple access to Psychrometry functions. Creates a default with SI Units and 0.001 tolerance
pub struct Psychrolib {
    /// SI Units by default. Not sure if this should be in here. Will be removed once we start using traits.
    pub unit_system: UnitSystem,
    /// During iterative solver, this is the solver exit critiera
    pub tolerance: f32,
}

/// By default the units system for Psychrolib is SI and tolerance is 0.001
impl Default for Psychrolib {
    fn default() -> Self {
        Psychrolib {
            unit_system: UnitSystem::SI,
            tolerance: 0.001,
        }
    }
}
impl Psychrolib {
    /// The parameter `units` will be set for future calculations.  
    /// The tolerance is the same in IP and SI
    pub fn set_units(&mut self, units: UnitSystem) -> &mut Self {
        if matches!(units, UnitSystem::IP) {
            self.tolerance = 0.001 * 9.0 / 5.0;
        } else {
            self.tolerance = 0.001;
        }
        self.unit_system = units;
        self
    }
}

impl Psychrolib {
    /******************************************************************************************************
     * Conversion between temperature units
     *****************************************************************************************************/
    /// Utility function to convert temperature to degree Rankine (°R)
    /// given temperature in degree Fahrenheit (°F).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    pub fn get_trankine_from_tfahrenheit(t_f: f64) -> f64 {
        t_f + ZERO_FAHRENHEIT_AS_RANKINE
    } /* exact */
    /// Utility function to convert temperature to degree Fahrenheit (°F)
    /// given temperature in degree Rankine (°R).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    pub fn get_tfahrenheit_from_trankine(t_r: f64) -> f64 {
        t_r - ZERO_FAHRENHEIT_AS_RANKINE
    } /* exact */
    /// Utility function to convert temperature to Kelvin (K)
    /// given temperature in degree Celsius (°C).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    pub fn get_tkelvin_from_tcelsius(t_c: f64) -> f64 {
        t_c + ZERO_CELSIUS_AS_KELVIN
    } /* exact */
    /// Utility function to convert temperature to degree Celsius (°C)
    /// given temperature in Kelvin (K).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    pub fn get_tcelsius_from_tkelvin(t_k: f64) -> f64 {
        t_k - ZERO_CELSIUS_AS_KELVIN
    } /* exact */

    /// Return saturation vapor pressure given dry-bulb temperature.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 5 & 6
    /// Important note: the ASHRAE formulae are defined above and below the freezing point but have
    /// a discontinuity at the freezing point. This is a small inaccuracy on ASHRAE's part: the formulae
    /// should be defined above and below the triple point of water (not the feezing point) in which case
    /// the discontinuity vanishes. It is essential to use the triple point of water otherwise function
    /// `get_tdew_point_from_vap_pres`, which inverts the present function, does not converge properly around
    /// the freezing point.
    pub fn get_sat_vap_pres(
        &self,
        tdry_bulb: f64, // (i) Dry bulb temperature in °F [IP] or °C [SI]
    ) -> Result<f64, PsychroLibErr> {
        let ln_pws: f64;
        let t: f64;

        if matches!(self.unit_system, UnitSystem::IP) {
            if !(-148.0..=392.0).contains(&tdry_bulb) {
                return Err(PsychroLibErr::RangeError(
                    "Dry bulb temperature is outside range -148 to 392F",
                ));
            }

            t = Psychrolib::get_trankine_from_tfahrenheit(tdry_bulb);

            if tdry_bulb <= TRIPLE_POINT_WATER_IP {
                ln_pws = -1.0214165E+04 / t - 4.8932428 - 5.3765794E-03 * t
                    + 1.9202377E-07 * t * t
                    + 3.5575832E-10 * t.powi(3)
                    - 9.0344688E-14 * t.powi(4)
                    + 4.1635019 * t.ln();
            } else {
                ln_pws = -1.0440397E+04 / t - 1.1294650E+01 - 2.7022355E-02 * t
                    + 1.2890360E-05 * t * t
                    - 2.4780681E-09 * t.powi(3)
                    + 6.5459673 * t.ln();
            }
        } else {
            if !(-100.0..=200.0).contains(&tdry_bulb) {
                return Err(PsychroLibErr::RangeError(
                    "Dry bulb temperature is outside range -100 to 200C",
                ));
            }

            t = Psychrolib::get_tkelvin_from_tcelsius(tdry_bulb);

            if tdry_bulb <= TRIPLE_POINT_WATER_SI {
                ln_pws = -5.6745359E+03 / t + 6.3925247 - 9.677843E-03 * t
                    + 6.2215701E-07 * t * t
                    + 2.0747825E-09 * t.powi(3)
                    - 9.484024E-13 * t.powi(4)
                    + 4.1635019 * t.ln();
            } else {
                ln_pws = -5.8002206E+03 / t + 1.3914993 - 4.8640239E-02 * t + 4.1764768E-05 * t * t
                    - 1.4452093E-08 * t.powi(3)
                    + 6.5459673 * t.ln();
            }
        }

        Ok(ln_pws.exp())
    }

    /// Return moist air enthalpy given dry-bulb temperature and humidity ratio.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 30
    /// `tdry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `hum_ratio` Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    /// Returns Moist air enthalpy in J Kg_Air⁻¹
    pub fn get_moist_air_enthalpy(
        &mut self,
        tdry_bulb: f64, // (i) Dry bulb temperature in °F [IP] or °C [SI]
        hum_ratio: f64, // (i) Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    ) -> Result<f64, PsychroLibErr> {
        if hum_ratio <= 0.0 {
            return Err(PsychroLibErr::RangeError("Humidity ratio is negative"));
        }

        let bounded_hum_ratio: f64 = hum_ratio.max(MIN_HUM_RATIO);

        if matches!(self.unit_system, UnitSystem::IP) {
            Ok(0.240 * tdry_bulb + bounded_hum_ratio * (1061.0 + 0.444 * tdry_bulb))
        } else {
            Ok((1.006 * tdry_bulb + bounded_hum_ratio * (2501.0 + 1.86 * tdry_bulb)) * 1000.0)
        }
    }

    /// Return vapor pressure given humidity ratio and pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 20 solved for pw
    /// Returns: Partial pressure of water vapor in moist air in Psi [IP] or Pa [SI]
    /// `hum_ratio` Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    /// `pressure` Atmospheric pressure in Psi [IP] or Pa [SI]
    pub fn get_vap_pres_from_hum_ratio(
        &self,
        hum_ratio: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr<'static>> {
        if hum_ratio <= 0.0 {
            return Err(PsychroLibErr::ValueError("Humidity ratio is negative"));
        }

        let bounded_hum_ratio: f64 = hum_ratio.max(MIN_HUM_RATIO);

        Ok(pressure * bounded_hum_ratio / (0.621945 + bounded_hum_ratio))
    }
    /// Return relative humidity given dry-bulb temperature and vapor pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 12, 22
    /// Returns: Relative humidity [0-1]
    /// `t_dry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `vap_pres` Partial pressure of water vapor in moist air in Psi [IP] or Pa [SI]
    pub fn get_rel_hum_from_vap_pres(
        &mut self,
        tdry_bulb: f64,
        vap_pres: f64,
    ) -> Result<f64, PsychroLibErr> {
        if vap_pres <= 0.0 {
            return Err(PsychroLibErr::ValueError(
                "Partial pressure of water vapor in moist air is negative",
            ));
        }
        let sat_vap_pres = self.get_sat_vap_pres(tdry_bulb)?;
        Ok(vap_pres / sat_vap_pres)
    }

    /// Return partial pressure of water vapor as a function of relative humidity and temperature.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 12, 22
    /// Partial pressure of water vapor in moist air in Psi [IP] or Pa [SI]
    /// `tdry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `rel_hum` Relative humidity [0-1]
    pub fn get_vap_pres_from_rel_hum(
        &self,
        tdry_bulb: f64,
        rel_hum: f64,
    ) -> Result<f64, PsychroLibErr> {
        if (0.0..=1.0).contains(&rel_hum) {
            let sat_vap_pres = self.get_sat_vap_pres(tdry_bulb)?;
            Ok(rel_hum * sat_vap_pres)
        } else {
            return Err(PsychroLibErr::RangeError(
                "Relative humidity should be between 0 and 1",
            ));
        }
    }

    /// Return humidity ratio given water vapor pressure and atmospheric pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 20
    /// Returns Humidity Ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    pub fn get_hum_ratio_from_vap_pres(
        &self,
        vap_pres: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr> {
        if vap_pres <= 0.0 {
            return Err(PsychroLibErr::ValueError(
                "Partial pressure of water vapor in moist air is negative",
            ));
        }
        let hum_ratio: f64 = 0.621945 * vap_pres / (pressure - vap_pres);
        Ok(hum_ratio.max(MIN_HUM_RATIO))
    }

    /// Return humidity ratio given dry-bulb temperature, relative humidity, and pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1
    /// Returns: Humidity Ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    /// `tdry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `rel_hum` Relative humidity [0-1]
    /// `pressure`  Atmospheric pressure in Psi [IP] or Pa [SI]
    pub fn get_hum_ratio_from_rel_hum(
        &mut self,
        tdry_bulb: f64,
        rel_hum: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr> {
        if !(0.0..=1.0).contains(&rel_hum) {
            return Err(PsychroLibErr::RangeError(
                "Relative humidity should be between 0 and 1",
            ));
        }
        let vap_pres = self.get_vap_pres_from_rel_hum(tdry_bulb, rel_hum)?;
        let hum_ratio = self.get_hum_ratio_from_vap_pres(vap_pres, pressure)?;
        Ok(hum_ratio)
    }
}

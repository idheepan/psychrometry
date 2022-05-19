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
    Value(&'a str),
    /// When one of the values in params is not within acceptable limits.
    Range(&'a str),
    /// When the solution doesn't converge for given conditions.
    Convergence(&'a str),
}

/// For psychrometric caculations that will be performed in SI
pub struct SI;
/// For psychrometric caculations that will be performed in Imperial
pub struct IP;

/// Functions for calculations that are dependent on the unit system is implemented in this trait
pub trait PsychrometryFunctions {
    /// Return saturation vapor pressure given dry-bulb temperature.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 5 & 6
    /// Important note: the ASHRAE formulae are defined above and below the freezing point but have
    /// a discontinuity at the freezing point. This is a small inaccuracy on ASHRAE's part: the formulae
    /// should be defined above and below the triple point of water (not the feezing point) in which case
    /// the discontinuity vanishes. It is essential to use the triple point of water otherwise function
    /// `get_tdew_point_from_vap_pres`, which inverts the present function, does not converge properly around
    /// the freezing point.
    /// Returns: Vapor Pressure of saturated air in Psi [IP] or Pa [SI]
    /// `tdry_bulb` in Dry bulb temperature in °F [IP] or °C [SI]
    fn get_sat_vap_pres(&self, tdry_bulb: f64) -> Result<f64, PsychroLibErr>;

    /// Return moist air enthalpy given dry-bulb temperature and humidity ratio.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 30
    /// `tdry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `hum_ratio` Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    /// Returns Moist air enthalpy in J Kg_Air⁻¹
    fn get_moist_air_enthalpy(&self, tdry_bulb: f64, hum_ratio: f64) -> Result<f64, PsychroLibErr>;
}

/// All psychrometric functions for calculations that are NOT dependent on the unit system is implemented in this trait
pub trait UnitIndependent {
    /// Return vapor pressure given humidity ratio and pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 20 solved for pw
    /// Returns: Partial pressure of water vapor in moist air in Psi [IP] or Pa [SI]
    /// `hum_ratio` Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    /// `pressure` Atmospheric pressure in Psi [IP] or Pa [SI]
    fn get_vap_pres_from_hum_ratio(
        &self,
        hum_ratio: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr>;

    /// Return partial pressure of water vapor as a function of relative humidity and temperature.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 12, 22
    /// Partial pressure of water vapor in moist air in Psi [IP] or Pa [SI]
    /// `tdry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `rel_hum` Relative humidity [0-1]
    fn get_vap_pres_from_rel_hum(&self, tdry_bulb: f64, rel_hum: f64)
        -> Result<f64, PsychroLibErr>;

    /// Return relative humidity given dry-bulb temperature and vapor pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 12, 22
    /// Returns: Relative humidity [0-1]
    /// `t_dry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `vap_pres` Partial pressure of water vapor in moist air in Psi [IP] or Pa [SI]
    fn get_rel_hum_from_vap_pres(
        &self,
        tdry_bulb: f64,
        vap_pres: f64,
    ) -> Result<f64, PsychroLibErr>;

    /// Return humidity ratio given water vapor pressure and atmospheric pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 20
    /// Returns Humidity Ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    fn get_hum_ratio_from_vap_pres(
        &self,
        vap_pres: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr>;

    /// Return humidity ratio given dry-bulb temperature, relative humidity, and pressure.
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1
    /// Returns: Humidity Ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    /// `tdry_bulb` Dry bulb temperature in °F [IP] or °C [SI]
    /// `rel_hum` Relative humidity [0-1]
    /// `pressure`  Atmospheric pressure in Psi [IP] or Pa [SI]
    fn get_hum_ratio_from_rel_hum(
        &self,
        tdry_bulb: f64,
        rel_hum: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr>;
}

impl IP {
    /// Utility function to convert temperature to degree Rankine (°R)
    /// given temperature in degree Fahrenheit (°F).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    fn get_trankine_from_tfahrenheit(&self, t_f: f64) -> f64 {
        t_f + ZERO_FAHRENHEIT_AS_RANKINE
    }
    /// Utility function to convert temperature to degree Fahrenheit (°F)
    /// given temperature in degree Rankine (°R).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    fn get_tfahrenheit_from_trankine(&self, t_r: f64) -> f64 {
        t_r - ZERO_FAHRENHEIT_AS_RANKINE
    }
}

impl SI {
    /// Utility function to convert temperature to Kelvin (K)
    /// given temperature in degree Celsius (°C).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    fn get_tkelvin_from_tcelsius(&self, t_c: f64) -> f64 {
        t_c + ZERO_CELSIUS_AS_KELVIN
    }
    /// Utility function to convert temperature to degree Celsius (°C)
    /// given temperature in Kelvin (K).
    /// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 section 3
    fn get_tcelsius_from_tkelvin(&self, t_k: f64) -> f64 {
        t_k - ZERO_CELSIUS_AS_KELVIN
    }
}

impl<T> UnitIndependent for T
where
    T: PsychrometryFunctions,
{
    fn get_vap_pres_from_hum_ratio(
        &self,
        hum_ratio: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr> {
        if hum_ratio <= 0.0 {
            return Err(PsychroLibErr::Value("Humidity ratio is negative"));
        }

        let bounded_hum_ratio: f64 = hum_ratio.max(MIN_HUM_RATIO);

        Ok(pressure * bounded_hum_ratio / (0.621945 + bounded_hum_ratio))
    }

    fn get_vap_pres_from_rel_hum(
        &self,
        tdry_bulb: f64,
        rel_hum: f64,
    ) -> Result<f64, PsychroLibErr> {
        if (0.0..=1.0).contains(&rel_hum) {
            let sat_vap_pres = self.get_sat_vap_pres(tdry_bulb)?;
            Ok(rel_hum * sat_vap_pres)
        } else {
            return Err(PsychroLibErr::Range(
                "Relative humidity should be between 0 and 1",
            ));
        }
    }

    fn get_rel_hum_from_vap_pres(
        &self,
        tdry_bulb: f64,
        vap_pres: f64,
    ) -> Result<f64, PsychroLibErr> {
        if vap_pres <= 0.0 {
            return Err(PsychroLibErr::Value(
                "Partial pressure of water vapor in moist air is negative",
            ));
        }
        let sat_vap_pres = self.get_sat_vap_pres(tdry_bulb)?;
        Ok(vap_pres / sat_vap_pres)
    }

    fn get_hum_ratio_from_vap_pres(
        &self,
        vap_pres: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr> {
        if vap_pres <= 0.0 {
            return Err(PsychroLibErr::Value(
                "Partial pressure of water vapor in moist air is negative",
            ));
        }
        let hum_ratio: f64 = 0.621945 * vap_pres / (pressure - vap_pres);
        Ok(hum_ratio.max(MIN_HUM_RATIO))
    }

    fn get_hum_ratio_from_rel_hum(
        &self,
        tdry_bulb: f64,
        rel_hum: f64,
        pressure: f64,
    ) -> Result<f64, PsychroLibErr> {
        if !(0.0..=1.0).contains(&rel_hum) {
            return Err(PsychroLibErr::Range(
                "Relative humidity should be between 0 and 1",
            ));
        }
        let vap_pres = self.get_vap_pres_from_rel_hum(tdry_bulb, rel_hum)?;
        let hum_ratio = self.get_hum_ratio_from_vap_pres(vap_pres, pressure)?;
        Ok(hum_ratio)
    }
}

impl PsychrometryFunctions for SI {
    fn get_sat_vap_pres(&self, tdry_bulb: f64) -> Result<f64, PsychroLibErr> {
        if !(-100.0..=200.0).contains(&tdry_bulb) {
            return Err(PsychroLibErr::Range(
                "Dry bulb temperature is outside range -100 to 200C",
            ));
        }

        let t = self.get_tkelvin_from_tcelsius(tdry_bulb);

        let ln_pws = if tdry_bulb <= TRIPLE_POINT_WATER_SI {
            -5.6745359E+03 / t + 6.3925247 - 9.677843E-03 * t
                + 6.2215701E-07 * t * t
                + 2.0747825E-09 * t.powi(3)
                - 9.484024E-13 * t.powi(4)
                + 4.1635019 * t.ln()
        } else {
            -5.8002206E+03 / t + 1.3914993 - 4.8640239E-02 * t + 4.1764768E-05 * t * t
                - 1.4452093E-08 * t.powi(3)
                + 6.5459673 * t.ln()
        };

        Ok(ln_pws.exp())
    }
    fn get_moist_air_enthalpy(
        &self,
        tdry_bulb: f64, // (i) Dry bulb temperature in °F [IP] or °C [SI]
        hum_ratio: f64, // (i) Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    ) -> Result<f64, PsychroLibErr> {
        if hum_ratio <= 0.0 {
            return Err(PsychroLibErr::Range("Humidity ratio is negative"));
        }
        let bounded_hum_ratio: f64 = hum_ratio.max(MIN_HUM_RATIO);
        Ok((1.006 * tdry_bulb + bounded_hum_ratio * (2501.0 + 1.86 * tdry_bulb)) * 1000.0)
    }
}

impl PsychrometryFunctions for IP {
    fn get_sat_vap_pres(&self, tdry_bulb: f64) -> Result<f64, PsychroLibErr> {
        if !(-148.0..=392.0).contains(&tdry_bulb) {
            return Err(PsychroLibErr::Range(
                "Dry bulb temperature is outside range -148 to 392F",
            ));
        }

        let t = self.get_trankine_from_tfahrenheit(tdry_bulb);

        let ln_pws = if tdry_bulb <= TRIPLE_POINT_WATER_IP {
            -1.0214165E+04 / t - 4.8932428 - 5.3765794E-03 * t
                + 1.9202377E-07 * t * t
                + 3.5575832E-10 * t.powi(3)
                - 9.0344688E-14 * t.powi(4)
                + 4.1635019 * t.ln()
        } else {
            -1.0440397E+04 / t - 1.1294650E+01 - 2.7022355E-02 * t + 1.2890360E-05 * t * t
                - 2.4780681E-09 * t.powi(3)
                + 6.5459673 * t.ln()
        };

        Ok(ln_pws.exp())
    }
    fn get_moist_air_enthalpy(
        &self,
        tdry_bulb: f64, // (i) Dry bulb temperature in °F [IP] or °C [SI]
        hum_ratio: f64, // (i) Humidity ratio in lb_H₂O lb_Air⁻¹ [IP] or kg_H₂O kg_Air⁻¹ [SI]
    ) -> Result<f64, PsychroLibErr> {
        if hum_ratio <= 0.0 {
            return Err(PsychroLibErr::Range("Humidity ratio is negative"));
        }

        let bounded_hum_ratio: f64 = hum_ratio.max(MIN_HUM_RATIO);
        Ok(0.240 * tdry_bulb + bounded_hum_ratio * (1061.0 + 0.444 * tdry_bulb))
    }
}

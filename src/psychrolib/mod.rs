use crate::quantities::{Pressure, SpecificEnthalpy, Temperature};
use crate::units::{Celcius, JoulesPerKg, Kelvin, Pascal};
use crate::units::{PressureUnit, SpecificEnthalpyUnit, TemperatureUnit};
// TODO: Implement in quantities a default check for temperature range -100...200 celcius
// TODO: Minimum humidity ratio should be 1E-7.
// TODO: Partial pressure cannot be negative

const TRIPLE_POINT_WATER: Temperature<Kelvin> = Temperature {
    micro_kelvin: 273_160_000,
    unit: core::marker::PhantomData,
};

#[derive(Debug)]
/// All types of errors possible within psychrometry crate.
pub enum PsychroLibErr {
    /// When one of the values in param is not valid
    Value,
    /// When one of the values in params is not within acceptable limits.
    Range,
    /// When the solution doesn't converge for given conditions.
    Convergence,
}

/// Return saturation vapor pressure given dry-bulb temperature.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 5 & 6
/// Important note: the ASHRAE formulae are defined above and below the freezing point but have
/// a discontinuity at the freezing point. This is a small inaccuracy on ASHRAE's part: the formulae
/// should be defined above and below the triple point of water (not the feezing point) in which case
/// the discontinuity vanishes. It is essential to use the triple point of water otherwise function
/// `get_tdew_point_from_vap_pres`, which inverts the present function, does not converge properly around
/// the freezing point.
/// Returns: Vapor Pressure of saturated air in Psi  or Pa  or atm
/// `tdry_bulb` in Dry bulb temperature in °F  or °C  or K
pub fn get_sat_vap_pres<T, P>(tdry_bulb: Temperature<T>) -> Result<Pressure<P>, PsychroLibErr>
where
    T: TemperatureUnit,
    P: PressureUnit,
{
    let tdry_k = Temperature::<Kelvin>::from(&tdry_bulb);
    let t_k = f64::from(&tdry_k);

    let ln_pws = if (tdry_k <= TRIPLE_POINT_WATER) {
        -5.6745359E+03 / t_k + 6.3925247 - 9.677843E-03 * t_k
            + 6.2215701E-07 * t_k * t_k
            + 2.0747825E-09 * t_k.powi(3)
            - 9.484024E-13 * t_k.powi(4)
            + 4.1635019 * t_k.ln()
    } else {
        -5.8002206E+03 / t_k + 1.3914993 - 4.8640239E-02 * t_k + 4.1764768E-05 * t_k * t_k
            - 1.4452093E-08 * t_k.powi(3)
            + 6.5459673 * t_k.ln()
    };
    let sat_vap_pres = Pressure::<Pascal>::from(ln_pws.exp());
    Ok(Pressure::<P>::from(&sat_vap_pres))
}

fn enthalpy_in_jpkg(tdcf: f64, hum_ratio: f64) -> SpecificEnthalpy<JoulesPerKg> {
    let ejpkgf = (1.006 * tdcf + hum_ratio * (2501. + 1.86 * tdcf)) * 1000.0;
    SpecificEnthalpy::<JoulesPerKg>::from(ejpkgf)
}

/// Return moist air enthalpy given dry-bulb temperature and humidity ratio.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 30
/// `tdry_bulb` Dry bulb temperature in °F  or °C or K
/// `hum_ratio` Humidity ratio in lb_H₂O lb_Air⁻¹  or kg_H₂O kg_Air⁻¹
/// Returns Moist air enthalpy in J Kg_Air⁻¹
pub fn get_moist_air_enthalpy_from_hum_ratio<T: TemperatureUnit, SPE: SpecificEnthalpyUnit>(
    tdry_bulb: Temperature<T>,
    hum_ratio: f64,
) -> Result<SpecificEnthalpy<SPE>, PsychroLibErr> {
    let tdc = Temperature::<Celcius>::from(&tdry_bulb);
    let tdcf = f64::from(&tdc);
    let moist_air_enthalpy = enthalpy_in_jpkg(tdcf, hum_ratio);
    Ok(SpecificEnthalpy::<SPE>::from(&moist_air_enthalpy))
}

/// Return moist air enthalpy given dry-bulb temperature and relative humidity.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 30
/// `tdry_bulb` Dry bulb temperature in °F  or °C or K
/// `rel_hum` Relative humidity [0-1]
/// Returns Moist air enthalpy in J Kg_Air⁻¹
pub fn get_moist_air_enthalpy_from_rel_hum<
    T: TemperatureUnit,
    S: SpecificEnthalpyUnit,
    P: PressureUnit,
>(
    tdry_bulb: Temperature<T>,
    rel_hum: f64,
    pres_ambient: Pressure<P>,
) -> Result<SpecificEnthalpy<S>, PsychroLibErr> {
    let tdc = Temperature::<Celcius>::from(&tdry_bulb);
    let tdcf = f64::from(&tdc);
    let hum_ratio = get_hum_ratio_from_rel_hum(tdry_bulb, rel_hum, pres_ambient)?;
    let moist_air_enthalpy = enthalpy_in_jpkg(tdcf, hum_ratio);
    Ok(SpecificEnthalpy::<S>::from(&moist_air_enthalpy))
}

/// Return vapor pressure given humidity ratio and pressure.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 20 solved for pw
/// Returns: Partial pressure of water vapor in moist air in Psi  or Pa or atm
/// `hum_ratio` Humidity ratio in lb_H₂O lb_Air⁻¹  or kg_H₂O kg_Air⁻¹
/// `pressure` Atmospheric pressure in Psi  or Pa or atm
pub fn get_vap_pres_from_hum_ratio<PA: PressureUnit, PV: PressureUnit>(
    hum_ratio: f64,
    pres_ambient: Pressure<PA>,
) -> Result<Pressure<PV>, PsychroLibErr> {
    // EFFICIENCY: Is it more efficient to have Pressure unit at the end? All operations as float till the pressure?
    let vap_pres = hum_ratio / (0.621945 + hum_ratio) * pres_ambient;
    Ok(Pressure::<PV>::from(&vap_pres))
}

/// Return partial pressure of water vapor as a function of relative humidity and temperature.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 12, 22
/// Partial pressure of water vapor in moist air in Psi  or Pa or atm
/// `tdry_bulb` Dry bulb temperature in °F  or °C or K
/// `rel_hum` Relative humidity [0-1]
pub fn get_vap_pres_from_rel_hum<T: TemperatureUnit, PV: PressureUnit>(
    tdry_bulb: Temperature<T>,
    rel_hum: f64,
) -> Result<Pressure<PV>, PsychroLibErr> {
    Ok(rel_hum * get_sat_vap_pres(tdry_bulb)?)
}

/// Return relative humidity given dry-bulb temperature and vapor pressure.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 12, 22
/// Returns: Relative humidity [0-1]
/// `t_dry_bulb` Dry bulb temperature in °F  or °C or K
/// `vap_pres` Partial pressure of water vapor in moist air in Psi  or Pa or atm
pub fn get_rel_hum_from_vap_pres<T: TemperatureUnit, PV: PressureUnit>(
    tdry_bulb: Temperature<T>,
    vap_pres: Pressure<PV>,
) -> Result<f64, PsychroLibErr> {
    let sat_vap_pres: Pressure<PV> = get_sat_vap_pres(tdry_bulb)?;
    Ok(vap_pres / sat_vap_pres)
}

/// Return humidity ratio given water vapor pressure and atmospheric pressure.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 20
/// Returns Humidity Ratio in lb_H₂O lb_Air⁻¹  or kg_H₂O kg_Air⁻¹
pub fn get_hum_ratio_from_vap_pres<PV: PressureUnit, P: PressureUnit>(
    vap_pres: Pressure<PV>,
    pres_ambient: Pressure<P>,
) -> Result<f64, PsychroLibErr> {
    let pres_ambient_vp = Pressure::<PV>::from(&pres_ambient);
    let vpf = f64::from(&vap_pres);
    let apf = f64::from(&pres_ambient_vp);
    let hum_ratio = 0.621945 * vpf / (apf - vpf);
    Ok(hum_ratio)
}

/// Return humidity ratio given dry-bulb temperature, relative humidity, and pressure.
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1
/// Returns: Humidity Ratio in lb_H₂O lb_Air⁻¹  or kg_H₂O kg_Air⁻¹
/// `tdry_bulb` Dry bulb temperature in °F  or °C or K
/// `rel_hum` Relative humidity [0-1]
/// `pressure`  Atmospheric pressure in Psi  or Pa or atm
pub fn get_hum_ratio_from_rel_hum<T: TemperatureUnit, P: PressureUnit>(
    tdry_bulb: Temperature<T>,
    rel_hum: f64,
    pres_ambient: Pressure<P>,
) -> Result<f64, PsychroLibErr> {
    let vap_pres: Pressure<P> = get_vap_pres_from_rel_hum(tdry_bulb, rel_hum)?;
    let hum_ratio = get_hum_ratio_from_vap_pres(vap_pres, pres_ambient)?;

    Ok(hum_ratio)
}

mod tests {
    use crate::units::{Atmosphere, Fahrenheit, Psi};

    use super::*;

    #[test]
    /// Simple tests. Compared with psychrolib packages
    fn get_sat_vap_pres_above_triple_point() {
        let tdrybulb = Temperature::<Celcius>::from(23.525);
        let sat_pres_exp = Pressure::<Pascal>::from(2901.087);
        let sat_pres_calc: Pressure<Pascal> = get_sat_vap_pres(tdrybulb).unwrap();
        assert_eq!(sat_pres_exp, sat_pres_calc);
    }
    #[test]
    fn get_sat_vap_pres_below_triple_point() {
        let tdry_bulb = Temperature::<Celcius>::from(-8.332);
        let sat_pres_exp = Pressure::<Pascal>::from(301.104);
        let sat_pres_calc: Pressure<Pascal> = get_sat_vap_pres(tdry_bulb).unwrap();
        assert_eq!(sat_pres_exp, sat_pres_calc);
    }
    #[test]
    fn get_moist_air_enthalpy_normal() {
        use crate::units::KilojoulesPerKg;
        let tdry_bulb = Temperature::<Fahrenheit>::from(86);
        let hum_ratio = 0.010;
        let enthalpy_exp = SpecificEnthalpy::<KilojoulesPerKg>::from(55.748);
        let enthalpy_calc: SpecificEnthalpy<KilojoulesPerKg> =
            get_moist_air_enthalpy_from_hum_ratio(tdry_bulb, hum_ratio).unwrap();
        assert_eq!(enthalpy_exp, enthalpy_calc);
    }

    #[test]
    fn get_vap_pres_from_hum_ratio_normal() {
        let hum_ratio = 0.005;
        let pres_ambient = Pressure::<Atmosphere>::from(1);
        let vap_pres_exp = Pressure::<Psi>::from(0.1172028493);
        let vap_pres_calc: Pressure<Pascal> =
            get_vap_pres_from_hum_ratio(hum_ratio, pres_ambient).unwrap();
        assert_eq!(vap_pres_exp, vap_pres_calc);
    }

    #[test]
    fn get_vap_pres_from_rel_hum_normal() {
        let rel_hum = 0.54303;
        let tdry_bulb = Temperature::<Celcius>::from(18.826);
        let vap_pres_exp = Pressure::<Pascal>::from(1180.5643);
        let vap_pres_calc: Pressure<Pascal> =
            get_vap_pres_from_rel_hum(tdry_bulb, rel_hum).unwrap();
        assert_eq!(vap_pres_exp, vap_pres_calc);
    }

    #[test]
    fn get_hum_ratio_from_vap_pres_normal() {
        let vap_pres = Pressure::<Pascal>::from(2292.850);
        let pres_ambient = Pressure::<Atmosphere>::from(1);
        let hum_ratio = get_hum_ratio_from_vap_pres(vap_pres, pres_ambient).unwrap();
        assert!((hum_ratio - 0.01439).abs() < 0.0001);
    }
    #[test]
    fn get_hum_ratio_from_rel_hum_normal() {
        let tdry_bulb = Temperature::<Fahrenheit>::from(86);
        let pres_ambient = Pressure::<Psi>::from(14.6959);
        let rel_hum = 0.25;
        let hum_ratio = get_hum_ratio_from_rel_hum(tdry_bulb, rel_hum, pres_ambient).unwrap();
        assert!((hum_ratio - 0.0065).abs() < 0.0001);
    }
}

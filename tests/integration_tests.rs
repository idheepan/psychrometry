/// Test functions of the library
///
extern crate psychrometry;

#[cfg(test)]
mod integration_tests {
    use psychrometry::psychrolib::*;
    use psychrometry::quantities::{Pressure, SpecificEnthalpy, Temperature};
    use psychrometry::units::{Atmosphere, Fahrenheit, JoulesPerKg, KilojoulesPerKg};

    #[test]
    fn calculate_enthalpy() {
        let rel_hum = 0.25;
        let tdry_bulb = Temperature::<Fahrenheit>::from(86);
        let pres_ambient = Pressure::<Atmosphere>::from(1);
        let sp_enthalpy: SpecificEnthalpy<KilojoulesPerKg> =
            get_moist_air_enthalpy_from_rel_hum(tdry_bulb, rel_hum, pres_ambient).unwrap();
        let sp_enthalpy_exp = SpecificEnthalpy::<JoulesPerKg>::from(47015.61);
        assert_eq!(sp_enthalpy_exp, sp_enthalpy);
    }
}

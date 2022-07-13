/// Test functions of the library
/// 

use psychrometry::{quantities::Temperature,units::{Kelvin, Celcius}};

mod tests {
    use psychrometry::units::Fahrenheit;

    use super::*;

    #[test]
    /// Simple tests. Compared with psychrolib packages
    fn celcius_test() {
        let t = 100.1_f64;
        let t4 = Temperature::<Fahrenheit>::from(212.18_f64);
        // let t0 = Temperature::<Celcius>::from(t);
        let t2 =  Temperature::<Celcius>::from(&t4);
        // let tr = i64::from(t2);
        assert_eq!(t, f64::from(t2));
    }
}

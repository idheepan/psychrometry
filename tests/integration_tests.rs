/// Test functions of the library
/// 

use psychrometry::{quantities::Temperature,units::Celcius};

mod tests {
    use psychrometry::units::Fahrenheit;

    use super::*;

    #[test]
    /// Simple tests. Compared with psychrolib packages
    fn celcius_test() {
        let a = 100.1_f64;
        let b = 212.18_f64;
        let t0 = Temperature::<Fahrenheit>::from(b);
        let t1 = Temperature::<Celcius>::from(a);
        let t2 =  Temperature::<Fahrenheit>::from(b);
        // assert_eq!(t1, t2);

        let t4 = t2 -3;
        // let tr = i64::from(t2);
        assert_eq!(t3, t4);
    }
}

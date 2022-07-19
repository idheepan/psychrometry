use core::cmp;
use core::marker::PhantomData;
use core::ops;

use crate::units::PressureUnit;
use crate::NewQuantity;

NewQuantity!(Pressure, PressureUnit, 200);

#[cfg(test)]
mod pressure_tests {
    use super::*;
    use crate::units::{Atmosphere, Pascal, Psi};
    #[test]
    fn create() {
        let a = 1.2_f64; //atm
        let b = 121_590; //Pa
        let c = 17.635_138; //psi
        let pa = Pressure::<Atmosphere>::from(a);
        let pb = Pressure::<Pascal>::from(b);
        let pc = Pressure::<Psi>::from(c);
        assert_eq!(pa, pc);
        assert!((f64::from(pa) - a).abs() < 1E-8);
        assert_eq!(pb, pc);
    }
}

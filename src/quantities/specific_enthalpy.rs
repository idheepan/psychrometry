use crate::units::SpecificEnthalpyUnit;
use crate::NewQuantity;

use core::cmp;
use core::marker::PhantomData;
use core::ops;

NewQuantity!(SpecificEnthalpy, SpecificEnthalpyUnit, 200);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::{BtuPerPound, JoulesPerKg, KilojoulesPerKg};

    #[test]
    fn create() {
        let a = 12.4; //kj/kg
        let b = 5.331; //btu/lb
        let ea = SpecificEnthalpy::<KilojoulesPerKg>::from(a);
        let eb = SpecificEnthalpy::<BtuPerPound>::from(b);
        let c = 12400; //j/kg;
        let ec = SpecificEnthalpy::<JoulesPerKg>::from(c);
        assert_eq!(ea, eb);
        assert_eq!(eb, ec);
    }
}

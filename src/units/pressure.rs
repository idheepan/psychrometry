#[macro_use]
use crate::{NewUnitType, NewUnit};

NewUnitType!(PressureUnit);
NewUnit!(PressureUnit, Pascal, "pascal", "Pa", 1_000);
NewUnit!(PressureUnit, Atmosphere, "atmosphere", "atm", 101_325_000);
NewUnit!(PressureUnit, Psi, "psi", "psi", 6_894_760);

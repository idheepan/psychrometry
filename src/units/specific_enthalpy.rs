#[macro_use]
use crate::{NewUnitType, NewUnit};

NewUnitType!(SpecificEnthalpyUnit);
//Base units for Specific Enthalpy milliJoules/kg
NewUnit!(
    SpecificEnthalpyUnit,
    JoulesPerKg,
    "joules per kilogram",
    "j kg⁻¹",
    1_000
);
NewUnit!(
    SpecificEnthalpyUnit,
    KilojoulesPerKg,
    "kilojoules per kilogram",
    "kj kg⁻¹",
    1_000_000
);
NewUnit!(
    SpecificEnthalpyUnit,
    BtuPerPound,
    "Btu per pound",
    "Btu lb⁻¹",
    2_326_000
);

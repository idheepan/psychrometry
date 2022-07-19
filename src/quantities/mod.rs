// TODO: Implement limits to quantities. Temperature and pressure specifically has no meaning when it is negative.
// Relative humidity cannot be outside 0...1
mod quantities_base;

mod pressure;
pub use pressure::Pressure;

mod temperature;
pub use temperature::Temperature;

mod specific_enthalpy;
pub use specific_enthalpy::SpecificEnthalpy;

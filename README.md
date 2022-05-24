# Psychrometry library for Rust
## Functions to calculate thermodynamic properties of gas-vapor mixtures
Psychrometry is derived from PsychroLib <https://github.com/psychrometrics/psychrolib>.

This library should make it easy to integrate temperature and humidity sensors with
your rust based dashboards. Versions of PsychroLib for other languages are available
from the above repository. The names are as close to the original as possible. The one
major difference is that the function calls in this library is in snake_case while the original
repository uses CamelCase. This library will update when it merges upstream.

## Quick Start
The following example lets you get the enthalpy of moist air with dry bulb temperature
and relative humidty.
```
use psychrometry::psychrolib;
let mut psychrolib = psychrolib::SI {};
let rel_hum = 0.25_f64; //Relative humidity from 0 to 1
let t_dry_bulb = 30_f64; //Dry bulb temperature in Celcius for SI
let pres_ambient = 101325_f64; //Ambient pressure in Pa for SI
let hum_ratio = psychrolib.get_hum_ratio_from_rel_hum(t_dry_bulb, rel_hum, pres_ambient).unwrap();
let enth = psychrolib.get_moist_air_enthalpy(t_dry_bulb, hum_ratio).unwrap();
assert_eq!(47015.61,
(enth*100.0).trunc()/100.0); //Truncating to two decimal points.
```
## Functions implemented so far
- get_trankine_from_tfahrenheit
- get_tfahrenheit_from_trankine
- get_tkelvin_from_tcelsius
- get_tcelsius_from_tkelvin
- get_sat_vap_pres
- get_moist_air_enthalpy
- get_vap_pres_from_hum_ratio
- get_rel_hum_from_vap_pres
- get_vap_pres_from_rel_hum
- get_hum_ratio_from_vap_pres
- get_hum_ratio_from_rel_hum

For questions, issues, feature requests like compatibility with similar devices
and other changes, please file an
[issue in the github project](https://github.com/idheepan/psychrometry/issues)

## License

Licensed under

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
   
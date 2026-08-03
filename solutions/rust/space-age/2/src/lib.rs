// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


const EARTH_YEAR_SECONDS: i32 = 31_557_600;

const MERCURY_ADJUSTMENT: f64 = 0.2408467;
const VENUS_ADJUSTMENT: f64 = 0.61519726;
const MARS_ADJUSTMENT: f64 = 1.8808158;
const JUPITER_ADJUSTMENT: f64 = 11.862615;
const SATURN_ADJUSTMENT: f64 = 29.447498;
const URANUS_ADJUSTMENT: f64 = 84.016846;
const NEPTUNE_ADJUSTMENT: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    default_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            default_years: (s as f64 / EARTH_YEAR_SECONDS as f64 * 100.0).round() / 100.0
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ($planet_name:ident, $adjustment:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn years_during(d: &Duration) -> f64 {
                d.default_years / $adjustment
            }
        }
    };
}

planet!(Mercury, MERCURY_ADJUSTMENT);
planet!(Saturn, SATURN_ADJUSTMENT);
planet!(Uranus, URANUS_ADJUSTMENT);
planet!(Neptune, NEPTUNE_ADJUSTMENT);
planet!(Jupiter, JUPITER_ADJUSTMENT);
planet!(Mars, MARS_ADJUSTMENT);
planet!(Venus, VENUS_ADJUSTMENT);
planet!(Earth, 1.0);

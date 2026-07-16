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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / MERCURY_ADJUSTMENT
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / VENUS_ADJUSTMENT
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.default_years
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / MARS_ADJUSTMENT
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / JUPITER_ADJUSTMENT
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / SATURN_ADJUSTMENT
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / URANUS_ADJUSTMENT
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.default_years / NEPTUNE_ADJUSTMENT
    }
}

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

const ONE_EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (Self::period() * ONE_EARTH_YEAR_IN_SECONDS)
    }
}

macro_rules! new_planet {
    // This macro takes an argument of designator `ident` and
    // creates a structure named `$planet_name` with trait `Planet`
    // and it's function `period` using `$eriod_length
    // The `ident` designator is used for variable/function names.
    ($planet_name:ident, $period_length:literal) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn period() -> f64 {
                $period_length
            }
        }
    };
}

new_planet!(Mercury, 0.2408467);
new_planet!(Venus, 0.61519726);
new_planet!(Earth, 1.0);
new_planet!(Mars, 1.8808158);
new_planet!(Jupiter, 11.862615);
new_planet!(Saturn, 29.447498);
new_planet!(Uranus, 84.016846);
new_planet!(Neptune, 164.79132);
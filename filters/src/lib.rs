use chrono::TimeDelta;
use google_maps::prelude::*;
use std::fmt;
use std::time::Duration;
use url::Url;
pub mod filter;
pub mod interests;
pub mod maps;
pub mod good_data;

pub struct TimeDistance {
    pub travel_duration: TimeDelta,
    pub distance: Distance,
}

#[derive(Debug)]
pub struct Distance {
    pub value: f64,
    pub unit: DistanceUnit,
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl Distance {
    pub fn from_kilometers(kilometers: f64) -> Distance {
        Distance {
            value: kilometers,
            unit: DistanceUnit::Kilometer,
        }
    }

    pub fn from_miles(miles: f64) -> Distance {
        Distance {
            value: miles,
            unit: DistanceUnit::Mile,
        }
    }

    pub fn to_kilometers(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value,
            DistanceUnit::Mile => self.value * 1.60934,
        }
    }

    pub fn to_miles(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value / 1.60934,
            DistanceUnit::Mile => self.value,
        }
    }
}

#[derive(Debug)]
pub enum DistanceUnit {
    Kilometer,
    Mile,
}
impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DistanceUnit::Kilometer => write!(f, "Kilometer"),
            DistanceUnit::Mile => write!(f, "Mile"),
        }
    }
}

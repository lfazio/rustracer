pub struct Radian {
    pub value: f64,
}

impl Radian {
    pub fn from(degrees: f64) -> Radian {
        Radian {
            value: degrees * std::f64::consts::PI / 180.0,
        }
    }
}
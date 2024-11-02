// http://web.mit.edu/fluids-modules/www/exper_techniques/2.Propagation_of_Uncertaint.pdf
use core::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
struct Measurement{
    measurement: f64,
    uncertainty: f64
}

impl Measurement{
    pub fn new(measurement: f64, uncertainty: f64)->Self{
        Self{
            measurement,
            uncertainty
        }
    }
    pub fn get_measurement(&self)->f64{
        return self.measurement
    }
    pub fn get_uncertainty(&self)->f64{
        return self.uncertainty
    }
}


impl Add for Measurement{
    type Output = Self;
    fn add(self, rhs: Self)->Self{
        Self{
            measurement: self.measurement + rhs.measurement,
            uncertainty: self.uncertainty + rhs.uncertainty
        }
    }
}

impl Sub for Measurement{
    type Output = Self;
    fn sub(self, rhs: Self)->Self{
        Self{
            measurement: self.measurement - rhs.measurement,
            uncertainty: self.uncertainty + rhs.uncertainty
        }
    }
}

impl Mul for Measurement{
    type Output = Self;
    fn mul(self, rhs: Self)->Self{
        let dx_x = self.uncertainty / self.measurement.abs();
        let dy_y = rhs.uncertainty / rhs.measurement.abs();
        let q = self.measurement * rhs.measurement;

        Self{
            measurement: q,
            uncertainty: (dx_x + dy_y)*q
        }
    }
}

impl Div for Measurement{
    type Output = Self;
    fn div(self, rhs: Self)->Self{
        let dx_x = self.uncertainty / self.measurement.abs();
        let dy_y = rhs.uncertainty / rhs.measurement.abs();
        let q = self.measurement / rhs.measurement;

        Self{
            measurement: q,
            uncertainty: (dx_x + dy_y)*q
        }
    }
}


#[cfg(test)]
mod test{
    // https://www.geol.lsu.edu/jlorenzo/geophysics/uncertainties/Uncertaintiespart2.html
    use super::*;
    #[test]
    fn add(){
        let x = Measurement::new(4.52, 0.02);
        let y = Measurement::new(2.0, 0.2);
        let z = Measurement::new(3.0, 0.6);
        let measurement = x+y+z;
        assert_eq!(measurement.measurement, 9.52);
        assert_eq!(measurement.uncertainty, 0.82);
    }
    #[test]
    fn sub(){
        let x = Measurement::new(4.52, 0.02);
        let y = Measurement::new(2.0, 0.2);
        let z = Measurement::new(3.0, 0.6);
        let measurement = x-y-z;
        assert_eq!(round(measurement.measurement, 2), -0.48);
        assert_eq!(round(measurement.uncertainty, 2), 0.82);
    }
    #[test]
    fn mul(){
        let x = Measurement::new(4.52, 0.02);
        let y = Measurement::new(2.0, 0.2);
        let measurement = x*y;
        assert_eq!(round(measurement.measurement, 3), 9.04);
        assert_eq!(round(measurement.uncertainty, 2), 0.94);
    }
    #[test]
    fn div(){
        let y = Measurement::new(2.0, 0.2);
        let z = Measurement::new(3.0, 0.6);
        let measurement = y/z;
        assert_eq!(round(measurement.measurement, 2), 0.67);
        assert_eq!(round(measurement.uncertainty, 2), 0.2);
    }

    fn round(x: f64, decimals: u32) -> f64 {
        if x == 0. || decimals == 0 {
          0.
        } else {
          let shift = decimals as i32 - x.abs().log10().ceil() as i32;
          let shift_factor = 10_f64.powi(shift);
          (x * shift_factor).round() / shift_factor
        }
    }

}
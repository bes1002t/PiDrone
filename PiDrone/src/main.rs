extern crate i2cdev;
extern crate docopt;

use std::thread;
use std::time::Duration;
use std::env::args;
use docopt::Docopt;
use i2cdev::sensors::{Thermometer, Barometer, Accelerometer};
use i2cdev::sensors::mpl115a2_barometer::*;
use i2cdev::sensors::adxl345_accelerometer::*;
#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;

const USAGE: &'static str = "
Reading sensor data from a variety of sensors
Usage:
  sensors <device>
  sensors (-h | --help)
  sensors --version
Options:
  -h --help    Show this help text.
  --version    Show version.
";


#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn main() {}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn main() {
    let args = Docopt::new(USAGE)
                   .and_then(|d| d.argv(args().into_iter()).parse())
                   .unwrap_or_else(|e| e.exit());
    let device = args.get_str("<device>");
    let mpl115a2_i2cdev = LinuxI2CDevice::new(device, MPL115A2_I2C_ADDR).unwrap();
    let adxl345_i2cdev = LinuxI2CDevice::new(device, 0x53).unwrap();

    let mut mpl115a2 = MPL115A2BarometerThermometer::new(mpl115a2_i2cdev).unwrap();
    let mut adxl345 = ADXL345Accelerometer::new(adxl345_i2cdev).unwrap();

    println!("== ADXL345 ID: 0x{:X} ==", adxl345.device_id().unwrap());

    loop {
        let accel = adxl345.accelerometer_sample().unwrap();
        println!("Temperature: {:?} C",
                 mpl115a2.temperature_celsius().unwrap());
        println!("Pressure:    {:?} kPa", mpl115a2.pressure_kpa().unwrap());
        println!("Accel:       {:?}", accel);
        println!("Accel Tot:   {:?}",
                 (accel.x.powi(2) + accel.y.powi(2) + accel.z.powi(2)).sqrt());
        thread::sleep(Duration::from_millis(1000));
    }
}
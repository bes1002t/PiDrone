use utils::bmp180_barometer;

fn main() {
	let sensor = BMP180_Barometer::new();
//    let hmc5883l_compass      = linuxi2cdevice::new(device, 0x3D);
//    let mpu6050_accelerometer = linuxi2cdevice::new(device, 0x68);
//    let mpu6050_gyroscope     = linuxi2cdevice::new(device, 0x68);
//    let bmp180_barometer      = linuxi2cdevice::new(device, 0x77);
//
//    loop {
//        bmp180_barometer.smbus_read_word_data(
//        let accel = adxl345.accelerometer_sample().unwrap();
//        println!("Temperature: {:?} C",
//                 mpl115a2.temperature_celsius().unwrap());
//        println!("Pressure:    {:?} kPa", mpl115a2.pressure_kpa().unwrap());
//        println!("Accel:       {:?}", accel);
//        println!("Accel Tot:   {:?}",
//                 (accel.x.powi(2) + accel.y.powi(2) + accel.z.powi(2)).sqrt());
//        thread::sleep(Duration::from_millis(1000));
//    }
}
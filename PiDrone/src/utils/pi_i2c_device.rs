use docopt::Docopt;
use i2cdev::linux::*;

struct Pi_I2C_Device {
	i2c_device: i2cdevice
}

impl Pi_I2C_Device {
	// If Raspberry Pi revision = 1 this value should be "/dev/i2c-0"
	device_uri = "/dev/i2c-1";

	pub fn new(hw_addr) -> I2cDevice {
		i2c_device = linuxi2cdevice::new(device_uri, hw_addr);
	}

	pub fn readU16(register) -> Result<(u16), MyError> {
		value = try!(i2c_device.smbus_read_word_data(self, register));
		Ok(value);
	}

	pub fn writeU16(register, value) -> Result<(), MyError> {
		try!(i2c_device.smbus_write_word_data(self, register, value));
		Ok(());
	}

	pub fn readU8(register) -> Result<(u8), MyError> {
		value = try!(i2c_device.smbus_read_byte_data(self, register));
		Ok(value);
	}

	pub fn writeU8(register, value) -> Result<(), MyError> {
		try!(i2c_device.smbus_write_byte_data(self, register, value));
		Ok(());
	}
}
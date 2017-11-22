struct BMP180_Barometer {
	bmp180_barometer: Pi_I2C_Device
}

impl BMP180_Barometer {
	let BMP180_HW_ADDRESS      = 0x77;

	// Operating Modes
	let BMP085_ULTRALOWPOWER   = 0;
	let BMP085_STANDARD        = 1;
	let BMP085_HIGHRES         = 2;
	let BMP085_ULTRAHIGHRES    = 3;

	// BMP085 Registers
	let BMP085_CAL_AC1         = 0xAA;  // R   Calibration data (16 bits)
	let BMP085_CAL_AC2         = 0xAC;  // R   Calibration data (16 bits)
	let BMP085_CAL_AC3         = 0xAE;  // R   Calibration data (16 bits)
	let BMP085_CAL_AC4         = 0xB0;  // R   Calibration data (16 bits)
	let BMP085_CAL_AC5         = 0xB2;  // R   Calibration data (16 bits)
	let BMP085_CAL_AC6         = 0xB4;  // R   Calibration data (16 bits)
	let BMP085_CAL_B1          = 0xB6;  // R   Calibration data (16 bits)
	let BMP085_CAL_B2          = 0xB8;  // R   Calibration data (16 bits)
	let BMP085_CAL_MB          = 0xBA;  // R   Calibration data (16 bits)
	let BMP085_CAL_MC          = 0xBC;  // R   Calibration data (16 bits)
	let BMP085_CAL_MD          = 0xBE;  // R   Calibration data (16 bits)
	let BMP085_CONTROL         = 0xF4;
	let BMP085_TEMPDATA        = 0xF6;
	let BMP085_PRESSUREDATA    = 0xF6;
	let BMP085_READTEMPCMD     = 0x2E;
	let BMP085_READPRESSURECMD = 0x34;

	// Private Fields
	let cal_AC1 = 0;
	let cal_AC2 = 0;
	let cal_AC3 = 0;
	let cal_AC4 = 0;
	let cal_AC5 = 0;
	let cal_AC6 = 0;
	let cal_B1  = 0;
	let cal_B2  = 0;
	let cal_MB  = 0;
	let cal_MC  = 0;
	let cal_MD  = 0;

	pub fn new() -> BMP180_Barometer {
		bmp180_barometer = Pi_I2C_Device::new(BMP180_Barometer.BMP180_HW_ADDRESS);
	}
}

// DEVICE MAP
pub const DEFAULT_ADDR: u8 = 0x28;

// Supported devices
pub const PID_CAP1208: u8 = 0b01101011;
pub const PID_CAP1188: u8 = 0b01010000;
pub const PID_CAP1166: u8 = 0b01010001;

// REGISTER MAP

pub const R_MAIN_CONTROL: u8 = 0x00;
pub const R_GENERAL_STATUS: u8 = 0x02;
pub const R_INPUT_STATUS: u8 = 0x03;
pub const R_LED_STATUS: u8 = 0x04;
pub const R_NOISE_FLAG_STATUS: u8 = 0x0A;

// Read-only delta counts for all inputs
pub const R_INPUT_1_DELTA: u8 = 0x10;
pub const R_INPUT_2_DELTA: u8 = 0x11;
pub const R_INPUT_3_DELTA: u8 = 0x12;
pub const R_INPUT_4_DELTA: u8 = 0x13;
pub const R_INPUT_5_DELTA: u8 = 0x14;
pub const R_INPUT_6_DELTA: u8 = 0x15;
pub const R_INPUT_7_DELTA: u8 = 0x16;
pub const R_INPUT_8_DELTA: u8 = 0x17;

pub const R_SENSITIVITY: u8 = 0x1F;
// B7     = N/A
// B6..B4 = Sensitivity
// B3..B0 = Base Shift
//const SENSITIVITY = {128: 0b000, 64:0b001, 32:0b010, 16:0b011, 8:0b100, 4:0b100, 2:0b110, 1:0b111}

pub const R_GENERAL_CONFIG: u8 = 0x20;
// B7 = Timeout
// B6 = Wake Config ( 1 = Wake pin asserted )
// B5 = Disable Digital Noise ( 1 = Noise threshold disabled )
// B4 = Disable Analog Noise ( 1 = Low frequency analog noise blocking disabled )
// B3 = Max Duration Recalibration ( 1 =  Enable recalibration if touch is held longer than max duration )
// B2..B0 = N/A

pub const R_INPUT_ENABLE: u8 = 0x21;

pub const R_INPUT_CONFIG: u8 = 0x22;

pub const R_INPUT_CONFIG2: u8 = 0x23; //  # Default 0x00000111

// Values for bits 3 to 0 of R_INPUT_CONFIG2
// Determines minimum amount of time before
// a "press and hold" event is detected.

// Also - Values for bits 3 to 0 of R_INPUT_CONFIG
// Determines rate at which interrupt will repeat
//
// Resolution of 35ms, max = 35 + (35 * 0b1111) = 560ms

pub const R_SAMPLING_CONFIG: u8 = 0x24; //  # Default 0x00111001
pub const R_CALIBRATION: u8 = 0x26; //  # Default 0b00000000
pub const R_INTERRUPT_EN: u8 = 0x27; //  # Default 0b11111111
pub const R_REPEAT_EN: u8 = 0x28; //  # Default 0b11111111
pub const R_MTOUCH_CONFIG: u8 = 0x2A; //  # Default 0b11111111
pub const R_MTOUCH_PAT_CONF: u8 = 0x2B;
pub const R_MTOUCH_PATTERN: u8 = 0x2D;
pub const R_COUNT_O_LIMIT: u8 = 0x2E;
pub const R_RECALIBRATION: u8 = 0x2F;

// R/W Touch detection thresholds for inputs
pub const R_INPUT_1_THRESH: u8 = 0x30;
pub const R_INPUT_2_THRESH: u8 = 0x31;
pub const R_INPUT_3_THRESH: u8 = 0x32;
pub const R_INPUT_4_THRESH: u8 = 0x33;
pub const R_INPUT_5_THRESH: u8 = 0x34;
pub const R_INPUT_6_THRESH: u8 = 0x35;
pub const R_INPUT_7_THRESH: u8 = 0x36;
pub const R_INPUT_8_THRESH: u8 = 0x37;

// R/W Noise threshold for all inputs
pub const R_NOISE_THRESH: u8 = 0x38;

// R/W Standby and Config Registers
pub const R_STANDBY_CHANNEL: u8 = 0x40;
pub const R_STANDBY_CONFIG: u8 = 0x41;
pub const R_STANDBY_SENS: u8 = 0x42;
pub const R_STANDBY_THRESH: u8 = 0x43;

pub const R_CONFIGURATION2: u8 = 0x44;
// B7 = Linked LED Transition Controls ( 1 = LED trigger is !touch )
// B6 = Alert Polarity ( 1 = Active Low Open Drain, 0 = Active High Push Pull )
// B5 = Reduce Power ( 1 = Do not power down between poll )
// B4 = Link Polarity/Mirror bits ( 0 = Linked, 1 = Unlinked )
// B3 = Show RF Noise ( 1 = Noise status registers only show RF, 0 = Both RF and EMI shown )
// B2 = Disable RF Noise ( 1 = Disable RF noise filter )
// B1..B0 = N/A

// Read-only reference counts for sensor inputs
pub const R_INPUT_1_BCOUNT: u8 = 0x50;
pub const R_INPUT_2_BCOUNT: u8 = 0x51;
pub const R_INPUT_3_BCOUNT: u8 = 0x52;
pub const R_INPUT_4_BCOUNT: u8 = 0x53;
pub const R_INPUT_5_BCOUNT: u8 = 0x54;
pub const R_INPUT_6_BCOUNT: u8 = 0x55;
pub const R_INPUT_7_BCOUNT: u8 = 0x56;
pub const R_INPUT_8_BCOUNT: u8 = 0x57;

// LED Controls - For CAP1188 and similar
pub const R_LED_OUTPUT_TYPE: u8 = 0x71;
pub const R_LED_LINKING: u8 = 0x72;
pub const R_LED_POLARITY: u8 = 0x73;
pub const R_LED_OUTPUT_CON: u8 = 0x74;
pub const R_LED_LTRANS_CON: u8 = 0x77;
pub const R_LED_MIRROR_CON: u8 = 0x79;

// LED Behaviour
pub const R_LED_BEHAVIOUR_1: u8 = 0x81; //  # For LEDs 1-4
pub const R_LED_BEHAVIOUR_2: u8 = 0x82; //  # For LEDs 5-8
pub const R_LED_PULSE_1_PER: u8 = 0x84;
pub const R_LED_PULSE_2_PER: u8 = 0x85;
pub const R_LED_BREATHE_PER: u8 = 0x86;
pub const R_LED_CONFIG: u8 = 0x88;
pub const R_LED_PULSE_1_DUT: u8 = 0x90;
pub const R_LED_PULSE_2_DUT: u8 = 0x91;
pub const R_LED_BREATHE_DUT: u8 = 0x92;
pub const R_LED_DIRECT_DUT: u8 = 0x93;
pub const R_LED_DIRECT_RAMP: u8 = 0x94;
pub const R_LED_OFF_DELAY: u8 = 0x95;

// R/W Power buttonc ontrol
pub const R_POWER_BUTTON: u8 = 0x60;
pub const R_POW_BUTTON_CONF: u8 = 0x61;

// Read-only upper 8-bit calibration values for sensors
pub const R_INPUT_1_CALIB: u8 = 0xB1;
pub const R_INPUT_2_CALIB: u8 = 0xB2;
pub const R_INPUT_3_CALIB: u8 = 0xB3;
pub const R_INPUT_4_CALIB: u8 = 0xB4;
pub const R_INPUT_5_CALIB: u8 = 0xB5;
pub const R_INPUT_6_CALIB: u8 = 0xB6;
pub const R_INPUT_7_CALIB: u8 = 0xB7;
pub const R_INPUT_8_CALIB: u8 = 0xB8;

// Read-only 2 LSBs for each sensor input
pub const R_INPUT_CAL_LSB1: u8 = 0xB9;
pub const R_INPUT_CAL_LSB2: u8 = 0xBA;

// Product ID Registers
pub const R_PRODUCT_ID: u8 = 0xFD;
pub const R_MANUFACTURER_ID: u8 = 0xFE;
pub const R_REVISION: u8 = 0xFF;

// LED Behaviour settings
pub const LED_BEHAVIOUR_DIRECT: u8 = 0b00;
pub const LED_BEHAVIOUR_PULSE1: u8 = 0b01;
pub const LED_BEHAVIOUR_PULSE2: u8 = 0b10;
pub const LED_BEHAVIOUR_BREATHE: u8 = 0b11;

pub const LED_OPEN_DRAIN: u8 = 0; //  # Default, LED is open-drain output with ext pullup
pub const LED_PUSH_PULL: u8 = 1; //  # LED is driven HIGH/LOW with logic 1/0

pub const LED_RAMP_RATE_2000MS: u8 = 7;
pub const LED_RAMP_RATE_1500MS: u8 = 6;
pub const LED_RAMP_RATE_1250MS: u8 = 5;
pub const LED_RAMP_RATE_1000MS: u8 = 4;
pub const LED_RAMP_RATE_750MS: u8 = 3;
pub const LED_RAMP_RATE_500MS: u8 = 2;
pub const LED_RAMP_RATE_250MS: u8 = 1;
pub const LED_RAMP_RATE_0MS: u8 = 0;

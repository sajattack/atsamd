#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::atsamd51g19a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd51g19a,

    /// Analog pin 0
    pin a0 = a2,
    /// Analog pin 1
    pin a1 = a5,
    /// INT pin
    pin a2 = a4,
    /// Microphone out
    pin micout = a6,
    /// Microphone in
    pin micin = a7,

    /// SDA
    pin sda = b8,
    /// SCL
    pin scl = b9,

    /// Accelerometer data signal (SDA)
    pin accel_sda = a12,
    /// Accelerometer clock signal (SCL)
    pin accel_scl = a13,

    /// Key Grid Column 0
    pin col0 = a14,
    /// Key Grid Column 1
    pin col1 = a15,
    /// Key Grid Column 2
    pin col2 = a16,
    /// Key Grid Column 3
    pin col3 = a17,
    /// Key Grid Column 4
    pin col4 = a20,
    /// Key Grid Column 5
    pin col5 = a21,
    /// Key Grid Column 6
    pin col6 = a22,
    /// Key Grid Column 7
    pin col7 = a23,

    /// Key Grid Row 0
    pin row0 = a18,
    /// Key Grid Row 1
    pin row1 = a19,
    /// Key Grid Row 2
    pin row2 = b22,
    /// Key Grid Row 3
    pin row3 = b23,

    /// NeoPixels
    pin neopixel = a27,

    /// APA102 (RGB LED control) MOSI
    pin dotstar_di = b3,
    /// APA102 (RGB LED control) SCK
    pin dotstar_ci = b2,
);

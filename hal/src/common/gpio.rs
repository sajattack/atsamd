//! Working with GPIO pins.
//! The pins are associated with the PORT hardware.  This module
//! defines a `split` method on the `PORT` type that is used to safely
//! reference the individual pin configuration.
//! The IO pins can be switched into alternate function modes, which
//! routes the pins to different peripherals depending on the mode
//! for the pin.   The pin configuration is reflected through the
//! use of type states to make the interface (ideally, or at least practically)
//! impossible to misuse.
use crate::target_device::port::GROUP;
use crate::target_device::PORT;
use core::marker::PhantomData;
use hal::digital::v2::OutputPin;

#[cfg(feature = "unproven")]
use hal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};

/// The GpioExt trait allows splitting the PORT hardware into
/// its constituent pin parts.
pub trait GpioExt {
    type Parts;
}

/// Represents a pin configured for input.
/// The MODE type is typically one of `Floating`, `PullDown` or
/// `PullUp`.
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Represents a pin configured for output.
/// The MODE type is typically one of `PushPull`, or
/// `OpenDrain`.
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

// The following collection of types is used to encode the
// state of the pin at compile time and helps to avoid misuse.

/// Floating Input
pub struct Floating;
/// Pulled down Input
pub struct PullDown;
/// Pulled up Input
pub struct PullUp;

/// Totem Pole aka Push-Pull
pub struct PushPull;
/// Open drain output
pub struct OpenDrain;
/// Open drain output, which can be read when not driven
pub struct ReadableOpenDrain;

/// Peripheral Function A
pub struct PfA;
/// Peripheral Function B
pub struct PfB;
/// Peripheral Function C
pub struct PfC;
/// Peripheral Function D
pub struct PfD;
/// Peripheral Function E
pub struct PfE;
/// Peripheral Function F
pub struct PfF;
/// Peripheral Function G
pub struct PfG;
/// Peripheral Function H
pub struct PfH;
/// Peripheral Function I
#[cfg(any(feature = "samd51", feature = "same54"))]
pub struct PfI;
/// Peripheral Function J
#[cfg(any(feature = "samd51", feature = "same54"))]
pub struct PfJ;
/// Peripheral Function K
#[cfg(any(feature = "samd51", feature = "same54"))]
pub struct PfK;
/// Peripheral Function L
#[cfg(any(feature = "samd51", feature = "same54"))]
pub struct PfL;
/// Peripheral Function M
#[cfg(any(feature = "samd51", feature = "same54"))]
pub struct PfM;
/// Peripheral Function N
#[cfg(any(feature = "samd51", feature = "same54"))]
pub struct PfN;

/// A trait that makes it easier to generically manage
/// converting a pin from its current state into some
/// other functional mode.  The configuration change
/// requires exclusive access to the GROUP hardware,
/// which is why this isn't simply the standard `Into`
/// trait.
pub trait IntoFunction<T> {
    /// Consume the pin and configure it to operate in
    /// the mode T.
    fn into_function(self, portgroup: &mut GROUP) -> T;
}

// rustfmt wants to keep indenting the nested macro on each run,
// so disable it for this whole block :-/
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! pin {
    (
        $group:ident,
        $PinType:ident,
        $pin_ident:ident,
        $pin_no:expr
    ) => {
        // Helper for pmux peripheral function configuration
        macro_rules! function {
            ($FuncType:ty, $func_ident:ident, $variant:expr) => {

        impl<MODE> $PinType<MODE> {
            /// Configures the pin to operate with a peripheral
            pub fn $func_ident(
                self,
                portgroup: &mut GROUP
            ) -> $PinType<$FuncType> {
                portgroup.pmux[$pin_no >> 1].modify(|_, w| {
                    if $pin_no & 1 == 1 {
                        // Odd-numbered pin
                      unsafe { w.pmuxo().bits($variant) }
                    } else {
                        // Even-numbered pin
                        unsafe { w.pmuxe().bits($variant) }
                    }
                });
                portgroup.pincfg[$pin_no].modify(|_, bits| {
                    bits.pmuxen().set_bit()
                });

                $PinType { _mode: PhantomData }
            }
        }
        impl<MODE> IntoFunction<$PinType<$FuncType>> for $PinType<MODE> {
            fn into_function(self, portgroup: &mut GROUP) -> $PinType<$FuncType> {
                self.$func_ident(portgroup)
            }
        }

            };
        }

        /// Represents the IO pin with the matching name.
        pub struct $PinType<MODE> {
            _mode: PhantomData<MODE>,
        }

        function!(PfA, into_function_a, 1);
        function!(PfB, into_function_b, 2);
        function!(PfC, into_function_c, 3);
        function!(PfD, into_function_d, 4);
        function!(PfE, into_function_e, 5);
        function!(PfF, into_function_f, 6);
        function!(PfG, into_function_g, 7);
        function!(PfH, into_function_h, 8);

        #[cfg(any(feature = "samd51", feature = "same54"))]
        function!(PfI, into_function_i, 9);
        #[cfg(any(feature = "samd51", feature = "same54"))]
        function!(PfJ, into_function_j, 10);
        #[cfg(any(feature = "samd51", feature = "same54"))]
        function!(PfK, into_function_k, 11);
        #[cfg(any(feature = "samd51", feature = "same54"))]
        function!(PfL, into_function_l, 12);
        #[cfg(any(feature = "samd51", feature = "same54"))]
        function!(PfM, into_function_m, 13);
        #[cfg(any(feature = "samd51", feature = "same54"))]
        function!(PfN, into_function_n, 14);

        impl<MODE> $PinType<MODE> {

            // TODO: datasheet mentions this, but is likely for
            // a slightly different variant
            // function!(PfI, into_function_i, i);

            /// Configures the pin to operate as a floating input
            pub fn into_floating_input(self, portgroup: &mut GROUP) -> $PinType<Input<Floating>> {
                portgroup.dirclr.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                portgroup.pincfg[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a pulled down input pin
            pub fn into_pull_down_input(self, portgroup: &mut GROUP) -> $PinType<Input<PullDown>> {
                portgroup.dirclr.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                portgroup.pincfg[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().set_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                // Pull down
                portgroup.outclr.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a pulled up input pin
            pub fn into_pull_up_input(self, portgroup: &mut GROUP) -> $PinType<Input<PullUp>> {
                portgroup.dirclr.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                portgroup.pincfg[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().set_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                // Pull up
                portgroup.outset.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as an open drain output
            pub fn into_open_drain_output(self, portgroup: &mut GROUP) -> $PinType<Output<OpenDrain>> {
                portgroup.dirset.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                portgroup.pincfg[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().clear_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as an open drain output which can be read
            pub fn into_readable_open_drain_output(self, portgroup: &mut GROUP) -> $PinType<Output<ReadableOpenDrain>> {
                portgroup.dirset.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                portgroup.pincfg[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a push-pull output
            pub fn into_push_pull_output(self, portgroup: &mut GROUP) -> $PinType<Output<PushPull>> {
                portgroup.dirset.write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                portgroup.pincfg[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }
        }

        impl $PinType<Output<OpenDrain>> {
            /// Control state of the internal pull up
            pub fn internal_pull_up(&mut self, portgroup: &mut GROUP, on: bool) {
                portgroup.pincfg[$pin_no].write(|bits| {
                    if on {
                        bits.pullen().set_bit();
                    } else {
                        bits.pullen().clear_bit();
                    }
                    bits
                });
            }
        }

        impl<MODE> $PinType<Output<MODE>> {
            /// Toggle the logic level of the pin; if it is currently
            /// high, set it low and vice versa.
            pub fn toggle(&mut self, portgroup: &mut GROUP) {
                self.toggle_impl(portgroup);
            }

            fn toggle_impl(&mut self, portgroup: &mut GROUP) {
                unsafe {
                    portgroup.outtgl.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> ToggleableOutputPin for $PinType<Output<MODE>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn toggle(&mut self) -> Result<(), Self::Error> {
                self.toggle_impl();

                Ok(())
            }
        }

        #[cfg(feature = "unproven")]
        impl InputPin for $PinType<Output<ReadableOpenDrain>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).in_.read().bits()) & (1 << $pin_no)) != 0 })
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).in_.read().bits()) & (1 << $pin_no)) == 0 })
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> InputPin for $PinType<Input<MODE>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.in_.read().bits()) & (1 << $pin_no)) != 0 })
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.in_.read().bits()) & (1 << $pin_no)) == 0 })
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> StatefulOutputPin for $PinType<Output<MODE>> {
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.out.read().bits()) & (1 << $pin_no)) != 0 })
            }

            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.out.read().bits()) & (1 << $pin_no)) == 0 })
            }
        }


        impl<MODE> OutputPin for $PinType<Output<MODE>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*PORT::ptr()).$group.outset.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }

                Ok(())
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*PORT::ptr()).$group.outclr.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }

                Ok(())
            }
        }
    };
}

pub struct Port {
    _0: ()
}

impl Port {
    fn group0(&mut self) -> &GROUP {
       unsafe { &(*PORT::ptr()).group0 }
    }
    fn group1(&mut self) -> &GROUP {
       unsafe { &(*PORT::ptr()).group1 }
    }
}

macro_rules! port {
    ([
       $($PinTypeA:ident: ($groupA:ident, $pin_identA:ident, $pin_noA:expr),)+
    ],[
       $($PinTypeB:ident: ($groupB:ident, $pin_identB:ident, $pin_noB:expr),)+
    ],[
       $($PinTypeC:ident: ($groupC:ident, $pin_identC:ident, $pin_noC:expr),)+
    ],[
       $($PinTypeD:ident: ($groupD:ident, $pin_identD:ident, $pin_noD:expr),)+
    ]) => {

/// Holds the GPIO GROUP peripheral and broken out pin instances
pub struct Parts {
    pub portgroup: GROUP,

    $(
        /// Pin $pin_identA
        pub $pin_identA: $PinTypeA<Input<Floating>>,
    )+
    $(
        /// Pin $pin_identB
        #[cfg(any(feature = "samd21g18a", feature="samd21j18a", feature = "samd51", feature = "same54"))]
        pub $pin_identB: $PinTypeB<Input<Floating>>,
    )+
    $(
        /// Pin $pin_identC
        #[cfg(any(feature = "same54"))]
        pub $pin_identC: $PinTypeC<Input<Floating>>,
    )+
    $(
        /// Pin $pin_identD
        #[cfg(any(feature = "same54"))]
        pub $pin_identD: $PinTypeD<Input<Floating>>,
    )+
}

$(
    pin!($groupA, $PinTypeA, $pin_identA, $pin_noA);
)+
$(
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a", feature = "samd51", feature = "same54"))]
    pin!($groupB, $PinTypeB, $pin_identB, $pin_noB);
)+
$(
    #[cfg(any(feature = "same54"))]
    pin!($groupC, $PinTypeC, $pin_identC, $pin_noC);
)+
$(
    #[cfg(any(feature = "same54"))]
    pin!($groupD, $PinTypeD, $pin_identD, $pin_noD);
)+

    };
}

port!([
    Pa0: (group0, pa0, 0),
    Pa1: (group0, pa1, 1),
    Pa2: (group0, pa2, 2),
    Pa3: (group0, pa3, 3),
    Pa4: (group0, pa4, 4),
    Pa5: (group0, pa5, 5),
    Pa6: (group0, pa6, 6),
    Pa7: (group0, pa7, 7),
    Pa8: (group0, pa8, 8),
    Pa9: (group0, pa9, 9),
    Pa10: (group0, pa10, 10),
    Pa11: (group0, pa11, 11),
    Pa12: (group0, pa12, 12),
    Pa13: (group0, pa13, 13),
    Pa14: (group0, pa14, 14),
    Pa15: (group0, pa15, 15),
    Pa16: (group0, pa16, 16),
    Pa17: (group0, pa17, 17),
    Pa18: (group0, pa18, 18),
    Pa19: (group0, pa19, 19),
    Pa20: (group0, pa20, 20),
    Pa21: (group0, pa21, 21),
    Pa22: (group0, pa22, 22),
    Pa23: (group0, pa23, 23),
    Pa24: (group0, pa24, 24),
    Pa25: (group0, pa25, 25),
    Pa26: (group0, pa26, 26),
    Pa27: (group0, pa27, 27),
    Pa28: (group0, pa28, 28),
    Pa29: (group0, pa29, 29),
    Pa30: (group0, pa30, 30),
    Pa31: (group0, pa31, 31),
],[
    Pb0: (group1, pb0, 0),
    Pb1: (group1, pb1, 1),
    Pb2: (group1, pb2, 2),
    Pb3: (group1, pb3, 3),
    Pb4: (group1, pb4, 4),
    Pb5: (group1, pb5, 5),
    Pb6: (group1, pb6, 6),
    Pb7: (group1, pb7, 7),
    Pb8: (group1, pb8, 8),
    Pb9: (group1, pb9, 9),
    Pb10: (group1, pb10, 10),
    Pb11: (group1, pb11, 11),
    Pb12: (group1, pb12, 12),
    Pb13: (group1, pb13, 13),
    Pb14: (group1, pb14, 14),
    Pb15: (group1, pb15, 15),
    Pb16: (group1, pb16, 16),
    Pb17: (group1, pb17, 17),
    Pb18: (group1, pb18, 18),
    Pb19: (group1, pb19, 19),
    Pb20: (group1, pb20, 20),
    Pb21: (group1, pb21, 21),
    Pb22: (group1, pb22, 22),
    Pb23: (group1, pb23, 23),
    Pb24: (group1, pb24, 24),
    Pb25: (group1, pb25, 25),
    Pb26: (group1, pb26, 26),
    Pb27: (group1, pb27, 27),
    Pb28: (group1, pb28, 28),
    Pb29: (group1, pb29, 29),
    Pb30: (group1, pb30, 30),
    Pb31: (group1, pb31, 31),
],
[
    Pc0: (group2, pc0, 0),
    Pc1: (group2, pc1, 1),
    Pc2: (group2, pc2, 2),
    Pc3: (group2, pc3, 3),
    Pc4: (group2, pc4, 4),
    Pc5: (group2, pc5, 5),
    Pc6: (group2, pc6, 6),
    Pc7: (group2, pc7, 7),
    Pc10: (group2, pc10, 10),
    Pc11: (group2, pc11, 11),
    Pc12: (group2, pc12, 12),
    Pc13: (group2, pc13, 13),
    Pc14: (group2, pc14, 14),
    Pc15: (group2, pc15, 15),
    Pc16: (group2, pc16, 16),
    Pc17: (group2, pc17, 17),
    Pc18: (group2, pc18, 18),
    Pc19: (group2, pc19, 19),
    Pc20: (group2, pc20, 20),
    Pc21: (group2, pc21, 21),
    Pc22: (group2, pc22, 22),
    Pc23: (group2, pc23, 23),
    Pc24: (group2, pc24, 24),
    Pc25: (group2, pc25, 25),
    Pc26: (group2, pc26, 26),
    Pc27: (group2, pc27, 27),
    Pc28: (group2, pc28, 28),
    Pc30: (group2, pc30, 30),
    Pc31: (group2, pc31, 31),
],
[
    Pd0: (group3, pd0, 0),
    Pd1: (group3, pd1, 1),
    Pd8: (group3, pd8, 8),
    Pd9: (group3, pd9, 9),
    Pd10: (group3, pd10, 10),
    Pd11: (group3, pd11, 11),
    Pd12: (group3, pd12, 12),
    Pd20: (group3, pd20, 20),
    Pd21: (group3, pd21, 21),
]);

/// This macro is a helper for defining a `Pins` type in a board support
/// crate.  This type is used to provide more meaningful aliases for the
/// various GPIO pins for a given board.
#[macro_export]
macro_rules! define_pins {
    ($(#[$topattr:meta])* struct $Type:ident,
     target_device: $target_device:ident,
     $( $(#[$attr:meta])* pin $name:ident = $pin_ident:ident),+ , ) => {

$crate::paste::item! {
    $(#[$topattr])*
    pub struct $Type {
        /// Opaque port reference
        pub portgroup: GROUP,

        $(
            $(#[$attr])*
            pub $name: gpio::[<P $pin_ident>]<Input<Floating>>
        ),+
    }
}

impl $Type {
    /// Returns the pins for the device
    $crate::paste::item! {
        pub fn new(portgroup: $target_device::GROUP) -> Self {
            $Type {
                portgroup: portgroup,
                $(
                $name: pins.[<p $pin_ident>]
                ),+
            }
        }
    }
}
}}

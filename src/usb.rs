//! USB peripheral
//!
//! Requires the `stm32-usbd` feature.
//!
//! See `examples` directory for usage examples.

#![cfg(feature = "stm32-usbd")]

use crate::stm32::{RCC, USB};
use stm32_usbd::UsbPeripheral;

use crate::gpio::gpioa::{PA11, PA12};
use crate::gpio::{Alternate, Floating, Input, AF10};
pub use stm32_usbd::UsbBus;

pub struct Peripheral {
    pub usb: USB,
    pub pin_dm: PA11<Alternate<AF10, Input<Floating>>>,
    pub pin_dp: PA12<Alternate<AF10, Input<Floating>>>,
}

unsafe impl Sync for Peripheral {}

unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = USB::ptr() as *const ();
    const DP_PULL_UP_FEATURE: bool = true;
    const EP_MEMORY: *const () = 0x4000_6c00 as _;
    const EP_MEMORY_SIZE: usize = 1024;
    // TG-CHANGE-CRITICAL: add this as true due to stm32-usbd update to 0.6.0
    const EP_MEMORY_ACCESS_2X16: bool = true;

    fn enable() {
        let rcc = unsafe { &*RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.apb1enr1.modify(|_, w| w.usben().set_bit());

            // Reset USB peripheral
            rcc.apb1rstr1.modify(|_, w| w.usbfsrst().set_bit());
            rcc.apb1rstr1.modify(|_, w| w.usbfsrst().clear_bit());
        });
    }

    fn startup_delay() {
        cortex_m::asm::delay(100);
    }
}

pub type UsbBusType = UsbBus<Peripheral>;

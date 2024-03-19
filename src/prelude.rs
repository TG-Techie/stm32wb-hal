//! Prelude - Include traits for hal

// TG-TODO: update to embedded-hal 1.0
pub use crate::hal::prelude::*; // embedded hal traits

// TG-TODO: update to embedded-hal 1.0
pub use crate::hal::digital::v2::OutputPin;

pub use crate::datetime::U32Ext as _stm32wb_hal_datetime_U32Ext;
pub use crate::ipcc::IpccExt as _stm32wb_hal_ipcc_IpccExt;
//pub use crate::dma::DmaExt as _stm32wb_hal_DmaExt;
//pub use crate::flash::FlashExt as _stm32wb_hal_FlashExt;
pub use crate::gpio::GpioExt as _stm32wb_hal_GpioExt;
pub use crate::pwm::PwmExt1 as _stm32l4_hal_PwmExt1;
pub use crate::pwm::PwmExt2 as _stm32l4_hal_PwmExt2;
pub use crate::rcc::RccExt as _stm32wb_hal_RccExt;
pub use crate::time::U32Ext as _stm32wb_hal_time_U32Ext;

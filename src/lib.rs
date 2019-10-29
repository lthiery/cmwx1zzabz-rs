//! Rust Board Support Crate (BSC) for the STM32 LoRa Discovery Board (B-L072Z-LRWAN1)
#![no_std]
pub extern crate stm32l0xx_hal as hal;
mod longfi_bindings;

pub use longfi_bindings::initialize_irq as initialize_radio_irq;
pub use longfi_bindings::LongFiBindings;
pub use longfi_bindings::RadioIRQ;
pub use longfi_bindings::TcxoEn;

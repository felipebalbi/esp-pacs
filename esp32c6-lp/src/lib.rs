#![doc = "Peripheral access API for ESP32-C6-LP microcontrollers (generated using svd2rust v0.29.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/46717278")]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn LP_TIMER();
    fn LP_UART();
    fn LP_I2C();
    fn LP_WDT();
    fn LP_PERI_TIMEOUT();
    fn LP_APM_M0();
    fn LP_APM_M1();
}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".trap.rodata"]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 22] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LP_TIMER },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LP_UART },
    Vector { _handler: LP_I2C },
    Vector { _handler: LP_WDT },
    Vector {
        _handler: LP_PERI_TIMEOUT,
    },
    Vector {
        _handler: LP_APM_M0,
    },
    Vector {
        _handler: LP_APM_M1,
    },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "Low-power I2C (Inter-Integrated Circuit) Controller"]
pub struct LP_I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_I2C {}
impl LP_I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_i2c::RegisterBlock = 0x600b_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_I2C {
    type Target = lp_i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_I2C").finish()
    }
}
#[doc = "Low-power I2C (Inter-Integrated Circuit) Controller"]
pub mod lp_i2c;
#[doc = "LP_PERI Peripheral"]
pub struct LP_PERI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_PERI {}
impl LP_PERI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_peri::RegisterBlock = 0x600b_2800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_peri::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_PERI {
    type Target = lp_peri::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_PERI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PERI").finish()
    }
}
#[doc = "LP_PERI Peripheral"]
pub mod lp_peri;
#[doc = "LP_ANA_PERI Peripheral"]
pub struct LP_ANA_PERI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_ANA_PERI {}
impl LP_ANA_PERI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_ana_peri::RegisterBlock = 0x600b_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_ana_peri::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_ANA_PERI {
    type Target = lp_ana_peri::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_ANA_PERI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_PERI").finish()
    }
}
#[doc = "LP_ANA_PERI Peripheral"]
pub mod lp_ana_peri;
#[doc = "LP_AON Peripheral"]
pub struct LP_AON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_AON {}
impl LP_AON {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_aon::RegisterBlock = 0x600b_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_aon::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_AON {
    type Target = lp_aon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_AON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AON").finish()
    }
}
#[doc = "LP_AON Peripheral"]
pub mod lp_aon;
#[doc = "Low-power Access Permission Management Controller"]
pub struct LP_APM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_APM {}
impl LP_APM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_apm::RegisterBlock = 0x600b_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_apm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_APM {
    type Target = lp_apm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_APM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_APM").finish()
    }
}
#[doc = "Low-power Access Permission Management Controller"]
pub mod lp_apm;
#[doc = "LP_CLKRST Peripheral"]
pub struct LP_CLKRST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_CLKRST {}
impl LP_CLKRST {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_clkrst::RegisterBlock = 0x600b_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_clkrst::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_CLKRST {
    type Target = lp_clkrst::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_CLKRST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLKRST").finish()
    }
}
#[doc = "LP_CLKRST Peripheral"]
pub mod lp_clkrst;
#[doc = "LP_I2C_ANA_MST Peripheral"]
pub struct LP_I2C_ANA_MST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_I2C_ANA_MST {}
impl LP_I2C_ANA_MST {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_i2c_ana_mst::RegisterBlock = 0x600b_2400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_i2c_ana_mst::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_I2C_ANA_MST {
    type Target = lp_i2c_ana_mst::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_I2C_ANA_MST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_I2C_ANA_MST").finish()
    }
}
#[doc = "LP_I2C_ANA_MST Peripheral"]
pub mod lp_i2c_ana_mst;
#[doc = "Low-power Input/Output Multiplexer"]
pub struct LP_IO_MUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_IO_MUX {}
impl LP_IO_MUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_io_mux::RegisterBlock = 0x600b_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_io_mux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_IO_MUX {
    type Target = lp_io_mux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_IO_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_IO_MUX").finish()
    }
}
#[doc = "Low-power Input/Output Multiplexer"]
pub mod lp_io_mux;
#[doc = "Low-power Trusted Execution Environment"]
pub struct LP_TEE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_TEE {}
impl LP_TEE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_tee::RegisterBlock = 0x600b_3400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_tee::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_TEE {
    type Target = lp_tee::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_TEE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TEE").finish()
    }
}
#[doc = "Low-power Trusted Execution Environment"]
pub mod lp_tee;
#[doc = "Low-power Timer"]
pub struct LP_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_TIMER {}
impl LP_TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_timer::RegisterBlock = 0x600b_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_TIMER {
    type Target = lp_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TIMER").finish()
    }
}
#[doc = "Low-power Timer"]
pub mod lp_timer;
#[doc = "Low-power UART (Universal Asynchronous Receiver-Transmitter) Controller"]
pub struct LP_UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_UART {}
impl LP_UART {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_uart::RegisterBlock = 0x600b_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_UART {
    type Target = lp_uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_UART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_UART").finish()
    }
}
#[doc = "Low-power UART (Universal Asynchronous Receiver-Transmitter) Controller"]
pub mod lp_uart;
#[doc = "Low-power Watchdog Timer"]
pub struct LP_WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LP_WDT {}
impl LP_WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lp_wdt::RegisterBlock = 0x600b_1c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lp_wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LP_WDT {
    type Target = lp_wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LP_WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_WDT").finish()
    }
}
#[doc = "Low-power Watchdog Timer"]
pub mod lp_wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "LP_I2C"]
    pub LP_I2C: LP_I2C,
    #[doc = "LP_PERI"]
    pub LP_PERI: LP_PERI,
    #[doc = "LP_ANA_PERI"]
    pub LP_ANA_PERI: LP_ANA_PERI,
    #[doc = "LP_AON"]
    pub LP_AON: LP_AON,
    #[doc = "LP_APM"]
    pub LP_APM: LP_APM,
    #[doc = "LP_CLKRST"]
    pub LP_CLKRST: LP_CLKRST,
    #[doc = "LP_I2C_ANA_MST"]
    pub LP_I2C_ANA_MST: LP_I2C_ANA_MST,
    #[doc = "LP_IO_MUX"]
    pub LP_IO_MUX: LP_IO_MUX,
    #[doc = "LP_TEE"]
    pub LP_TEE: LP_TEE,
    #[doc = "LP_TIMER"]
    pub LP_TIMER: LP_TIMER,
    #[doc = "LP_UART"]
    pub LP_UART: LP_UART,
    #[doc = "LP_WDT"]
    pub LP_WDT: LP_WDT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            LP_I2C: LP_I2C {
                _marker: PhantomData,
            },
            LP_PERI: LP_PERI {
                _marker: PhantomData,
            },
            LP_ANA_PERI: LP_ANA_PERI {
                _marker: PhantomData,
            },
            LP_AON: LP_AON {
                _marker: PhantomData,
            },
            LP_APM: LP_APM {
                _marker: PhantomData,
            },
            LP_CLKRST: LP_CLKRST {
                _marker: PhantomData,
            },
            LP_I2C_ANA_MST: LP_I2C_ANA_MST {
                _marker: PhantomData,
            },
            LP_IO_MUX: LP_IO_MUX {
                _marker: PhantomData,
            },
            LP_TEE: LP_TEE {
                _marker: PhantomData,
            },
            LP_TIMER: LP_TIMER {
                _marker: PhantomData,
            },
            LP_UART: LP_UART {
                _marker: PhantomData,
            },
            LP_WDT: LP_WDT {
                _marker: PhantomData,
            },
        }
    }
}

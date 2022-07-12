#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const RPI_HWVER_TYPE_UNKNOWN: u32 = 0;
pub const RPI_HWVER_TYPE_PI1: u32 = 1;
pub const RPI_HWVER_TYPE_PI2: u32 = 2;
pub const RPI_HWVER_TYPE_PI4: u32 = 3;
pub const RPI_PWM_CHANNELS: u32 = 2;
pub const RPI_PWM_CTL_MSEN2: u32 = 32768;
pub const RPI_PWM_CTL_USEF2: u32 = 8192;
pub const RPI_PWM_CTL_POLA2: u32 = 4096;
pub const RPI_PWM_CTL_SBIT2: u32 = 2048;
pub const RPI_PWM_CTL_RPTL2: u32 = 1024;
pub const RPI_PWM_CTL_MODE2: u32 = 512;
pub const RPI_PWM_CTL_PWEN2: u32 = 256;
pub const RPI_PWM_CTL_MSEN1: u32 = 128;
pub const RPI_PWM_CTL_CLRF1: u32 = 64;
pub const RPI_PWM_CTL_USEF1: u32 = 32;
pub const RPI_PWM_CTL_POLA1: u32 = 16;
pub const RPI_PWM_CTL_SBIT1: u32 = 8;
pub const RPI_PWM_CTL_RPTL1: u32 = 4;
pub const RPI_PWM_CTL_MODE1: u32 = 2;
pub const RPI_PWM_CTL_PWEN1: u32 = 1;
pub const RPI_PWM_STA_STA4: u32 = 4096;
pub const RPI_PWM_STA_STA3: u32 = 2048;
pub const RPI_PWM_STA_STA2: u32 = 1024;
pub const RPI_PWM_STA_STA1: u32 = 512;
pub const RPI_PWM_STA_BERR: u32 = 256;
pub const RPI_PWM_STA_GAP04: u32 = 128;
pub const RPI_PWM_STA_GAP03: u32 = 64;
pub const RPI_PWM_STA_GAP02: u32 = 32;
pub const RPI_PWM_STA_GAP01: u32 = 16;
pub const RPI_PWM_STA_RERR1: u32 = 8;
pub const RPI_PWM_STA_WERR1: u32 = 4;
pub const RPI_PWM_STA_EMPT1: u32 = 2;
pub const RPI_PWM_STA_FULL1: u32 = 1;
pub const RPI_PWM_DMAC_ENAB: u32 = 2147483648;
pub const PWM_OFFSET: u32 = 2146304;
pub const PWM_PERIPH_PHYS: u32 = 2116075520;
pub const WS2811_TARGET_FREQ: u32 = 800000;
pub const SK6812_STRIP_RGBW: u32 = 403703808;
pub const SK6812_STRIP_RBGW: u32 = 403701768;
pub const SK6812_STRIP_GRBW: u32 = 403181568;
pub const SK6812_STRIP_GBRW: u32 = 403177488;
pub const SK6812_STRIP_BRGW: u32 = 402657288;
pub const SK6812_STRIP_BGRW: u32 = 402655248;
pub const SK6812_SHIFT_WMASK: u32 = 4026531840;
pub const WS2811_STRIP_RGB: u32 = 1050624;
pub const WS2811_STRIP_RBG: u32 = 1048584;
pub const WS2811_STRIP_GRB: u32 = 528384;
pub const WS2811_STRIP_GBR: u32 = 524304;
pub const WS2811_STRIP_BRG: u32 = 4104;
pub const WS2811_STRIP_BGR: u32 = 2064;
pub const WS2812_STRIP: u32 = 528384;
pub const SK6812_STRIP: u32 = 528384;
pub const SK6812W_STRIP: u32 = 403181568;

pub type ws2811_led_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rpi_hw_t {
    pub type_: u32,
    pub hwver: u32,
    pub periph_base: u32,
    pub videocore_base: u32,
    pub desc: *mut libc::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct videocore_mbox_t {
    pub handle: libc::c_int,
    pub mem_ref: libc::c_uint,
    pub bus_addr: libc::c_uint,
    pub size: libc::c_uint,
    pub virt_addr: *mut u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ws2811_device {
    pub driver_mode: libc::c_int,
    pub pxl_raw: *mut u8,
    pub dma: *mut libc::c_void,
    pub pwm: *mut libc::c_void,
    pub pcm: *mut libc::c_void,
    pub spi_fd: libc::c_int,
    pub dma_cb: *mut libc::c_void,
    pub dma_cb_addr: u32,
    pub gpio: *mut libc::c_void,
    pub cm_clk: *mut libc::c_void,
    pub mbox: videocore_mbox_t,
    pub max_count: libc::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ws2811_channel_t {
    pub gpionum: ::libc::c_int,
    pub invert: ::libc::c_int,
    pub count: ::libc::c_int,
    pub strip_type: ::libc::c_int,
    pub leds: *mut ws2811_led_t,
    pub brightness: u8,
    pub wshift: u8,
    pub rshift: u8,
    pub gshift: u8,
    pub bshift: u8,
    pub gamma: *mut u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ws2811_t {
    pub render_wait_time: u64,
    pub device: *mut ws2811_device,
    pub rpi_hw: *const rpi_hw_t,
    pub freq: u32,
    pub dmanum: ::libc::c_int,
    pub channel: [ws2811_channel_t; RPI_PWM_CHANNELS as usize],
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ws2811_return_t {
    WS2811_SUCCESS = 0,
    WS2811_ERROR_GENERIC = -1,
    WS2811_ERROR_OUT_OF_MEMORY = -2,
    WS2811_ERROR_HW_NOT_SUPPORTED = -3,
    WS2811_ERROR_MEM_LOCK = -4,
    WS2811_ERROR_MMAP = -5,
    WS2811_ERROR_MAP_REGISTERS = -6,
    WS2811_ERROR_GPIO_INIT = -7,
    WS2811_ERROR_PWM_SETUP = -8,
    WS2811_ERROR_MAILBOX_DEVICE = -9,
    WS2811_ERROR_DMA = -10,
    WS2811_ERROR_ILLEGAL_GPIO = -11,
    WS2811_ERROR_PCM_SETUP = -12,
    WS2811_ERROR_SPI_SETUP = -13,
    WS2811_ERROR_SPI_TRANSFER = -14,
}

impl ws2811_return_t {
    #[inline]
    ///Gets textual representation of error.
    ///
    ///Returns empty string on encoding error.
    pub fn as_str(self) -> &'static str {
        let bytes = unsafe {
            let c_str = ws2811_get_return_t_str(self);
            let len = libc::strlen(c_str);
            core::slice::from_raw_parts(c_str as *const u8, len as usize)
        };

        match core::str::from_utf8(bytes) {
            Ok(result) => result,
            Err(_) => "",
        }
    }
}

impl core::fmt::Display for ws2811_return_t {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

extern "C" {
    ///Allocate and initialize memory, buffers, pages, PWM, DMA, and GPIO.
    ///
    ///Initialize following fields before passing to this function
    ///```c++
    ///
    ///const int GPIO_PIN = 18; //PWM
    ///const int WIDTH = 8;
    ///const int HEIGHT = 8;
    ///const int LED_COUNT = WIDTH * HEIGHT;
    ///const int STRIP_TYPE = WS2811_STRIP_RGB;
    ///
    ///ws2811_t ledstring =
    ///{
    ///    .freq = WS2811_TARGET_FREQ,
    ///    .dmanum = 10,
    ///    .channel =
    ///    {
    ///        [0] =
    ///        {
    ///            .gpionum = GPIO_PIN,
    ///            .invert = 0,
    ///            .count = LED_COUNT,
    ///            .strip_type = STRIP_TYPE,
    ///            .brightness = 255,
    ///        },
    ///        [1] =
    ///        {
    ///            .gpionum = 0,
    ///            .invert = 0,
    ///            .count = 0,
    ///            .brightness = 0,
    ///        },
    ///    },
    ///};
    ///```
    ///
    ///On success returns WS2811_SUCCESS
    pub fn ws2811_init(ws2811: *mut ws2811_t) -> ws2811_return_t;
    ///Shut down DMA, PWM, and cleanup memory.
    pub fn ws2811_fini(ws2811: *mut ws2811_t);
    ///Renders LEDs onto device.
    ///
    ///Can only fail due to DMA/SPI transfer errors
    pub fn ws2811_render(ws2811: *mut ws2811_t) -> ws2811_return_t;
    ///Wait for any executing DMA operation to complete before returning.
    pub fn ws2811_wait(ws2811: *mut ws2811_t) -> ws2811_return_t;
    ///Gets textual description of error.
    pub fn ws2811_get_return_t_str(state: ws2811_return_t) -> *const ::libc::c_char;
    ///Sets gamma factor
    pub fn ws2811_set_custom_gamma_factor(ws2811: *mut ws2811_t, gamma_factor: f64);
    ///Determines raspberry pi version
    pub fn rpi_hw_detect() -> *const rpi_hw_t;
}

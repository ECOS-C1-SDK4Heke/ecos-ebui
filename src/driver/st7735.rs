use core::marker::Copy;
use core::prelude::rust_2024::derive;
use core::{
    default::Default,
    option::Option,
    option::Option::{None, Some},
    result::Result,
    result::Result::Ok,
};

use core::clone::Clone;
use core::fmt::Debug;

use embedded_hal::digital::OutputPin;

use crate::adapter::delay::EbdHalDelay;
use crate::adapter::gpio::EbdHalGpio;
use crate::adapter::spi::{EbdHalSpiDevice, SpiError};

#[cfg(feature = "st7735-lcd")]
use st7735_lcd::ST7735;

#[cfg(feature = "st7735-lcd-doublebuffering")]
use st7735_lcd_doublebuffering::{Orientation, ST7735Buffered};

/// ST7735显示类型
#[cfg(feature = "st7735-lcd")]
pub type St7735Display = ST7735<EbdHalSpiDevice, EbdHalGpio, EbdHalGpio>;

#[cfg(feature = "st7735-lcd-doublebuffering")]
pub type St7735Display = ST7735Buffered<EbdHalSpiDevice, EbdHalGpio>;

/// ST7735硬件配置
#[derive(Debug, Clone, Copy)]
pub struct St7735Config {
    /// DC引脚（数据/命令选择）
    pub dc_pin: u32,
    /// RST引脚（复位，可选）
    pub rst_pin: Option<u32>,
    /// 屏幕宽度
    pub width: u16,
    /// 屏幕高度
    pub height: u16,
    /// RGB模式（true = RGB，false = BGR）
    pub rgb: bool,
    /// 颜色反转
    pub inverted: bool,
}

impl Default for St7735Config {
    fn default() -> Self {
        Self {
            dc_pin: 14, // 对应实际引脚`2`
            rst_pin: None,
            width: 128,
            height: 128,
            rgb: false,
            inverted: false,
        }
    }
}

/// ST7735显示构建器
pub struct St7735Builder {
    config: St7735Config,
}

impl St7735Builder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: St7735Config::default(),
        }
    }

    /// 设置DC引脚
    pub fn dc_pin(mut self, pin: u32) -> Self {
        self.config.dc_pin = pin;
        self
    }

    /// 设置RST引脚
    pub fn rst_pin(mut self, pin: u32) -> Self {
        self.config.rst_pin = Some(pin);
        self
    }

    /// 设置屏幕尺寸
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    /// 设置RGB模式
    pub fn rgb(mut self, rgb: bool) -> Self {
        self.config.rgb = rgb;
        self
    }

    /// 设置颜色反转
    pub fn inverted(mut self, inverted: bool) -> Self {
        self.config.inverted = inverted;
        self
    }

    /// 构建ST7735显示驱动
    pub fn build(self) -> Result<St7735Display, SpiError> {
        // 创建SPI设备
        let spi_device = EbdHalSpiDevice::new().ok_or(SpiError::NotInitialized)?;

        // 创建DC引脚
        let mut dc_pin = EbdHalGpio::new(self.config.dc_pin);
        dc_pin.set_high().map_err(|_| SpiError::GpioError)?;

        // 根据启用的特性创建不同的显示驱动
        #[cfg(feature = "st7735-lcd")]
        {
            // 创建RST引脚
            let rst_pin = if let Some(pin_num) = self.config.rst_pin {
                let mut pin = EbdHalGpio::new(pin_num);
                pin.set_high().map_err(|_| SpiError::GpioError)?;
                pin
            } else {
                // 使用虚拟RST引脚
                EbdHalGpio::new(0)
            };

            let display = ST7735::new(
                spi_device,
                dc_pin,
                rst_pin,
                self.config.rgb,
                self.config.inverted,
                self.config.width as u32,
                self.config.height as u32,
            );

            Ok(display)
        }

        #[cfg(feature = "st7735-lcd-doublebuffering")]
        {
            // 双缓冲版本不需要RST引脚
            let display = ST7735Buffered::new(
                spi_device,
                dc_pin,
                self.config.rgb,
                self.config.width as u32,
                self.config.height as u32,
            );

            Ok(display)
        }
    }
}

impl Default for St7735Builder {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷函数：初始化QSPI并创建显示驱动
pub fn init_display(config: St7735Config) -> Result<St7735Display, SpiError> {
    // 初始化QSPI
    ecos_ssc1::qspi::init_qspi(0);

    St7735Builder::new()
        .dc_pin(config.dc_pin)
        .rst_pin(config.rst_pin.unwrap_or(0))
        .size(config.width, config.height)
        .rgb(config.rgb)
        .inverted(config.inverted)
        .build()
}

/// 便捷函数：使用默认配置初始化显示
pub fn init_default_display() -> Result<St7735Display, SpiError> {
    init_display(St7735Config::default())
}

/// ST7735显示管理器（包含延迟对象）
pub struct St7735Manager {
    pub display: St7735Display,
    pub delay: EbdHalDelay,
}

impl St7735Manager {
    /// 创建显示管理器
    pub fn new(config: St7735Config) -> Result<Self, SpiError> {
        let display = init_display(config)?;
        let delay = EbdHalDelay;

        Ok(Self { display, delay })
    }

    /// 初始化显示驱动
    pub fn init(&mut self) -> Result<(), SpiError> {
        #[cfg(feature = "st7735-lcd")]
        {
            self.display
                .init(&mut self.delay)
                .map_err(|_| SpiError::TransferFailed)?;
            self.display.set_offset(2, 1);
        }

        #[cfg(feature = "st7735-lcd-doublebuffering")]
        {
            self.display
                .init(&mut self.delay, &Orientation::Portrait)
                .map_err(|_| SpiError::TransferFailed)?;
            self.display.set_offset(2, 3);
        }

        Ok(())
    }
}

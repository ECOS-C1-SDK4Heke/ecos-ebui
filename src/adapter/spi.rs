use core::{
    convert::From,
    option::Option,
    option::Option::{None, Some},
    result::Result,
    result::Result::Ok,
};

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiDevice};

use core::clone::Clone;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;
use core::prelude::rust_2024::derive;

use super::delay::EbdHalDelay;
use super::gpio::EbdHalGpio;
use ecos_ssc1::{Qspi, QspiError, qspi};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    Timeout,
    InvalidParameter,
    TransferFailed,
    GpioError,
    NotInitialized,
}

impl From<QspiError> for SpiError {
    fn from(error: QspiError) -> Self {
        match error {
            QspiError::Timeout => SpiError::Timeout,
            QspiError::InvalidParameter => SpiError::InvalidParameter,
            QspiError::TransferFailed => SpiError::TransferFailed,
        }
    }
}

impl embedded_hal::spi::Error for SpiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self {
            _ => embedded_hal::spi::ErrorKind::Other,
        }
    }
}

// ========== EbdHalSpiDevice ==========
pub struct EbdHalSpiDevice {
    qspi: &'static mut Qspi,
    cs_pin: Option<EbdHalGpio>,
}

impl EbdHalSpiDevice {
    /// 创建新的 SPI 设备 - 无片选
    pub fn new() -> Option<Self> {
        qspi::get_qspi().map(|qspi| Self { qspi, cs_pin: None })
    }

    /// 使用片选引脚创建 SPI 设备
    pub fn with_cs_pin(cs_pin_num: u32) -> Option<Self> {
        qspi::get_qspi().map(|qspi| {
            let mut cs_pin = EbdHalGpio::new(cs_pin_num);
            // 默认设置 CS 为高电平
            let _ = cs_pin.set_high();

            Self {
                qspi,
                cs_pin: Some(cs_pin),
            }
        })
    }

    /// 激活片选
    fn cs_select(&mut self) -> Result<(), SpiError> {
        if let Some(cs) = &mut self.cs_pin {
            cs.set_low().map_err(|_| SpiError::GpioError)
        } else {
            Ok(())
        }
    }

    /// 取消片选
    fn cs_deselect(&mut self) -> Result<(), SpiError> {
        if let Some(cs) = &mut self.cs_pin {
            cs.set_high().map_err(|_| SpiError::GpioError)
        } else {
            Ok(())
        }
    }

    /// 执行单个操作
    fn execute_operation(&mut self, operation: &mut Operation<'_, u8>) -> Result<(), SpiError> {
        match operation {
            Operation::Write(data) => {
                // 单个字节使用优化的写入
                if data.len() == 1 {
                    self.qspi.write_u8(data[0])?;
                } else {
                    self.qspi.write_bytes(data)?;
                }
                Ok(())
            }
            Operation::Read(buffer) => {
                // QSPI 主要支持写入，读取需要特殊处理
                // 发送虚拟数据并尝试读取
                for i in 0..buffer.len() {
                    // 发送虚拟字节
                    self.qspi.write_u8(0x00)?;
                    // 读取响应
                    let word = self.qspi.read_u32();
                    buffer[i] = (word >> 24) as u8; // 取最高字节
                }
                Ok(())
            }
            Operation::Transfer(read, write) => {
                // 写入数据
                if !write.is_empty() {
                    if write.len() == 1 {
                        self.qspi.write_u8(write[0])?;
                    } else {
                        self.qspi.write_bytes(write)?;
                    }
                }

                // 读取数据
                if !read.is_empty() {
                    for i in 0..read.len() {
                        // 发送虚拟字节
                        self.qspi.write_u8(0x00)?;
                        // 读取响应
                        let word = self.qspi.read_u32();
                        read[i] = (word >> 24) as u8;
                    }
                }
                Ok(())
            }
            Operation::TransferInPlace(buffer) => {
                // 复制数据到临时缓冲区
                let write_data = buffer.to_vec();

                // 写入数据
                if write_data.len() == 1 {
                    self.qspi.write_u8(write_data[0])?;
                } else {
                    self.qspi.write_bytes(&write_data)?;
                }

                // 读取数据到同一个缓冲区
                for i in 0..buffer.len() {
                    // 发送虚拟字节
                    self.qspi.write_u8(0x00)?;
                    // 读取响应
                    let word = self.qspi.read_u32();
                    buffer[i] = (word >> 24) as u8;
                }
                Ok(())
            }
            Operation::DelayNs(delay_ns) => {
                let mut timer: EbdHalDelay = EbdHalDelay {};
                Ok(timer.delay_ns(*delay_ns))
            }
        }
    }
}

impl ErrorType for EbdHalSpiDevice {
    type Error = SpiError;
}

impl SpiDevice<u8> for EbdHalSpiDevice {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        // 激活片选
        self.cs_select()?;

        // 执行所有操作
        for operation in operations.iter_mut() {
            self.execute_operation(operation)?;
        }

        // 等待所有传输完成
        self.qspi.wait_transfer_complete_full()?;

        // 取消片选
        self.cs_deselect()
    }
}

/// 创建 SPI 设备 - 无片选
pub fn create_spi_device() -> Option<EbdHalSpiDevice> {
    EbdHalSpiDevice::new()
}

/// 创建带片选的 SPI 设备
pub fn create_spi_device_with_cs(cs_pin_num: u32) -> Option<EbdHalSpiDevice> {
    EbdHalSpiDevice::with_cs_pin(cs_pin_num)
}

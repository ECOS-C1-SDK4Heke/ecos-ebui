#![no_std]
#![no_main]

use ecos_ssc1::ecos_main;
use embedded_graphics::pixelcolor::Bgr565;
use embedded_graphics::prelude::*;

use ecos_ebui::adapter::st7735::{St7735Config, St7735Manager};

#[ecos_main(tick)]
fn main() -> ! {
    loop {
        if let Some(c) = ecos_ssc1::uart::Uart::read_byte_nonblock() {
            if c as char == '\n' {
                println!("开始初始化！");
                break;
            }
        }
    }

    println!("ECOS ST7735 LCD 简单测试\n");

    // 初始化显示屏
    match init_display() {
        Ok(manager) => {
            println!("显示屏初始化成功！\n");
            println!("开始颜色循环测试...\n");
            color_loop_test(manager);
        }
        Err(e) => {
            println!("显示屏初始化失败: {:?}", e);
            println!("程序退出");
            loop {}
        }
    }
}

fn init_display() -> Result<St7735Manager, &'static str> {
    println!("初始化ST7735显示屏...");

    // 配置显示参数
    let config = St7735Config {
        dc_pin: 14,    // DC引脚
        rst_pin: None, // 无硬件复位
        width: 128,
        height: 128,
        rgb: false, // BGR模式
        inverted: false,
    };

    // 使用管理器创建显示驱动
    println!("创建显示管理器...");
    let mut manager = St7735Manager::new(config).map_err(|e| {
        println!("创建显示管理器失败: {:?}", e);
        "创建显示管理器失败"
    })?;

    // 初始化显示驱动
    println!("初始化显示驱动...");
    manager.init().map_err(|e| {
        println!("显示初始化失败: {:?}", e);
        "显示初始化失败"
    })?;

    println!("显示屏初始化完成");
    Ok(manager)
}

// 颜色循环测试
fn color_loop_test(mut manager: St7735Manager) -> ! {
    // 颜色序列：蓝、红、绿、白、黑、粉
    let colors = [
        ("蓝色", Bgr565::new(0, 0, 31)),   // 蓝色
        ("红色", Bgr565::new(31, 0, 0)),   // 红色
        ("绿色", Bgr565::new(0, 63, 0)),   // 绿色
        ("白色", Bgr565::new(31, 63, 31)), // 白色
        ("黑色", Bgr565::new(0, 0, 0)),    // 黑色
        ("粉色", Bgr565::new(31, 0, 31)),  // 粉色
    ];

    let mut cycle_count = 0;

    loop {
        println!("=== 第 {} 轮颜色循环 ===", cycle_count + 1);

        for (name, color) in &colors {
            println!("显示: {}", name);

            // 清屏显示颜色
            if let Err(e) = manager.display.clear((*color).into()) {
                println!("清屏失败: {:?}", e);
                continue;
            }

            // 如果是双缓冲版本，需要交换缓冲区
            #[cfg(feature = "st7735-lcd-doublebuffering")]
            {
                if let Err(e) = manager.display.swap_buffers() {
                    println!("交换缓冲区失败: {:?}", e);
                    continue;
                }
            }

            use embedded_hal::delay::DelayNs;
            manager.delay.delay_ms(1000);
        }

        cycle_count += 1;
        println!("\n");
    }
}

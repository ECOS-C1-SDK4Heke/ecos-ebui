#![no_std]
#![no_main]

use ecos_ssc1::{Timer, ecos_main, gpio, qspi};

#[ecos_main(tick)]
fn main() -> ! {
    println!("ST7735 QSPI 四色显示测试开始");

    // 1. 初始化QSPI - 使用新的API
    println!("初始化QSPI...");
    qspi::init_qspi(0); // 直接传入clkdiv值

    // 2. 获取QSPI实例 - 使用新的API
    println!("获取QSPI实例...");
    let qspi_instance = match qspi::get_qspi() {
        Some(qspi) => {
            println!("获取QSPI实例成功");
            qspi
        }
        None => {
            println!("获取QSPI失败: 未初始化");
            loop {}
        }
    };

    // 3. 配置DC引脚
    println!("配置DC引脚...");
    let dc_pin_num = 14; // 排针号14对应GPIO2

    // 使用gpio::GpioPin封装配置引脚
    gpio::GpioPin::config_pins(
        1 << (dc_pin_num - 1),
        ecos_ssc1::bindings::gpio_mode_t_GPIO_MODE_OUTPUT,
    );
    println!("DC引脚配置为输出模式 (pin {})", dc_pin_num);

    // 4. ST7735简单初始化
    println!("ST7735初始化中...");

    // 辅助函数：发送命令
    fn write_cmd(qspi: &mut qspi::Qspi, dc_pin: u32, cmd: u8) -> Result<(), qspi::QspiError> {
        gpio::GpioPin::set_level(dc_pin, false);
        qspi.write_u8(cmd)
    }

    // 辅助函数：发送数据
    fn write_data(qspi: &mut qspi::Qspi, dc_pin: u32, data: u8) -> Result<(), qspi::QspiError> {
        gpio::GpioPin::set_level(dc_pin, true);
        qspi.write_u8(data)
    }

    // 初始延迟
    println!("初始延迟...");
    Timer::delay_ms(120);

    // 睡眠退出
    println!("发送睡眠退出命令...");
    let _ = write_cmd(qspi_instance, dc_pin_num, 0x11);
    Timer::delay_ms(120);

    // 设置颜色模式 (16位RGB565)
    println!("设置颜色模式...");
    let _ = write_cmd(qspi_instance, dc_pin_num, 0x3A);
    let _ = write_data(qspi_instance, dc_pin_num, 0x05); // 65k模式

    // 设置显示方向（竖屏）
    println!("设置显示方向...");
    let _ = write_cmd(qspi_instance, dc_pin_num, 0x36);
    let _ = write_data(qspi_instance, dc_pin_num, 0xC8); // 竖屏

    // 开启显示
    println!("开启显示...");
    let _ = write_cmd(qspi_instance, dc_pin_num, 0x29);
    Timer::delay_ms(100);

    println!("ST7735初始化完成");

    // 5. 定义设置窗口函数
    fn set_window(
        qspi: &mut qspi::Qspi,
        dc_pin: u32,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
    ) -> Result<(), qspi::QspiError> {
        // 添加偏移（与之前C代码一致）
        let x0 = x0 + 2;
        let x1 = x1 + 2;
        let y0 = y0 + 3;
        let y1 = y1 + 3;

        // 设置列地址 (0x2A)
        write_cmd(qspi, dc_pin, 0x2A)?;
        write_data(qspi, dc_pin, (x0 >> 8) as u8)?;
        write_data(qspi, dc_pin, (x0 & 0xFF) as u8)?;
        write_data(qspi, dc_pin, (x1 >> 8) as u8)?;
        write_data(qspi, dc_pin, (x1 & 0xFF) as u8)?;

        // 设置行地址 (0x2B)
        write_cmd(qspi, dc_pin, 0x2B)?;
        write_data(qspi, dc_pin, (y0 >> 8) as u8)?;
        write_data(qspi, dc_pin, (y0 & 0xFF) as u8)?;
        write_data(qspi, dc_pin, (y1 >> 8) as u8)?;
        write_data(qspi, dc_pin, (y1 & 0xFF) as u8)?;

        // 开始内存写入 (0x2C)
        write_cmd(qspi, dc_pin, 0x2C)?;
        Ok(())
    }

    // 6. 定义颜色（16位RGB565）
    const COLOR_BLACK: u16 = 0x0000; // 黑色
    const COLOR_WHITE: u16 = 0xFFFF; // 白色
    const COLOR_RED: u16 = 0xF800; // 红色
    const COLOR_GREEN: u16 = 0x07E0; // 绿色

    // 7. 使用write_words优化显示性能
    println!("开始四色循环显示测试");
    let mut color_index = 0;

    // 预计算每个区域的数据
    let black_data = vec![COLOR_BLACK as u32; 64 * 64];
    let white_data = vec![COLOR_WHITE as u32; 64 * 64];
    let red_data = vec![COLOR_RED as u32; 64 * 64];
    let green_data = vec![COLOR_GREEN as u32; 64 * 64];

    loop {
        // 根据索引选择颜色模式
        match color_index % 4 {
            0 => {
                println!("当前颜色：黑白红绿");
                // 左上黑 (0-63, 0-63)
                set_window(qspi_instance, dc_pin_num, 0, 0, 63, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&black_data).unwrap();

                // 右上白 (64-127, 0-63)
                set_window(qspi_instance, dc_pin_num, 64, 0, 127, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&white_data).unwrap();

                // 左下红 (0-63, 64-127)
                set_window(qspi_instance, dc_pin_num, 0, 64, 63, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&red_data).unwrap();

                // 右下绿 (64-127, 64-127)
                set_window(qspi_instance, dc_pin_num, 64, 64, 127, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&green_data).unwrap();
            }
            1 => {
                println!("当前颜色：绿黑白红");
                // 左上绿 (0-63, 0-63)
                set_window(qspi_instance, dc_pin_num, 0, 0, 63, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&green_data).unwrap();

                // 右上黑 (64-127, 0-63)
                set_window(qspi_instance, dc_pin_num, 64, 0, 127, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&black_data).unwrap();

                // 左下白 (0-63, 64-127)
                set_window(qspi_instance, dc_pin_num, 0, 64, 63, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&white_data).unwrap();

                // 右下红 (64-127, 64-127)
                set_window(qspi_instance, dc_pin_num, 64, 64, 127, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&red_data).unwrap();
            }
            2 => {
                println!("当前颜色：红绿黑白");
                // 左上红 (0-63, 0-63)
                set_window(qspi_instance, dc_pin_num, 0, 0, 63, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&red_data).unwrap();

                // 右上绿 (64-127, 0-63)
                set_window(qspi_instance, dc_pin_num, 64, 0, 127, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&green_data).unwrap();

                // 左下黑 (0-63, 64-127)
                set_window(qspi_instance, dc_pin_num, 0, 64, 63, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&black_data).unwrap();

                // 右下白 (64-127, 64-127)
                set_window(qspi_instance, dc_pin_num, 64, 64, 127, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&white_data).unwrap();
            }
            3 => {
                println!("当前颜色：白红绿黑");
                // 左上白 (0-63, 0-63)
                set_window(qspi_instance, dc_pin_num, 0, 0, 63, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&white_data).unwrap();

                // 右上红 (64-127, 0-63)
                set_window(qspi_instance, dc_pin_num, 64, 0, 127, 63).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&red_data).unwrap();

                // 左下绿 (0-63, 64-127)
                set_window(qspi_instance, dc_pin_num, 0, 64, 63, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&green_data).unwrap();

                // 右下黑 (64-127, 64-127)
                set_window(qspi_instance, dc_pin_num, 64, 64, 127, 127).unwrap();
                gpio::GpioPin::set_level(dc_pin_num, true);
                qspi_instance.write_words(&black_data).unwrap();
            }
            _ => {}
        }

        // 颜色索引递增，实现循环
        color_index += 1;

        // 等待2秒
        Timer::delay_ms(2000);
    }
}

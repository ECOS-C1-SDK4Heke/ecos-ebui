#![no_std]
#![no_main]

use ecos_ssc1::bindings::*;
use ecos_ssc1::ecos_main;

#[ecos_main(tick)]
fn main() -> ! {
    println!("ST7735 QSPI 四色显示测试开始");

    // 1. 初始化QSPI
    let mut qspi_config = qspi_config_t { clkdiv: 0 };
    unsafe {
        qspi_init(&mut qspi_config);
    }
    println!("QSPI初始化完成");

    // 2. 配置GPIO2为输出模式（DC引脚）
    let dc_pin = gpio_num_t_GPIO_NUM_2;
    let gpio_cfg = gpio_config_t {
        pin_bit_mask: 1 << 2, // GPIO2
        mode: gpio_mode_t_GPIO_MODE_OUTPUT,
    };

    unsafe {
        gpio_config(&gpio_cfg);
    }
    println!("GPIO2配置为输出模式（DC引脚）");

    // 3. ST7735简单初始化
    println!("ST7735初始化中...");

    // 辅助函数：发送命令
    fn write_cmd(dc_pin: gpio_num_t, cmd: u8) {
        unsafe {
            gpio_set_level(dc_pin, 0);
            qspi_write_8(cmd);
        }
    }

    // 辅助函数：发送数据
    fn write_data(dc_pin: gpio_num_t, data: u8) {
        unsafe {
            gpio_set_level(dc_pin, 1);
            qspi_write_8(data);
        }
    }

    // 初始延迟
    ecos_ssc1::Timer::delay_ms(120);

    // 睡眠退出
    write_cmd(dc_pin, 0x11);
    ecos_ssc1::Timer::delay_ms(120);

    // 设置颜色模式 (16位RGB565)
    write_cmd(dc_pin, 0x3A);
    write_data(dc_pin, 0x05); // 65k模式

    // 设置显示方向（竖屏）
    write_cmd(dc_pin, 0x36);
    write_data(dc_pin, 0xC8); // 竖屏

    // 开启显示
    write_cmd(dc_pin, 0x29);
    ecos_ssc1::Timer::delay_ms(100);

    println!("ST7735初始化完成");

    // 4. 定义设置窗口函数
    fn set_window(dc_pin: gpio_num_t, x0: u16, y0: u16, x1: u16, y1: u16) {
        // 添加偏移（与之前C代码一致）
        let x0 = x0 + 2;
        let x1 = x1 + 2;
        let y0 = y0 + 3;
        let y1 = y1 + 3;

        // 设置列地址 (0x2A)
        write_cmd(dc_pin, 0x2A);
        write_data(dc_pin, (x0 >> 8) as u8);
        write_data(dc_pin, (x0 & 0xFF) as u8);
        write_data(dc_pin, (x1 >> 8) as u8);
        write_data(dc_pin, (x1 & 0xFF) as u8);

        // 设置行地址 (0x2B)
        write_cmd(dc_pin, 0x2B);
        write_data(dc_pin, (y0 >> 8) as u8);
        write_data(dc_pin, (y0 & 0xFF) as u8);
        write_data(dc_pin, (y1 >> 8) as u8);
        write_data(dc_pin, (y1 & 0xFF) as u8);

        // 开始内存写入 (0x2C)
        write_cmd(dc_pin, 0x2C);
    }

    // 5. 定义颜色（16位RGB565）
    const COLOR_BLACK: u16 = 0x0000; // 黑色
    const COLOR_WHITE: u16 = 0xFFFF; // 白色
    const COLOR_RED: u16 = 0xF800; // 红色
    const COLOR_GREEN: u16 = 0x07E0; // 绿色

    // 6. 四色显示循环
    println!("开始四色循环显示测试");
    let mut color_index = 0;

    loop {
        // 根据索引选择颜色模式
        match color_index % 4 {
            0 => {
                println!("当前颜色：黑白红绿");
                // 填充整个屏幕：左上黑，右上白，左下红，右下绿
                // 左上黑 (0-63, 0-63)
                set_window(dc_pin, 0, 0, 63, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_BLACK);
                    }
                }

                // 右上白 (64-127, 0-63)
                set_window(dc_pin, 64, 0, 127, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_WHITE);
                    }
                }

                // 左下红 (0-63, 64-127)
                set_window(dc_pin, 0, 64, 63, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_RED);
                    }
                }

                // 右下绿 (64-127, 64-127)
                set_window(dc_pin, 64, 64, 127, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_GREEN);
                    }
                }
            }
            1 => {
                println!("当前颜色：绿黑白红");
                // 左上绿，右上黑，左下白，右下红
                // 左上绿
                set_window(dc_pin, 0, 0, 63, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_GREEN);
                    }
                }

                // 右上黑
                set_window(dc_pin, 64, 0, 127, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_BLACK);
                    }
                }

                // 左下白
                set_window(dc_pin, 0, 64, 63, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_WHITE);
                    }
                }

                // 右下红
                set_window(dc_pin, 64, 64, 127, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_RED);
                    }
                }
            }
            2 => {
                println!("当前颜色：红绿黑白");
                // 左上红，右上绿，左下黑，右下白
                // 左上红
                set_window(dc_pin, 0, 0, 63, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_RED);
                    }
                }

                // 右上绿
                set_window(dc_pin, 64, 0, 127, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_GREEN);
                    }
                }

                // 左下黑
                set_window(dc_pin, 0, 64, 63, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_BLACK);
                    }
                }

                // 右下白
                set_window(dc_pin, 64, 64, 127, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_WHITE);
                    }
                }
            }
            3 => {
                println!("当前颜色：白红绿黑");
                // 左上白，右上红，左下绿，右下黑
                // 左上白
                set_window(dc_pin, 0, 0, 63, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_WHITE);
                    }
                }

                // 右上红
                set_window(dc_pin, 64, 0, 127, 63);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_RED);
                    }
                }

                // 左下绿
                set_window(dc_pin, 0, 64, 63, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_GREEN);
                    }
                }

                // 右下黑
                set_window(dc_pin, 64, 64, 127, 127);
                unsafe {
                    gpio_set_level(dc_pin, 1);
                    for _ in 0..(64 * 64) {
                        qspi_write_16(COLOR_BLACK);
                    }
                }
            }
            _ => {}
        }

        // 颜色索引递增，实现循环
        color_index += 1;

        // 等待2秒
        ecos_ssc1::Timer::delay_ms(2000);
    }
}

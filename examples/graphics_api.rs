#![no_std]
#![no_main]

use ecos_ssc1::ecos_main;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::Text,
};

use ecos_ebui::{St7735Config, St7735Manager};

#[ecos_main(tick)]
fn main() -> ! {
    println!("ECOS embedded-graphics 演示\n");

    // 配置显示参数
    let config = St7735Config {
        dc_pin: 14,
        rst_pin: None,
        width: 128,
        height: 128,
        rgb: true, // 使用 RGB 模式
        inverted: false,
    };

    // 创建显示管理器
    let mut manager = match St7735Manager::new(config) {
        Ok(manager) => {
            println!("显示管理器创建成功");
            manager
        }
        Err(e) => {
            println!("显示管理器创建失败: {:?}", e);
            loop {}
        }
    };

    // 初始化显示
    match manager.init() {
        Ok(_) => println!("显示初始化成功！"),
        Err(e) => {
            println!("显示初始化失败: {:?}", e);
            loop {}
        }
    }

    println!("开始 embedded-graphics 演示...\n");

    loop {
        // 演示1: 基础图形
        basic_shapes_demo(&mut manager);

        // 演示2: 文本和动画
        text_and_animation_demo(&mut manager);

        // 演示3: 动态效果
        dynamic_effects_demo(&mut manager);
    }
}

// 演示1: 基础图形
fn basic_shapes_demo(manager: &mut St7735Manager) {
    use embedded_hal::delay::DelayNs;

    println!("=== 演示1: 基础图形 ===");

    // 1. 清屏为黑色
    manager.display.clear(Rgb565::BLACK).unwrap();
    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(1000);

    // 2. 绘制红色填充矩形
    println!("绘制红色矩形...");
    Rectangle::new(Point::new(10, 10), Size::new(40, 30))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut manager.display)
        .unwrap();

    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(1000);

    // 3. 绘制绿色填充圆形
    println!("绘制绿色圆形...");
    Circle::new(Point::new(90, 40), 15)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
        .draw(&mut manager.display)
        .unwrap();

    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(1000);

    // 4. 绘制蓝色填充三角形
    println!("绘制蓝色三角形...");
    Triangle::new(Point::new(20, 60), Point::new(60, 60), Point::new(40, 100))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut manager.display)
        .unwrap();

    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(1000);

    // 5. 绘制黄色边框矩形
    println!("绘制黄色边框矩形...");
    Rectangle::new(Point::new(70, 80), Size::new(40, 40))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::YELLOW, 2))
        .draw(&mut manager.display)
        .unwrap();

    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(1000);
}

// 演示2: 文本和动画
fn text_and_animation_demo(manager: &mut St7735Manager) {
    use embedded_hal::delay::DelayNs;

    println!("=== 演示2: 文本和动画 ===");

    // 清屏为深蓝色
    manager.display.clear(Rgb565::new(0, 0, 15)).unwrap();
    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(500);

    // 绘制文本
    println!("绘制文本...");
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    Text::new("ECOS", Point::new(10, 20), text_style)
        .draw(&mut manager.display)
        .unwrap();

    Text::new("ST7735", Point::new(10, 35), text_style)
        .draw(&mut manager.display)
        .unwrap();

    Text::new("Graphics", Point::new(10, 50), text_style)
        .draw(&mut manager.display)
        .unwrap();

    Text::new("Demo", Point::new(10, 65), text_style)
        .draw(&mut manager.display)
        .unwrap();

    #[cfg(feature = "st7735-lcd-doublebuffering")]
    manager.display.swap_buffers().unwrap();

    manager.delay.delay_ms(1000);

    // 简单动画：移动的方块
    println!("移动方块动画...");
    for x in 0..100 {
        // 清屏
        manager.display.clear(Rgb565::new(0, 0, 15)).unwrap();

        // 重新绘制文本
        Text::new("ECOS", Point::new(10, 20), text_style)
            .draw(&mut manager.display)
            .unwrap();

        // 绘制移动的方块
        Rectangle::new(Point::new(x as i32, 80), Size::new(20, 20))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
            .draw(&mut manager.display)
            .unwrap();

        #[cfg(feature = "st7735-lcd-doublebuffering")]
        manager.display.swap_buffers().unwrap();

        manager.delay.delay_ms(30);
    }

    manager.delay.delay_ms(1000);
}

// 演示3: 动态效果
fn dynamic_effects_demo(manager: &mut St7735Manager) {
    use embedded_hal::delay::DelayNs;

    println!("=== 演示3: 动态效果 ===");

    // 彩虹渐变效果
    println!("彩虹渐变效果...");

    for frame in 0..64 {
        // 清屏
        manager.display.clear(Rgb565::BLACK).unwrap();

        // 绘制彩虹条
        for i in 0..8 {
            let y = i * 16;
            let color = match i {
                0 => Rgb565::RED,
                1 => Rgb565::new(31, 16, 0), // 橙色
                2 => Rgb565::YELLOW,
                3 => Rgb565::GREEN,
                4 => Rgb565::CYAN,
                5 => Rgb565::BLUE,
                6 => Rgb565::new(16, 0, 31), // 紫色
                _ => Rgb565::MAGENTA,
            };

            Rectangle::new(Point::new(0, y as i32), Size::new(128, 16))
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(&mut manager.display)
                .unwrap();
        }

        // 绘制移动的光标
        let cursor_x = (frame * 2) % 128;
        Rectangle::new(Point::new(cursor_x as i32, 0), Size::new(4, 128))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(&mut manager.display)
            .unwrap();

        #[cfg(feature = "st7735-lcd-doublebuffering")]
        manager.display.swap_buffers().unwrap();

        manager.delay.delay_ms(50);
    }

    manager.delay.delay_ms(1000);
}

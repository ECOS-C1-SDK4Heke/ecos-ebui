# ecos-ebui

> cargo ecos build [--release]

示例程序位于example

其中，实现了ebd-hal层，所有兼容ebd的显示器（drawable trait）都可以直接接入ebd-graphics方便的绘制图形

给出了示例的兼容层`st7735`，可以发现就是使用ebd-hal直接包装init即可实现显示

其余支持的屏幕直接可见：[ebd-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/)文档里面提到的兼容的驱动，大致20+款，或者自己直接查实现了drawable的已有的驱动

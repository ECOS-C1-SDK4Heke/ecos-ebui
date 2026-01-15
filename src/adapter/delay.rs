use ecos_ssc1::Timer;
use embedded_hal::delay::DelayNs;

pub struct EbdHalDelay;

impl DelayNs for EbdHalDelay {
    fn delay_ns(&mut self, ns: u32) {
        // 70MHz ns 级别支持不能
        let us = (ns + 999) / 1_000;
        if us > 0 {
            Timer::delay_us(us);
        }
    }

    fn delay_us(&mut self, us: u32) {
        Timer::delay_us(us);
    }

    fn delay_ms(&mut self, ms: u32) {
        Timer::delay_ms(ms);
    }
}

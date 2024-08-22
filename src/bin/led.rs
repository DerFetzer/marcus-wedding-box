#![deny(unsafe_code)]
#![no_main]
#![no_std]

use marcus_wedding_box as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use hal::{gpio::NoPin, pac, prelude::*};
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use stm32f4xx_hal::{
    self as hal,
    hal::{
        delay::DelayNs,
        spi::{Mode, Phase, Polarity},
    },
};
use ws2812_spi as ws2812;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("cannot take peripherals");

    // Configure APB bus clock to 48 MHz, cause ws2812b requires 3 Mbps SPI
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = dp.TIM1.delay_us(&clocks);
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let spi = dp.SPI1.spi(
        (gpioa.pa5, NoPin::new(), gpiob.pb5),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnSecondTransition,
        },
        3000.kHz(),
        &clocks,
    );

    const NUM_LEDS: usize = 41;
    let mut buffer = [0; NUM_LEDS * 12 + 40];
    let mut ws = ws2812::prerendered::Ws2812::new(spi, buffer.as_mut_slice());

    // Wait before start write for synchronization
    delay.delay_ms(1);

    loop {
        for j in 0..(256 * 5) {
            let data = (0..NUM_LEDS)
                .map(|i| wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8));
            ws.write(brightness(data, 32)).unwrap();
            delay.delay(10.millis());
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}

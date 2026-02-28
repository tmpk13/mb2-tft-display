#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_graphics::{
    Drawable,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use gc9a01::{self, mode::DisplayConfiguration};
use microbit::hal::{
    Spim,
    gpio::Level,
    spim::{self, Frequency},
    timer::Timer,
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();

    let mut timer0 = Timer::new(board.TIMER0);

    // Setup SPI
    let sck = board.pins.p0_17.into_push_pull_output(Level::Low).degrade();
    let coti = board.pins.p0_13.into_push_pull_output(Level::Low).degrade();

    let dc = board.edge.e08.into_push_pull_output(Level::Low);
    let cs = board.edge.e01.into_push_pull_output(Level::Low);
    let mut rst = board.edge.e09.into_push_pull_output(Level::High);

    let spi_bus = Spim::new(
        board.SPIM0,
        microbit::hal::spim::Pins {
            sck: Some(sck),
            mosi: Some(coti),
            miso: None,
        },
        Frequency::M8,
        spim::MODE_0,
        0xFF, // ORC overflow character
    );
    let spi = display_interface_spi::SPIInterface::new(
        ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap(),
        dc,
    );

    // Setup gc9a01 display
    let mut display = gc9a01::Gc9a01::new(
        spi,
        gc9a01::prelude::DisplayResolution240x240,
        gc9a01::prelude::DisplayRotation::Rotate180,
    );
    display.reset(&mut rst, &mut timer0);
    display.init(&mut timer0).unwrap();

    // Call `embedded_graphics` `clear()` trait method
    <_ as embedded_graphics::draw_target::DrawTarget>::clear(&mut display, Rgb565::WHITE).unwrap();

    let rect = |color| {
        // make small rect
        let rect_style = PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        Rectangle::new(
            Point { x: 70, y: 70 },
            Size {
                width: 100,
                height: 100,
            },
        )
        .into_styled(rect_style)
    };

    let rects = [rect(Rgb565::BLUE), rect(Rgb565::RED)];

    for i in 0.. {
        // Draw
        rects[i & 1].draw(&mut display).unwrap();

        // Hold
        timer0.delay_ms(1000);
    }
    unsafe { core::hint::unreachable_unchecked() }
}

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_graphics::{
    Drawable,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use gc9a01::{self, mode::DisplayConfiguration};
use microbit::hal::{
    gpio::{p0::Parts, Level},
    spim::{self, Frequency},
    timer::Timer,
    Spim,
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = microbit::pac::Peripherals::take().unwrap();

    // Put port 0 pins into gpio list
    let port0 = Parts::new(peripherals.P0);
    let mut timer0 = Timer::new(peripherals.TIMER0);

    // Setup SPI 
    let sck = port0.p0_17.into_push_pull_output(Level::Low).degrade();
    let coti = port0.p0_13.into_push_pull_output(Level::Low).degrade();

    let dc = port0.p0_10.into_push_pull_output(Level::Low);
    let cs = port0.p0_12.into_push_pull_output(Level::Low);
    let mut rst = port0.p0_09.into_push_pull_output(Level::High);

    let spi_bus = Spim::new(
        peripherals.SPIM0,
        microbit::hal::spim::Pins {
            sck: Some(sck),
            mosi: Some(coti),
            miso: None,
        },
        Frequency::M16,
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
    display.clear().unwrap();


    // Conflict with display.clear from embedded graphics
    // Using rect as background
    let bg_rect_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::WHITE)
        .build();
    Rectangle::new(
        Point { x: 0, y: 0 },
        Size {
            width: 240,
            height: 240,
        },
        )
        .into_styled(bg_rect_style)
        .draw(&mut display)
        .unwrap();

    // Draw small rect
    let rect_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();
    Rectangle::new(
        Point { x: 70, y: 70 },
        Size {
            width: 100,
            height: 100,
        },
        )
        .into_styled(rect_style)
        .draw(&mut display)
        .unwrap();


    loop {}
}

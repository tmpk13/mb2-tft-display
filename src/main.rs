#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_graphics::{
    Drawable,
    pixelcolor::Rgb565,
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use gc9a01::{mode::DisplayConfiguration, Gc9a01};
use microbit::hal::{
    gpio::{p0::Parts, Level},
    spim::{self, Frequency},
    timer::Timer,
    Spim,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let peripherals = microbit::pac::Peripherals::take().unwrap();

    // Put port 0 pins into gpio list
    let port0 = Parts::new(peripherals.P0);
    let mut timer0 = Timer::new(peripherals.TIMER0);

    // Setup serial line
    let sck = port0.p0_17.into_push_pull_output(Level::Low).degrade();
    let mosi = port0.p0_13.into_push_pull_output(Level::Low).degrade();

    let dc = port0.p0_10.into_push_pull_output(Level::Low);
    let cs = port0.p0_12.into_push_pull_output(Level::Low);
    let mut rst = port0.p0_09.into_push_pull_output(Level::Low);

    let spi_bus = Spim::new(
        peripherals.SPIM0,
        microbit::hal::spim::Pins {
            sck: Some(sck),
            mosi: Some(mosi),
            miso: None,
        },
        Frequency::M8,
        spim::MODE_0,
        0xFF,
    );

    let spi = display_interface_spi::SPIInterface::new(
        ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap(),
        dc,
    );

    let mut display = Gc9a01::new(
        spi,
        gc9a01::prelude::DisplayResolution240x240,
        gc9a01::prelude::DisplayRotation::Rotate180,
    ).into_buffered_graphics();
    display.reset(&mut rst, &mut timer0);
    display.init(&mut timer0).unwrap();
    display.clear();

    let rect_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();
    Rectangle::new(
        Point { x: 20, y: 20 },
        Size {
            width: 200,
            height: 200,
        },
        )
        .into_styled(rect_style)
        .draw(&mut display)
        .unwrap();

    loop {}
}

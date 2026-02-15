#![no_main]
#![no_std]
// Referenced https://github.com/yuri91/ili9341-rs for SPI setup

use cortex_m_rt::entry;
use embedded_hal_bus::spi::{ExclusiveDevice};
use gc9a01::{Gc9a01, mode::DisplayConfiguration};
use microbit::{
    hal::{
        Spim, gpio::{Level, p0::{Parts}}, spim::{self, Frequency}, timer::Timer
    }
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
        0xFF
    ); 

    let spi = display_interface_spi::SPIInterface::new( 
        ExclusiveDevice::new_no_delay( 
            spi_bus, 
            cs, 
        ).unwrap(), 
        dc, 
    ); 

    let mut display = Gc9a01::new( 
        spi, 
        gc9a01::prelude::DisplayResolution240x240, 
        gc9a01::prelude::DisplayRotation::Rotate180, 
    ); 
    display.reset(&mut rst, &mut timer0); 
    display.init(&mut timer0).unwrap(); 

    let mut grad: [u16; 240*240] = [0; 240*240]; 
    
    fn gradient(grad: &mut [u16; 240*240], start: usize, end: usize, shift: usize) {
        for i in start..end { 
            for j in 0..240 { 
                grad[i * 240 + j] = (((end - i) * 32 / (end - start)) as u16) << shift;
            } 
        } 
    }

    gradient(&mut grad, 0, 240/3, 0);
    gradient(&mut grad, 240/3, (240/3)*2, 5);
    gradient(&mut grad, (240/3)*2, 240, 11);

    match display.bounded_draw(&grad, 240, (0, 0), (239, 239)) {
        Ok(_) => {}
        Err(e) => { rprintln!("{:?}", e); }
    }

    loop {}
}

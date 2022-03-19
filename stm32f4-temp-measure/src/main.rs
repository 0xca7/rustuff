#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use cortex_m::Peripherals as _core_periph;

use crate::hal::{pac, prelude::*};
use stm32f4xx_hal::serial::{config, Serial};
use stm32f4xx_hal::delay::Delay;
use crate::hal::i2c::I2c;

/// slave address of the MLX90614
const SLAVE_ADDR:   u8 = 0x5a;

/// temperature registers address
const OBJ_TEMP_REG: [u8;1] = [0x07];

#[entry]
fn main() -> ! {

    // peripheral access crate periphs
    let dp = pac::Peripherals::take().unwrap();
    // core peripherals
    let cp = _core_periph::take().unwrap();

    // get some peripheral handles
    
    // we will use gpioa for the UART TX 
    let gpioa = dp.GPIOA.split();
    // gpiob for i2c
    let gpiob = dp.GPIOB.split();
    // rcc for clock control
    let rcc = dp.RCC.constrain();

    // set the clock which we want to use including the frequency
    // we'll set 8 MHz here
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    // this configures the system timer as a delay provider
    let mut delay = Delay::new(cp.SYST, &clocks);

    // configure the onboard LED
    let mut onboard_led = gpioa
        .pa5
        .into_push_pull_output();
    onboard_led.set_low();

    // configure the I2C SCL and SDA lines
    let scl = gpiob
        .pb6
        .into_alternate()
        .internal_pull_up(true)
        .set_open_drain();
    let sda = gpiob
        .pb7
        .into_alternate()
        .internal_pull_up(true)
        .set_open_drain();

    // set up i2c
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 100.khz(), &clocks);

    // set alternate function mode for GPIOA pin 9
    let tx_pin = gpioa.pa9.into_alternate();

    // configure the USART. use the default, with a baudrate of 9600
    let usart_config = config::Config::default().baudrate(9600.bps());

    // now get a serial tx we can write to, implement the config on USART1
    let mut tx = Serial::tx(dp.USART1, tx_pin, usart_config, &clocks).unwrap();

    // buffer for i2c, 16 bits in total
    let mut buffer = [0u8;2];

    // buffer for uart TX
    let mut uart_tx_buffer = [0u8;4];
    // add carriage return and newline
    uart_tx_buffer[2] = 0x0d;
    uart_tx_buffer[3] = 0x0a;

    loop {

        // don't spam to the UART all too much
        delay.delay_ms(1_00_u16);

        // write the address we want to read from, read the data returned
        match i2c.write_read(SLAVE_ADDR, &OBJ_TEMP_REG, &mut buffer) {
            Ok(()) => (),
            Err(_e) => {
                onboard_led.set_high();
            },   
        }
        
        // load the data we read from the sensor into the buffer for UART
        uart_tx_buffer[0] = buffer[0];
        uart_tx_buffer[1] = buffer[1];

        // write to the UART 
        match tx.bwrite_all(&uart_tx_buffer) {
            Ok(()) => (),
            Err(_e) => {
                onboard_led.set_high();
            },
        }

    } // loop

}
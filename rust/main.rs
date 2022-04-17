
// rust demo of dcc train protocol
// demo in rust to generate dcc packet using raspberry pi pico
// see nmra for docs -> https://www.nmra.org/sites/default/files/s-92-2004-07.pdf
// I've included a video with this repository as proof of concept
#![no_std]
#![no_main]

mod dcc;

use cortex_m_rt::entry;
use embedded_time::rate::*;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;
use rp_pico::hal::gpio::{ FunctionUart};
use rp_pico::hal::uart::{UartPeripheral, common_configs};

// found on stack overflow
fn as_u32_be_lower(array: &[u8; 8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}
fn as_u32_be_upper(array: &[u8; 8]) -> u32 {
    ((array[4] as u32) << 24) +
    ((array[5] as u32) << 16) +
    ((array[6] as u32) <<  8) +
    ((array[7] as u32) <<  0)
}

// function assemble_packet:
//     you supply locomotive address and data byte (direction, speed)
// this function will generate dcc packet consisting of 8 bytes,
// see nmra for dcc packet format: https://www.nmra.org/sites/default/files/s-92-2004-07.pdf
// I've chosen 8 bytes to align to two 32 bits (4 bytes) for the pico pio tx fifo buffer
// 8 bytes format as follows:
// 1st byte - 0xff (generates eight 1 bits, parts of preamble)
// 2nd byte - 0xfe (generates seven 1 bits (still preamble), last lsb bit is zero which is packet start bit (see nmra docs)
// 3rd byte - 0x?? address of the locomotive, msb must be 0 (this version is only good for 1 byte addressing)
// 4th byte - 0x?? bit 7 must be 0 (start of data byte start bit)
//                 bits 6 and 5 must be 0b01 (part of data byte, see nmra docs)
//                 bits 4,3,2,1,0 (part of data byte)
// 5th byte - 0x?? bit 7 is the lsb bit of data byte
//                 bit 6 must be 0 is the start bit of checksum
//                 bits 5,4,3,2,1,0 is part of the checksum (address xor data byte)
// 6th byte - 0x?? bits 7 and 6 are the lsb bits of the checksum from 5th byte
//                 bit 5 must be a 1 and this is the end of the packet
//                 rest of the bits are don't care bits all the way to the 8th byte
fn assemble_packet(address: u8, data: u8) -> (u32, u32){
    let checksum = address ^ data;
    let mut packet:[u8; 8] = [0xff,0xfe,address,0x00,0x00,0x00,0x00,0x00];
    packet[3] |= (data >> 1) << 0;
    packet[4] |= ((data << 6) << 6) | (0b0 << 5) | ((checksum >> 2) << 0);
    packet[5] |= (checksum << 6) << 0;
    packet[5] |= 0x20; // end of packet bit
    let w1 = as_u32_be_lower(&packet);
    let w2 = as_u32_be_upper(&packet); 
    (w1, w2)
}

#[entry]
fn main() -> ! {
    // hardware setup
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,).ok().unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,);

        let tx = pins.gpio12.into_mode::<FunctionUart>();
        let rx = pins.gpio13.into_mode::<FunctionUart>();
        
        let mut uart = UartPeripheral::<_, _, _>::new(pac.UART0, (tx, rx), &mut pac.RESETS)
            .enable(
                common_configs::_115200_8_N_1,
                clocks.peripheral_clock.into(),
            )
            .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);    
    let mut tx = dcc::init(
        pins.gpio15.into_mode(), 
        &mut pio, 
        sm0,
        clocks.peripheral_clock.freq(),);
    
    let address = 50;
    let forward = 0x76u8;
    let reverse = 0x56u8;

    loop {
        
       // forward packet
       let (w1, w2) = assemble_packet(address, forward);
       tx.write(w1);
       tx.write(w2);
       delay.delay_ms(5);
       tx.write(w1);
       tx.write(w2);     
       delay.delay_ms(3000);
       // delay should be idle packet every 5 millisecond

       // reverse packet
       let (w1, w2) = assemble_packet(address, reverse);
       tx.write(w1);
       tx.write(w2);
       delay.delay_ms(5);
       tx.write(w1);
       tx.write(w2);        
       delay.delay_ms(3000);
    }}

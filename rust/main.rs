////////////// filename: main.rs
// demo of rust pico dcc using bluetooth
// dcc booster is hooked up on gpio 15
// 
// hc-19 ble vcc - pico 5v, ground - pico any ground pin
// hc-19 tx - pico gpio13
// hc-19 rx - pico gpio12
// I changed the baud rate for the hc-19 to 115200
//
// rust install on a fresh linux liteos
//    -- https://www.linuxliteos.com/
//
// booster from www.pololu.com
//   ---- https://www.pololu.com/product/2136
//   booster hook up
//   vdd   - pico +3.3v
//   brk   - nc
//   slp   - nc
//   dir   - pico gpio15
//   pwm   - pico +3.3v
//   gnd   - any pico gnd
//   out+  - dcc rail
//   out-  - dcc rail
//   vmm   - +12v
//   cs    - nc
//   fault - nc
/* 
sudo apt install git gdb-multiarch
    
sudo apt install automake autoconf build-essential texinfo libtool libftdi-dev libusb-1.0-0-dev libudev-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc -- version (to check if rust installs successfully)

rustup update

rustup target install thumbv6m-none-eabi (this is for pico)

cargo install flip-link

cargo install elf2uf2-rs --locked
*/

#![no_std]
#![no_main]

use panic_halt as _;
mod setup;

#[rtic::app(device = rp_pico::hal::pac, dispatchers = [XIP_IRQ])]
mod app {
   
    use crate::setup::setup;
    use crate::setup::LedPin;
    use crate::setup::UartType;
    use crate::setup::PioTx;
    use embedded_hal::digital::v2::ToggleableOutputPin;
    use rp2040_monotonic::*;
    
    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Monotonic = Rp2040Monotonic;

    #[shared]
    struct Shared {uart: UartType, pio_tx: PioTx}

    #[local]
    struct Local {led: LedPin}

    #[init(local = [])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (mono, led, uart, pio_tx) = setup(cx.device, cx.core);

        led_blinker::spawn().ok();

        (
            Shared {uart, pio_tx},
            Local {led},
            init::Monotonics(mono),
        )
    }

    // toggle led - shows we're stil alive
    #[task(local = [led])]
    fn led_blinker(cx: led_blinker::Context) {
        cx.local.led.toggle().ok();
        led_blinker::spawn_after(500.millis()).ok();
    }
    
    // get a char on serial port and change the w2812 color
    // base on the choice
    #[task(binds = UART0_IRQ, priority = 2, shared = [uart, pio_tx])]
    fn on_rx(cx: on_rx::Context){

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

      let mut data = [0u8; 4];
      let uart = cx.shared.uart;
      let pio_tx = cx.shared.pio_tx;

      (uart, pio_tx).lock(|uart_a, pio_tx_a|{
        match uart_a.read_full_blocking(&mut data){
          Err(_e) => {}
          Ok(_count) => {
 
            let (w1,w2) = assemble_packet(data[1], data[2]);
            pio_tx_a.write(w1);
            pio_tx_a.write(w2);
            pio_tx_a.write(w1);
            pio_tx_a.write(w2);

          }
        }
      });
    }
}

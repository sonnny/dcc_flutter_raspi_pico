/////////////////// filename: setup.rs
use rp2040_monotonic::Rp2040Monotonic;
use embedded_time::fixed_point::FixedPoint;
use rp_pico::{
    hal::{
        clocks::{Clock, init_clocks_and_plls,},
        gpio::{pin::bank0::*, Pin, FunctionPio0, FunctionUart, Pins, PushPullOutput},
        pac,
        pac::{UART0,PIO0},
        watchdog::Watchdog,
        Sio,
        uart::{UartPeripheral, Enabled, common_configs},
        pio::{PIOExt, ShiftDirection,PIOBuilder, Tx, SM0, PinDir,},
    },
    XOSC_CRYSTAL_FREQ,
};

type UartTx = Pin<Gpio12, FunctionUart>;
type UartRx = Pin<Gpio13, FunctionUart>;

pub type PioTx = Tx<(PIO0,SM0)>;
pub type LedPin = Pin<Gpio25, PushPullOutput>;
pub type UartType = UartPeripheral<Enabled, UART0, (UartTx, UartRx)>;

pub fn setup(
    pac: pac::Peripherals,
    _core: cortex_m::Peripherals,
) -> (Rp2040Monotonic, LedPin, UartType, PioTx) {
    let mut resets = pac.RESETS;
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut resets,
        &mut watchdog,
    ).ok().unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut resets);
    
    let tx = pins.gpio12.into_mode::<FunctionUart>();
    let rx = pins.gpio13.into_mode::<FunctionUart>();
    
    let mut uart = UartPeripheral::<_, _, _>::new(pac.UART0, (tx, rx), &mut resets)
            .enable(
                common_configs::_115200_8_N_1,
                clocks.peripheral_clock.into(),
            )
            .unwrap();

        uart.enable_rx_interrupt();


    let led = pins.gpio25.into_push_pull_output();

    let mono = Rp2040Monotonic::new(pac.TIMER);
    
    let _dcc_pin: Pin<_, FunctionPio0> = pins.gpio15.into_mode();
    let program = pio_proc::pio!( // if you use pio! then use pio-proc = "0.1" in Cargo.toml otherwise you will get an error
32,
"
.wrap_target
bitloop:
  set pins, 1    [20]
  out x, 1
  jmp !x do_zero
  set pins, 0    [21]
  jmp bitloop
do_zero:
  nop            [16]
  set pins, 0    [30]
  nop             [8]

.wrap
"
    );
    let (mut pio, sm0, _, _, _,) = pac.PIO0.split(&mut resets);
    let installed = pio.install(&program.program).unwrap();
    let div = clocks.system_clock.freq().integer() as f32 / 400_000 as f32;
    let (mut sm, _, pio_tx) = PIOBuilder::from_program(installed)
    .set_pins(15,1)     
    .autopull(true)
    .out_shift_direction(ShiftDirection::Left)
    .clock_divisor(div)
      .build(sm0);
    sm.set_pindirs([(15, PinDir::Output)]);
    sm.start();

    (mono, led, uart, pio_tx)
}

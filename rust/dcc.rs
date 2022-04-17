// filename: dcc.rs

use embedded_time::{
    fixed_point::FixedPoint,};
    
use rp2040_hal::{
    gpio::bank0::Gpio15,
    gpio::{Function, Pin, PinId},
    pio::{PIOBuilder, ShiftDirection, Tx, UninitStateMachine, PIO, SM0},
    pac::PIO0,};

    pub fn init(
        _pin: Pin<Gpio15, Function<PIO0>>,
        pio: &mut PIO<PIO0>,
        sm: UninitStateMachine<(PIO0, SM0)>,
        clock_freq: embedded_time::rate::Hertz,
    ) -> Tx<(PIO0, SM0)> {
        let program = pio_proc::pio!(32, "
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
");
    let installed = pio.install(&program.program).unwrap();   
    let div = clock_freq.integer() as f32 / 400_000 as f32; // note the clock divisor
    let (mut sm, _, tx) = PIOBuilder::from_program(installed)
       .set_pins(15,1)     
       .autopull(true)
       .out_shift_direction(ShiftDirection::Left)
       .clock_divisor(div)
       .build(sm);           
    sm.set_pindirs([(Gpio15::DYN.num, rp2040_hal::pio::PinDir::Output)]);
    sm.start();
    tx}


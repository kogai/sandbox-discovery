pub use cortex_m_rt::entry;
use f3::{
  hal::{delay::Delay, prelude::*, stm32f30x},
  led::Leds,
};

pub fn init() -> (Delay, Leds) {
  let cp = cortex_m::Peripherals::take().unwrap();
  let dp = stm32f30x::Peripherals::take().unwrap();

  let mut flash = dp.FLASH.constrain();
  let mut rcc = dp.RCC.constrain();

  let clocks = rcc.cfgr.freeze(&mut flash.acr);

  let delay = Delay::new(cp.SYST, clocks);

  let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

  (delay, leds)
}

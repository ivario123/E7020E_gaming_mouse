// From terminal:
// cargo run --example rtic_blinky_48mhz
// Assumes cargo/config.toml
// runner = "arm-none-eabi-gdb -q -x openocd.gdb"
// Assumes `openocd` running in separate terminal
// > openocd -f openocd.cfg
//
// From vscode:
// Press F5 (to compile/flash/debug the application)
// TERMINAL->gdb-server (to view semihosting trace in the openocd session)
// DEBUG CONSOLE (to view/control the gdb session)
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::*;
    use stm32f4xx_hal::prelude::*;

    // Core clock at 48MHz
    const FREQ_CORE: u32 = 48_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 48MHz cycle accurate accuracy

    #[shared]
    struct Shared {
        #[lock_free] // shared between tasks at the same priority
        gpioa: stm32f4::stm32f411::GPIOA,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init").ok();

        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let rcc = cx.device.RCC;

        // enable gpioa
        rcc.ahb1enr.write(|w| w.gpioaen().enabled());

        // set core clock to 48MHz and freeze the settings
        let clk = rcc.constrain().cfgr.sysclk(48.mhz()).freeze();

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, clk.sysclk().0);

        let gpioa = cx.device.GPIOA;

        // set mode
        gpioa.moder.write(|w| w.moder5().output());

        led_on::spawn().ok();

        (Shared { gpioa }, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("idle").ok();

        loop {}
    }

    #[task(shared = [gpioa])]
    fn led_on(cx: led_on::Context) {
        hprintln!("led_on").ok();
        cx.shared.gpioa.bsrr.write(|w| w.br5().set_bit());
        led_off::spawn_after(1.secs()).ok();
    }

    #[task(shared = [gpioa])]
    fn led_off(cx: led_off::Context) {
        hprintln!("led_off").ok();
        cx.shared.gpioa.bsrr.write(|w| w.bs5().set_bit());
        led_on::spawn_after(1.secs()).ok();
    }
}
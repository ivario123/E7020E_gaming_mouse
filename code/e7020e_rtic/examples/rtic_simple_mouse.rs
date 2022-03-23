//! rtic_usb_mouse.rs
//!
//! Mouse HID example
//!
//! What it covers:
//! - Mouse HID communication
//! - good design, interrupt driven polling
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.
#![no_main]
#![no_std]

use panic_rtt_target as _;
//type Button = ErasedPin<Input<PullUp>>;
#[rtic::app(device = stm32f4::stm32f401, dispatchers = [DMA1_STREAM0,DMA1_STREAM1])]
mod app {
    // Relative app imports
    use app::pmw3389::Pmw3389;
    use app::mouseReport::MouseState;
    use app::pca9624_pw::*;
    use app::hidDescriptors::MouseKeyboard;
    use app::mouseKeyboardReport::MouseKeyboardState;
    use app::rgb_pattern_things::*;
    // Absolute imports
    use eeprom::EEPROM;
    use stm32f4::stm32f401::*;
    
    use rtt_target::{
        rprintln,
        rtt_init_print
    };
    use dwt_systick_monotonic::*;
    use usb_device::{
        bus::UsbBusAllocator,
        prelude::*,
        endpoint::*
    };
    use usbd_hid::{
        descriptor::{generator_prelude::*},
        hid_class::HIDClass,
    };
    use stm32f4xx_hal::{
        gpio::{Alternate, Output, Pin, PushPull, Speed},
        prelude::*,
        spi::{Spi, TransferModeNormal},
        timer::Delay,
        gpio::*,
        otg_fs::{UsbBus, USB},
        i2c::*,
    };
    
    // Includes for the spi interface
    use embedded_hal::spi::MODE_3;
    use stm32f4::stm32f401::{SPI1, TIM5};


    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    
    type Button = ErasedPin<Input<PullDown>>;
    // Types for spi interface
    type SCK = Pin<Alternate<PushPull, 5_u8>, 'A', 5_u8>;
    type MOSI = Pin<Alternate<PushPull, 5_u8>, 'A', 7_u8>;
    type MISO = Pin<Alternate<PushPull, 5_u8>, 'A', 6_u8>;
    type CS = Pin<Output<PushPull>, 'A', 4_u8>;
    type SPI = Spi<SPI1, (SCK, MISO, MOSI), TransferModeNormal>;
    // Types for pmw3389 device driver
    type DELAY = Delay<TIM5, 1000000_u32>;
    type PMW3389 = Pmw3389<SPI, CS, DELAY>;
    // Types for the i2c interface
    type SCL = Pin<Alternate<OpenDrain, 4_u8>, 'A', 8_u8>;
    type SDA = Pin<Alternate<OpenDrain, 4_u8>, 'C', 9_u8>;
    type I2C = I2c<I2C3, (SCL, SDA)>;
    
    type SERIAL_BUSS<'a> = usbd_serial::SerialPort<'a,UsbBus<USB> > ;
    
    #[shared]
    struct Shared {
        mouse: MouseKeyboardState,
        EXTI : EXTI,
    }
    
    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, UsbBus<USB>>,
        hid: HIDClass<'static, UsbBus<USB>>,
        left: Button,
        right: Button,
        middle: Button,
        front: Button,
        back: Button,
        phase_a : Button,
        phase_b : Button,
        motion : Button,
        ts : u32,
        rgb_pattern_driver : RgbController
    }

    const POLL_INTERVAL_MS: u8 = 1;

    #[init(local = [EP_MEMORY: [u32; 1024] = [0; 1024], bus: Option<UsbBusAllocator<UsbBus<USB>>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // grab core and device pointers
        let mut cd = cx.core;
        let mut dp = cx.device;

        // Systic config
        let mut sys_cfg = dp.SYSCFG.constrain();
        let mono = DwtSystick::new(&mut cd.DCB, cd.DWT,cd.SYST , FREQ_CORE);

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze();
        // Grab gpio pins
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();
        // define usb config
        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        // Configure pmw3389 sensor
        let sck         : SCK       = gpioa.pa5.into_alternate().set_speed(Speed::VeryHigh);
        let miso        : MISO      = gpioa.pa6.into_alternate().set_speed(Speed::High);
        let mosi        : MOSI      = gpioa.pa7.into_alternate().set_speed(Speed::High);
        let cs          : CS        = gpioa.pa4.into_push_pull_output().set_speed(Speed::High);
        let spi         : SPI       = Spi::new(dp.SPI1, (sck, miso, mosi), MODE_3, 1.MHz(), &clocks);
        let delay       : DELAY     = dp.TIM5.delay_us(&clocks);
        let mut pmw3389 : PMW3389   = Pmw3389::new(spi, cs, delay).unwrap();
        // Write the cpi regs on startup
        pmw3389.store_cpi().ok();
        // Defines a i2c interface
        let scl         : SCL       = gpioa.pa8.into_alternate_open_drain();
        let sda         : SDA       = gpioc.pc9.into_alternate_open_drain();
        let mut i2c     : I2C       = I2c::new(dp.I2C3, (scl,sda), Mode::from(1.MHz()), &clocks); 
        
        
        let mut interfaces = standard_interfaces();
        let mut rgb_controller:PCA9624PW = PCA9624PW::new(i2c,interfaces,0xC0); // Might be 0xC2
        // Initiates the pattern controller
        let mut rgb_pattern_driver = RgbController::new(rgb_controller);



        // Configure IO pins
        let mut motion:Button = gpiob.pb13.into_pull_down_input().erase();
        let mut phase_a:Button = gpiob.pb2.into_pull_down_input().erase();
        let mut phase_b:Button = gpioc.pc10.into_pull_down_input().erase();
        let mut left:Button = gpiob.pb0.into_pull_down_input().erase();
        let mut right:Button = gpiob.pb1.into_pull_down_input().erase();
        let mut middle:Button = gpiob.pb12.into_pull_down_input().erase();
        let mut front:Button = gpioc.pc5.into_pull_down_input().erase();
        let mut back:Button = gpioc.pc4.into_pull_down_input().erase();

        init_button(&mut phase_a,&mut dp.EXTI,Edge::Rising,&mut sys_cfg);
        init_button(&mut phase_b,&mut dp.EXTI,Edge::Rising,&mut sys_cfg);
        init_button(&mut left,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        init_button(&mut right,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        init_button(&mut middle,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        init_button(&mut front,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        init_button(&mut back,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        
        cx.local.bus.replace(UsbBus::new(usb, cx.local.EP_MEMORY));
        // Configure usb class
        let hid = HIDClass::new(
            cx.local.bus.as_ref().unwrap(),
            MouseKeyboard::desc(),
            POLL_INTERVAL_MS,
        );

        let mouse = MouseKeyboardState::new(pmw3389);
        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0xc410, 0x000))
                .manufacturer("Ivar och Erik")
                .product("Banger gaming mus")
                .serial_number("1234")
                .max_power(500) // Just take all the power
                .composite_with_iads()      // Define as a composite device
                .build();
        // Enable host wakeup
        usb_dev.remote_wakeup_enabled();
        // It seems that we can install multiple endpoints
        // We would need to denfine 
        let mut EXTI = dp.EXTI;
        let ts = 0;
        (
            Shared {
                mouse,
                EXTI
            },
            Local { 
                usb_dev,
                hid,
                left,
                right, 
                middle, 
                front, 
                back, 
                phase_a,
                phase_b,
                motion,
                ts,
                rgb_pattern_driver
            },
            init::Monotonics(mono)
        )
    }
    fn init_button(btn : &mut Button,  exti : &mut EXTI,edge : stm32f4xx_hal::gpio::Edge,sys_cfg : &mut stm32f4xx_hal::syscfg::SysCfg){
        btn.make_interrupt_source(sys_cfg);
        btn.enable_interrupt(exti);
        btn.trigger_on_edge(exti, edge);

    }

    /// defines a simple pattern loop this should be started at startup when there is some form of pre saved pattern
    #[task(local = [rgb_pattern_driver])]
    fn pattern_itterator(cx : pattern_itterator::Context){
        let mut step = cx.local.rgb_pattern_driver.next_color();
        //pattern_itterator::spawn_after(dwt_systick_monotonic::fugit::Duration(step as u32));
    }




    #[task(binds=EXTI15_10,priority = 2, local = [middle,motion,ts], shared = [mouse])]
    fn middle_hand(mut cx: middle_hand::Context) {
        // this should be automatic
        cx.local.middle.clear_interrupt_pending_bit();
        if cx.local.middle.is_low() {
                rprintln!("middle low");
                cx.shared.mouse.lock(|mouse| {
                    mouse.release_middle();
                });
        } else if cx.local.middle.is_high() {
                rprintln!("middle high");
                cx.shared.mouse.lock(|mouse| {
                    mouse.push_middle();
                });
        }
        
    }
    
    /*#[task(binds=EXTI0, priority = 2, local = [left], shared = [mouse])]
    extern "Rust"{
    mouse.left_button();
    }*/
    #[task(binds=EXTI0, priority = 2, local = [left], shared = [mouse])]
    fn left_hand(mut cx: left_hand::Context) {
        // this should be automatic
        cx.local.left.clear_interrupt_pending_bit();

        if cx.local.left.is_low() {
            rprintln!("left low");
            cx.shared.mouse.lock(|mouse| {
                mouse.release_left();
            });
        } else {
            rprintln!("left high");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_left();
            });
        }
    }
    #[task(binds=EXTI1, local = [right], shared = [mouse])]
    fn right_hand(mut cx: right_hand::Context) {
        // this should be automatic
        cx.local.right.clear_interrupt_pending_bit();

        if cx.local.right.is_low() {
            rprintln!("right low");
            cx.shared.mouse.lock(|mouse| {
                mouse.release_right();
            });
        } else {
            rprintln!("right high");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_right();
            });
        }
    }
    #[task(binds=EXTI2, local = [phase_a], shared = [mouse])]
    fn phase_a_hand(mut cx: phase_a_hand::Context) {
        // this should be automatic
        cx.local.phase_a.clear_interrupt_pending_bit();

        rprintln!("phase_a high");
        cx.shared.mouse.lock(|mouse| {
                mouse.handle_scroll('a');
        });
        
    }
    #[task(binds=EXTI9_5, local = [front,phase_b], shared = [mouse,EXTI])]
    fn front_hand(mut cx: front_hand::Context) {
        cx.local.front.clear_interrupt_pending_bit();
        cx.local.phase_b.clear_interrupt_pending_bit();
        if cx.local.phase_b.is_high() {
            rprintln!("phase_b low");
            cx.shared.mouse.lock(|mouse| {
                mouse.handle_scroll('b');
            });
        } 
        else
        {
                if cx.local.front.is_low() {
                    rprintln!("front low");
                    cx.shared.mouse.lock(|mouse| {
                    
                });
            } else {
                rprintln!("front high");
                cx.shared.mouse.lock(|mouse| {
                    mouse.increment_dpi(1);
                    //mouse.push_front();
                });
            }
            // Temporarelly disable interrupts
            cx.shared.EXTI.lock(|EXTI|{
                cx.local.front.disable_interrupt(EXTI);
            });
            delay(160000);
            cx.shared.EXTI.lock(|EXTI|{
                cx.local.front.enable_interrupt(EXTI);
            });
        }
    }
    #[no_mangle]
    fn delay(td:u32){
        let time = monotonics::now().ticks() as u32;
        while(monotonics::now().ticks() as u32 - time) <  td{}
    }

    #[task(binds=EXTI4, local = [back], shared = [mouse,EXTI])]
    fn back_hand(mut cx: back_hand::Context) {
        // this should be automatic
        cx.local.back.clear_interrupt_pending_bit();
        // Temporarelly disable interrupts
        cx.shared.EXTI.lock(|EXTI|{
            cx.local.back.disable_interrupt(EXTI);
        });
        delay(160000);
        cx.shared.EXTI.lock(|EXTI|{
            cx.local.back.enable_interrupt(EXTI);
        });
        if cx.local.back.is_low() {
            rprintln!("back low");
            cx.shared.mouse.lock(|mouse| {
                mouse.increment_dpi(-1);
                //mouse.release_front();
            });
        } else {
            //rprintln!("back high");
            cx.shared.mouse.lock(|mouse| {
                
                //mouse.push_front();
            });
        }
    }

    // interrupt generated each time the hid device is polled
    // in this example each 1ms (POLL_INTERVAL_MS = 1)
    #[task(
        binds=OTG_FS,
        priority = 1,
        local = [usb_dev, hid, first :bool = true, counter:u16 = 0],
        shared = [mouse]
    )]
    fn usb_fs(mut cx: usb_fs::Context) {
        let usb_fs::LocalResources {
            usb_dev,
            hid,
            first,
            counter,
        } = cx.local;

        if *first {
            rprintln!("first");
            *first = false;
        }

        // Buffer could be extended if needed
        let mut buf = [0u8; 1024];
        match hid.pull_raw_output(&mut buf).ok(){
            // Should return almost istantaneously if there is no data
            Some(len) => {
                // The mouse has been polled for update purposes
                rprintln!("{:?}",buf);
                handle_host_call::spawn(buf);
            },
            None => {
            }
        }
        // The mouse has been polled for non update purposes
        cx.shared.mouse.lock(|mouse| {
            let report = mouse.get_report_and_reset();
            // push the report
            hid.push_input(&report).ok();
        });
        // update the usb device state
        if usb_dev.poll(&mut [hid]) {
            return;
        }
        
    }
    #[task()]
    fn handle_host_call(mut cx :handle_host_call::Context,buffer : [u8;1024]) {
        rprintln!("handle host call");
        rprintln!("{:?}", buffer);
        // Defines an api
        match buffer[0]{
            0x01 => {
                rprintln!("RGB _controll");
            },
            0x02 => {
                rprintln!("DPI _controll");
                // In this case the next 2 bytes are the new dpi
                let dpi = (buffer[1] as u16) << 8 | buffer[2] as u16;
                handle_dpi::spawn(dpi).unwrap();
            },
            0x03 => {
                rprintln!("DPI _controll");
            },
            0x04 => {
                rprintln!("Macro _controll");
            },
            _ => {
                rprintln!("unknown");
            }
        }

    }
    #[task(shared = [mouse])]
    fn handle_dpi(mut cx : handle_dpi::Context,dpi : u16){
        rprintln!("{:}",dpi);
        cx.shared.mouse.lock(|mouse| {
            mouse.write_dpi(dpi);
        });
    }
    #[idle(shared = [mouse])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
            cx.shared.mouse.lock(|mouse|{
                mouse.read_sensor();
            });   
        }
    }
    
    
}

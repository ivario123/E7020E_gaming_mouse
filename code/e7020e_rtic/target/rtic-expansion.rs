#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f401 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; pub use
    rtic :: Monotonic as _ ;
    #[doc = r" Holds static methods for each monotonic."] pub mod monotonics
    {
        pub use MyMono :: now ;
        #[doc =
          "This module holds the static implementation for `MyMono::now()`"]
        #[allow(non_snake_case)] pub mod MyMono
        {
            #[doc = r" Read the current time from this monotonic"] pub fn
            now() -> < super :: super :: MyMono as rtic :: Monotonic > ::
            Instant
            {
                rtic :: export :: interrupt ::
                free(| _ |
                     {
                         use rtic :: Monotonic as _ ; if let Some(m) = unsafe
                         {
                             & mut * super :: super ::
                             __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()
                         } { m.now() } else
                         {
                             < super :: super :: MyMono as rtic :: Monotonic >
                             :: zero()
                         }
                     })
            }
        }
    } use rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal ::
    otg_fs :: { UsbBus, USB } ; use stm32f4xx_hal :: prelude :: * ; use
    stm32f4xx_hal :: gpio :: * ; use dwt_systick_monotonic :: * ; use
    usb_device :: { bus :: UsbBusAllocator, prelude :: * } ; use usbd_hid ::
    {
        descriptor :: { generator_prelude :: *, MouseReport }, hid_class ::
        HIDClass,
    } ; use systick_monotonic :: * ; use app :: hidDescriptors ::
    MouseKeyboard ; use app :: mouseKeyboardReport :: MouseKeyboardState ; use
    app :: macroSystem ::
    {
        MacroConfig, Function, MacroType, do_function, end_function,
        MacroSequence
    } ; #[doc = r" User code from within the module"] type Button = ErasedPin
    < Input < PullUp > > ; const FREQ_CORE : u32 = 48_000_000 ; type MyMono =
    DwtSystick < FREQ_CORE > ; const POLL_INTERVAL_MS : u8 = 1 ; fn
    handle_macro(m : & 'static MacroType, mouse : & mut MouseKeyboardState,
                 is_push : bool)
    {
        match m
        {
            MacroType :: MacroSingle(f) =>
            {
                match is_push
                {
                    true => do_function(* f, mouse), false =>
                    end_function(* f, mouse),
                }
            }, MacroType :: MacroMultiple(s) => {}
        }
    } #[doc = r" User code end"] #[inline(always)] #[allow(non_snake_case)] fn
    init(cx : init :: Context) -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let mut dp = cx.device ; let
        systick = cx.core.SYST ; let mut dcb = cx.core.DCB ; let dwt =
        cx.core.DWT ; let mono = DwtSystick ::
        new(& mut dcb, dwt, systick, FREQ_CORE) ; let rcc = dp.RCC.constrain()
        ; let clocks = rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze() ;
        let gpioa = dp.GPIOA.split() ; let usb = USB
        {
            usb_global : dp.OTG_FS_GLOBAL, usb_device : dp.OTG_FS_DEVICE,
            usb_pwrclk : dp.OTG_FS_PWRCLK, pin_dm :
            gpioa.pa11.into_alternate(), pin_dp : gpioa.pa12.into_alternate(),
            hclk : clocks.hclk(),
        } ; let gpioc = dp.GPIOC.split() ; let gpiob = dp.GPIOB.split() ; let
        mut button = gpioa.pa13.into_pull_up_input().erase() ; let mut sys_cfg
        = dp.SYSCFG.constrain() ; button.make_interrupt_source(& mut sys_cfg)
        ; button.enable_interrupt(& mut dp.EXTI) ;
        button.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        cx.local.bus.replace(UsbBus :: new(usb, cx.local.EP_MEMORY)) ; let hid
        = HIDClass ::
        new(cx.local.bus.as_ref().unwrap(), MouseKeyboard :: desc(),
            POLL_INTERVAL_MS,) ; let mouse = MouseKeyboardState :: new() ; let
        mut macro_conf = MacroConfig :: new() ;
        macro_conf.update_config(MacroType ::
                                 MacroSingle(Function :: PressKeyboard(0x4)),
                                 MacroType ::
                                 MacroSingle(Function :: PressKeyboard(0x4)),
                                 MacroType ::
                                 MacroSingle(Function :: Nothing), MacroType
                                 :: MacroSingle(Function :: Nothing),
                                 MacroType ::
                                 MacroSingle(Function :: Nothing), MacroType
                                 :: MacroSingle(Function :: Nothing),
                                 MacroType ::
                                 MacroSingle(Function :: Nothing),) ; let
        usb_dev = UsbDeviceBuilder ::
        new(cx.local.bus.as_ref().unwrap(),
            UsbVidPid(0xc410,
                      0x0000)).manufacturer("e7020e").product("Mouse").serial_number("1337").device_class(0).build()
        ;
        (Shared { mouse, macro_conf }, Local { usb_dev, hid, button }, init ::
         Monotonics(mono))
    } #[allow(non_snake_case)] fn
    button_pressed(mut cx : button_pressed :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.button.clear_interrupt_pending_bit() ; if
        cx.local.button.is_low()
        {
            rprintln! ("button low") ;
            cx.shared.macro_conf.lock(| macro_conf |
                                      {
                                          cx.shared.mouse.lock(| mouse | {}) ;
                                      }) ;
        } else
        {
            rprintln! ("button high") ;
            cx.shared.macro_conf.lock(| macro_conf |
                                      {
                                          cx.shared.mouse.lock(| mouse | {}) ;
                                      }) ;
        }
    } #[allow(non_snake_case)] fn usb_fs(mut cx : usb_fs :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let usb_fs
        :: LocalResources { usb_dev, hid, first, counter, } = cx.local ; if *
        first { rprintln! ("first") ; * first = false ; } * counter =
        (* counter + 1) % 200 ; let mov = match * counter
        {
            100 => { rprintln! ("10") ; 10 } 0 => { rprintln! ("-10") ; - 10 }
            _ => 0,
        } ; cx.shared.mouse.lock(| mouse | { mouse.add_x_movement(mov) ; }) ;
        cx.shared.mouse.lock(| mouse |
                             {
                                 let report = mouse.get_report_and_reset() ;
                                 hid.push_input(& report).ok() ;
                             }) ; if usb_dev.poll(& mut [hid]) { return ; }
    } #[allow(non_snake_case)] fn
    do_macro(cx : do_macro :: Context, m : & 'static MacroSequence, i : u8)
    { use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; } struct
    Shared { mouse : MouseKeyboardState, macro_conf : MacroConfig, } struct
    Local
    {
        usb_dev : UsbDevice < 'static, UsbBus < USB > >, hid : HIDClass <
        'static, UsbBus < USB > >, button : Button,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < >
    {
        pub EP_MEMORY : & 'static mut [u32 ; 1024], pub bus : & 'static mut
        Option < UsbBusAllocator < UsbBus < USB > > >,
    } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_Monotonics(pub DwtSystick < FREQ_CORE >) ;
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc = r" Core (Cortex-M) peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals"] pub device : stm32f4 ::
        stm32f401 :: Peripherals, #[doc = r" Critical section token for init"]
        pub cs : rtic :: export :: CriticalSection < 'a >,
        #[doc = r" Local Resources this task has access to"] pub local : init
        :: LocalResources < >,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                device : stm32f4 :: stm32f401 :: Peripherals :: steal(), cs :
                rtic :: export :: CriticalSection :: new(), core, local : init
                :: LocalResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_initLocalResources as
        LocalResources ; pub use super :: __rtic_internal_Monotonics as
        Monotonics ; pub use super :: __rtic_internal_init_Context as Context
        ;
    } mod shared_resources
    {
        use rtic :: export :: Priority ; #[doc(hidden)]
        #[allow(non_camel_case_types)] pub struct
        mouse_that_needs_to_be_locked < 'a > { priority : & 'a Priority, }
        impl < 'a > mouse_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { mouse_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        macro_conf_that_needs_to_be_locked < 'a >
        { priority : & 'a Priority, } impl < 'a >
        macro_conf_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { macro_conf_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `button_pressed` has access to"] pub struct
    __rtic_internal_button_pressedLocalResources < 'a >
    { pub button : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `button_pressed` has access to"] pub struct
    __rtic_internal_button_pressedSharedResources < 'a >
    {
        pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >,
        pub macro_conf : shared_resources ::
        macro_conf_that_needs_to_be_locked < 'a >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_button_pressed_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        button_pressed :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        button_pressed :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_button_pressed_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_button_pressed_Context
            {
                local : button_pressed :: LocalResources :: new(), shared :
                button_pressed :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod button_pressed
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_button_pressedLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_button_pressedSharedResources as SharedResources ; pub
        use super :: __rtic_internal_button_pressed_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `usb_fs` has access to"] pub struct
    __rtic_internal_usb_fsLocalResources < 'a >
    {
        pub usb_dev : & 'a mut UsbDevice < 'static, UsbBus < USB > >, pub hid
        : & 'a mut HIDClass < 'static, UsbBus < USB > >, pub first : & 'a mut
        bool, pub counter : & 'a mut u16,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `usb_fs` has access to"] pub struct
    __rtic_internal_usb_fsSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_usb_fs_Context <
    'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        usb_fs :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        usb_fs :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_usb_fs_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usb_fs_Context
            {
                local : usb_fs :: LocalResources :: new(), shared : usb_fs ::
                SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod usb_fs
    {
        #[doc(inline)] pub use super :: __rtic_internal_usb_fsLocalResources
        as LocalResources ; #[doc(inline)] pub use super ::
        __rtic_internal_usb_fsSharedResources as SharedResources ; pub use
        super :: __rtic_internal_usb_fs_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `do_macro` has access to"] pub struct
    __rtic_internal_do_macroSharedResources < 'a >
    {
        pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >,
        pub macro_conf : shared_resources ::
        macro_conf_that_needs_to_be_locked < 'a >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_do_macro_Context
    < 'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        do_macro :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_do_macro_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_do_macro_Context
            { shared : do_macro :: SharedResources :: new(priority), }
        }
    } #[doc = r" Spawns the task directly"] pub fn
    __rtic_internal_do_macro_spawn(_0 : & 'static MacroSequence, _1 : u8,) ->
    Result < (), (& 'static MacroSequence, u8,) >
    {
        let input = (_0, _1,) ; unsafe
        {
            if let Some(index) = rtic :: export :: interrupt ::
            free(| _ |
                 (& mut * __rtic_internal_do_macro_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_do_macro_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                              ::
                                                                              from(index)).as_mut_ptr().write(input)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).enqueue_unchecked((P1_T
                                                                              ::
                                                                              do_macro,
                                                                              index))
                         ;
                     }) ; rtic ::
                pend(stm32f4 :: stm32f401 :: interrupt :: EXTI0) ; Ok(())
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_do_macro_MyMono_SpawnHandle
    { #[doc(hidden)] marker : u32, } impl core :: fmt :: Debug for
    __rtic_internal_do_macro_MyMono_SpawnHandle
    {
        fn fmt(& self, f : & mut core :: fmt :: Formatter < '_ >) -> core ::
        fmt :: Result { f.debug_struct("MyMono::SpawnHandle").finish() }
    } impl __rtic_internal_do_macro_MyMono_SpawnHandle
    {
        pub fn cancel(self) -> Result < (& 'static MacroSequence, u8,), () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let tq = & mut * __rtic_internal_TQ_MyMono.get_mut() ; if
                     let Some((_task, index)) = tq.cancel_marker(self.marker)
                     {
                         let msg =
                         (& *
                          __rtic_internal_do_macro_INPUTS.get()).get_unchecked(usize
                                                                               ::
                                                                               from(index)).as_ptr().read()
                         ;
                         (& mut *
                          __rtic_internal_do_macro_FQ.get_mut()).split().0.enqueue_unchecked(index)
                         ; Ok(msg)
                     } else { Err(()) }
                 })
        } #[inline] pub fn
        reschedule_after(self, duration : < MyMono as rtic :: Monotonic > ::
                         Duration) -> Result < Self, () >
        { self.reschedule_at(monotonics :: MyMono :: now() + duration) } pub
        fn
        reschedule_at(self, instant : < MyMono as rtic :: Monotonic > ::
                      Instant) -> Result < Self, () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let marker =
                     __rtic_internal_TIMER_QUEUE_MARKER.get().read() ;
                     __rtic_internal_TIMER_QUEUE_MARKER.get_mut().write(marker.wrapping_add(1))
                     ; let tq = (& mut * __rtic_internal_TQ_MyMono.get_mut())
                     ;
                     tq.update_marker(self.marker, marker, instant, || rtic ::
                                      export :: SCB ::
                                      set_pendst()).map(| _ | do_macro ::
                                                        MyMono :: SpawnHandle
                                                        { marker })
                 })
        }
    }
    #[doc =
      r" Spawns the task after a set duration relative to the current time"]
    #[doc = r""]
    #[doc =
      r" This will use the time `Instant::new(0)` as baseline if called in `#[init]`,"]
    #[doc =
      r" so if you use a non-resetable timer use `spawn_at` when in `#[init]`"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_do_macro_MyMono_spawn_after(duration : < MyMono as rtic ::
                                                Monotonic > :: Duration, _0 :
                                                & 'static MacroSequence, _1 :
                                                u8) -> Result < do_macro ::
    MyMono :: SpawnHandle, (& 'static MacroSequence, u8,) >
    {
        let instant = monotonics :: MyMono :: now() ;
        __rtic_internal_do_macro_MyMono_spawn_at(instant + duration, _0, _1)
    } #[doc = r" Spawns the task at a fixed time instant"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_do_macro_MyMono_spawn_at(instant : < MyMono as rtic ::
                                             Monotonic > :: Instant, _0 : &
                                             'static MacroSequence, _1 : u8)
    -> Result < do_macro :: MyMono :: SpawnHandle,
    (& 'static MacroSequence, u8,) >
    {
        unsafe
        {
            let input = (_0, _1,) ; if let Some(index) = rtic :: export ::
            interrupt ::
            free(| _ |
                 (& mut * __rtic_internal_do_macro_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_do_macro_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                              ::
                                                                              from(index)).as_mut_ptr().write(input)
                ;
                (& mut *
                 __rtic_internal_do_macro_MyMono_INSTANTS.get_mut()).get_unchecked_mut(usize
                                                                                       ::
                                                                                       from(index)).as_mut_ptr().write(instant)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         let marker =
                         __rtic_internal_TIMER_QUEUE_MARKER.get().read() ; let
                         nr = rtic :: export :: NotReady
                         {
                             instant, index, task : SCHED_T :: do_macro,
                             marker,
                         } ;
                         __rtic_internal_TIMER_QUEUE_MARKER.get_mut().write(__rtic_internal_TIMER_QUEUE_MARKER.get().read().wrapping_add(1))
                         ; let tq = & mut *
                         __rtic_internal_TQ_MyMono.get_mut() ;
                         tq.enqueue_unchecked(nr, || core :: mem :: transmute
                                              :: < _, rtic :: export :: SYST >
                                              (()).enable_interrupt(), || rtic
                                              :: export :: SCB ::
                                              set_pendst(),
                                              (& mut *
                                               __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut())
                         ; Ok(do_macro :: MyMono :: SpawnHandle { marker })
                     })
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod do_macro
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_do_macroSharedResources as SharedResources ; pub use
        super :: __rtic_internal_do_macro_Context as Context ; pub use super
        :: __rtic_internal_do_macro_spawn as spawn ; pub use MyMono ::
        spawn_after ; pub use MyMono :: spawn_at ; pub use MyMono ::
        SpawnHandle ; pub mod MyMono
        {
            pub use super :: super ::
            __rtic_internal_do_macro_MyMono_spawn_after as spawn_after ; pub
            use super :: super :: __rtic_internal_do_macro_MyMono_spawn_at as
            spawn_at ; pub use super :: super ::
            __rtic_internal_do_macro_MyMono_SpawnHandle as SpawnHandle ;
        }
    } #[doc = r" app module"] impl < > __rtic_internal_initLocalResources < >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_initLocalResources
            {
                EP_MEMORY : & mut *
                __rtic_internal_local_init_EP_MEMORY.get_mut(), bus : & mut *
                __rtic_internal_local_init_bus.get_mut(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic41117"] static
    __rtic_internal_shared_resource_mouse : rtic :: RacyCell < core :: mem ::
    MaybeUninit < MouseKeyboardState >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: mouse_that_needs_to_be_locked < 'a >
    {
        type T = MouseKeyboardState ; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut MouseKeyboardState) ->
         RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_mouse.get_mut() as * mut
                     _, self.priority(), CEILING, stm32f4 :: stm32f401 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic41118"] static
    __rtic_internal_shared_resource_macro_conf : rtic :: RacyCell < core ::
    mem :: MaybeUninit < MacroConfig >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: macro_conf_that_needs_to_be_locked < 'a >
    {
        type T = MacroConfig ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut MacroConfig) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_macro_conf.get_mut() as *
                     mut _, self.priority(), CEILING, stm32f4 :: stm32f401 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic41119"] static
    __rtic_internal_local_resource_usb_dev : rtic :: RacyCell < core :: mem ::
    MaybeUninit < UsbDevice < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic41120"] static
    __rtic_internal_local_resource_hid : rtic :: RacyCell < core :: mem ::
    MaybeUninit < HIDClass < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic41121"] static
    __rtic_internal_local_resource_button : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_EP_MEMORY : rtic ::
    RacyCell < [u32 ; 1024] > = rtic :: RacyCell :: new([0 ; 1024]) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_bus : rtic :: RacyCell <
    Option < UsbBusAllocator < UsbBus < USB > > > > = rtic :: RacyCell ::
    new(None) ; #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_local_usb_fs_first : rtic :: RacyCell < bool > = rtic ::
    RacyCell :: new(true) ; #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_local_usb_fs_counter : rtic :: RacyCell < u16 > = rtic ::
    RacyCell :: new(0) ; #[allow(non_snake_case)] #[no_mangle] unsafe fn
    EXTI15_10()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                button_pressed(button_pressed :: Context ::
                               new(& rtic :: export :: Priority ::
                                   new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_button_pressedLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_button_pressedLocalResources
            {
                button : & mut *
                (& mut *
                 __rtic_internal_local_resource_button.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_button_pressedSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_button_pressedSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority), macro_conf : shared_resources ::
                macro_conf_that_needs_to_be_locked :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn OTG_FS()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                usb_fs(usb_fs :: Context ::
                       new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_usb_fsLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_usb_fsLocalResources
            {
                usb_dev : & mut *
                (& mut *
                 __rtic_internal_local_resource_usb_dev.get_mut()).as_mut_ptr(),
                hid : & mut *
                (& mut *
                 __rtic_internal_local_resource_hid.get_mut()).as_mut_ptr(),
                first : & mut * __rtic_internal_local_usb_fs_first.get_mut(),
                counter : & mut *
                __rtic_internal_local_usb_fs_counter.get_mut(),
            }
        }
    } impl < 'a > __rtic_internal_usb_fsSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usb_fsSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_do_macro_FQ : rtic :: RacyCell <
    rtic :: export :: SCFQ < 2 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ;
    #[link_section = ".uninit.rtic41122"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_do_macro_MyMono_INSTANTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit << DwtSystick < FREQ_CORE > as rtic ::
     Monotonic > :: Instant > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ;
    #[link_section = ".uninit.rtic41123"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_do_macro_INPUTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit < (& 'static MacroSequence, u8,) > ; 1] > =
    rtic :: RacyCell :: new([core :: mem :: MaybeUninit :: uninit(),]) ; impl
    < 'a > __rtic_internal_do_macroSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_do_macroSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority), macro_conf : shared_resources ::
                macro_conf_that_needs_to_be_locked :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)] #[doc(hidden)] pub enum P1_T { do_macro, }
    #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_P1_RQ : rtic ::
    RacyCell < rtic :: export :: SCRQ < P1_T, 2 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ; #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch tasks at priority 1"] #[no_mangle]
    unsafe fn EXTI0()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                while let Some((task, index)) =
                (& mut * __rtic_internal_P1_RQ.get_mut()).split().1.dequeue()
                {
                    match task
                    {
                        P1_T :: do_macro =>
                        {
                            let(_0, _1,) =
                            (& *
                             __rtic_internal_do_macro_INPUTS.get()).get_unchecked(usize
                                                                                  ::
                                                                                  from(index)).as_ptr().read()
                            ;
                            (& mut *
                             __rtic_internal_do_macro_FQ.get_mut()).split().0.enqueue_unchecked(index)
                            ; let priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ;
                            do_macro(do_macro :: Context :: new(priority), _0,
                                     _1)
                        }
                    }
                }
            }) ;
    } #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_TIMER_QUEUE_MARKER
    : rtic :: RacyCell < u32 > = rtic :: RacyCell :: new(0) ; #[doc(hidden)]
    #[allow(non_camel_case_types)] #[derive(Clone, Copy)] pub enum SCHED_T
    { do_macro, } #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_TQ_MyMono : rtic
    :: RacyCell < rtic :: export :: TimerQueue < DwtSystick < FREQ_CORE >,
    SCHED_T, 1 > > = rtic :: RacyCell ::
    new(rtic :: export ::
        TimerQueue(rtic :: export :: SortedLinkedList :: new_u16())) ;
    #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static
    __rtic_internal_MONOTONIC_STORAGE_MyMono : rtic :: RacyCell < Option <
    DwtSystick < FREQ_CORE > >> = rtic :: RacyCell :: new(None) ; #[no_mangle]
    #[allow(non_snake_case)] unsafe fn SysTick()
    {
        while let Some((task, index)) = rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) =
             (& mut *
              __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut()
             {
                 (& mut *
                  __rtic_internal_TQ_MyMono.get_mut()).dequeue(|| core :: mem
                                                               :: transmute ::
                                                               < _, rtic ::
                                                               export :: SYST
                                                               >
                                                               (()).disable_interrupt(),
                                                               mono)
             } else { core :: hint :: unreachable_unchecked() })
        {
            match task
            {
                SCHED_T :: do_macro =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ |
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).split().0.enqueue_unchecked((P1_T
                                                                                        ::
                                                                                        do_macro,
                                                                                        index)))
                    ; rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: EXTI0) ;
                }
            }
        } rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) =
             (& mut *
              __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut()
             { mono.on_interrupt() ; }) ;
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: assert_send :: < MouseKeyboardState > () ; rtic
            :: export :: assert_send :: < MacroConfig > () ; rtic :: export ::
            assert_send :: < UsbDevice < 'static, UsbBus < USB > > > () ; rtic
            :: export :: assert_send :: < HIDClass < 'static, UsbBus < USB > >
            > () ; rtic :: export :: assert_send :: < Button > () ; rtic ::
            export :: assert_send :: < & 'static MacroSequence > () ; rtic ::
            export :: assert_send :: < u8 > () ; rtic :: export ::
            assert_monotonic :: < DwtSystick < FREQ_CORE > > () ; rtic ::
            export :: interrupt :: disable() ;
            (0 ..
             1u8).for_each(| i |
                           (& mut *
                            __rtic_internal_do_macro_FQ.get_mut()).enqueue_unchecked(i))
            ; let mut core : rtic :: export :: Peripherals = rtic :: export ::
            Peripherals :: steal().into() ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: EXTI0 ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI0, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI0) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI15_10, rtic :: export
                                   ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI15_10) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: OTG_FS, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: OTG_FS) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) -
              (1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) as usize)] ;
            core.SCB.set_priority(rtic :: export :: SystemHandler :: SysTick,
                                  rtic :: export ::
                                  logical2hw((1 << stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS), stm32f4 ::
                                             stm32f401 :: NVIC_PRIO_BITS),) ;
            if! < DwtSystick < FREQ_CORE > as rtic :: Monotonic > ::
            DISABLE_INTERRUPT_ON_EMPTY_QUEUE
            {
                core :: mem :: transmute :: < _, rtic :: export :: SYST >
                (()).enable_interrupt() ;
            } #[inline(never)] fn __rtic_init_resources < F > (f : F) where F
            : FnOnce() { f() ; }
            __rtic_init_resources(||
                                  {
                                      let(shared_resources, local_resources,
                                          mut monotonics) =
                                      init(init :: Context ::
                                           new(core.into())) ;
                                      __rtic_internal_shared_resource_mouse.get_mut().write(core
                                                                                            ::
                                                                                            mem
                                                                                            ::
                                                                                            MaybeUninit
                                                                                            ::
                                                                                            new(shared_resources.mouse))
                                      ;
                                      __rtic_internal_shared_resource_macro_conf.get_mut().write(core
                                                                                                 ::
                                                                                                 mem
                                                                                                 ::
                                                                                                 MaybeUninit
                                                                                                 ::
                                                                                                 new(shared_resources.macro_conf))
                                      ;
                                      __rtic_internal_local_resource_usb_dev.get_mut().write(core
                                                                                             ::
                                                                                             mem
                                                                                             ::
                                                                                             MaybeUninit
                                                                                             ::
                                                                                             new(local_resources.usb_dev))
                                      ;
                                      __rtic_internal_local_resource_hid.get_mut().write(core
                                                                                         ::
                                                                                         mem
                                                                                         ::
                                                                                         MaybeUninit
                                                                                         ::
                                                                                         new(local_resources.hid))
                                      ;
                                      __rtic_internal_local_resource_button.get_mut().write(core
                                                                                            ::
                                                                                            mem
                                                                                            ::
                                                                                            MaybeUninit
                                                                                            ::
                                                                                            new(local_resources.button))
                                      ; monotonics.0.reset() ;
                                      __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut().write(Some(monotonics.0))
                                      ; rtic :: export :: interrupt ::
                                      enable() ;
                                  }) ; loop { rtic :: export :: nop() }
        }
    }
}
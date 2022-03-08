(function() {var implementors = {};
implementors["embedded_hal"] = [];
implementors["stm32f4xx_hal"] = [{"text":"impl&lt;MODE, const P:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.char.html\">char</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a>&gt; <a class=\"trait\" href=\"embedded_hal/digital/blocking/trait.ToggleableOutputPin.html\" title=\"trait embedded_hal::digital::blocking::ToggleableOutputPin\">ToggleableOutputPin</a> for <a class=\"struct\" href=\"stm32f4xx_hal/gpio/struct.Pin.html\" title=\"struct stm32f4xx_hal::gpio::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"stm32f4xx_hal/gpio/struct.Output.html\" title=\"struct stm32f4xx_hal::gpio::Output\">Output</a>&lt;MODE&gt;, P, N&gt;","synthetic":false,"types":["stm32f4xx_hal::gpio::Pin"]},{"text":"impl&lt;MODE&gt; <a class=\"trait\" href=\"embedded_hal/digital/blocking/trait.ToggleableOutputPin.html\" title=\"trait embedded_hal::digital::blocking::ToggleableOutputPin\">ToggleableOutputPin</a> for <a class=\"struct\" href=\"stm32f4xx_hal/gpio/struct.ErasedPin.html\" title=\"struct stm32f4xx_hal::gpio::ErasedPin\">ErasedPin</a>&lt;<a class=\"struct\" href=\"stm32f4xx_hal/gpio/struct.Output.html\" title=\"struct stm32f4xx_hal::gpio::Output\">Output</a>&lt;MODE&gt;&gt;","synthetic":false,"types":["stm32f4xx_hal::gpio::erased::ErasedPin"]},{"text":"impl&lt;MODE, const P:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.char.html\">char</a>&gt; <a class=\"trait\" href=\"embedded_hal/digital/blocking/trait.ToggleableOutputPin.html\" title=\"trait embedded_hal::digital::blocking::ToggleableOutputPin\">ToggleableOutputPin</a> for <a class=\"struct\" href=\"stm32f4xx_hal/gpio/struct.PartiallyErasedPin.html\" title=\"struct stm32f4xx_hal::gpio::PartiallyErasedPin\">PartiallyErasedPin</a>&lt;<a class=\"struct\" href=\"stm32f4xx_hal/gpio/struct.Output.html\" title=\"struct stm32f4xx_hal::gpio::Output\">Output</a>&lt;MODE&gt;, P&gt;","synthetic":false,"types":["stm32f4xx_hal::gpio::partially_erased::PartiallyErasedPin"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
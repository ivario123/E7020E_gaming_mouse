(function() {var implementors = {};
implementors["embedded_hal"] = [];
implementors["stm32f4xx_hal"] = [{"text":"impl&lt;SPI, PINS&gt; <a class=\"trait\" href=\"embedded_hal/spi/blocking/trait.Write.html\" title=\"trait embedded_hal::spi::blocking::Write\">Write</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"stm32f4xx_hal/spi/struct.Spi.html\" title=\"struct stm32f4xx_hal::spi::Spi\">Spi</a>&lt;SPI, PINS, <a class=\"struct\" href=\"stm32f4xx_hal/spi/struct.TransferModeNormal.html\" title=\"struct stm32f4xx_hal::spi::TransferModeNormal\">TransferModeNormal</a>&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: <a class=\"trait\" href=\"embedded_hal/spi/nb/trait.FullDuplex.html\" title=\"trait embedded_hal::spi::nb::FullDuplex\">FullDuplex</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a>, Error = <a class=\"enum\" href=\"stm32f4xx_hal/spi/enum.Error.html\" title=\"enum stm32f4xx_hal::spi::Error\">Error</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SPI: <a class=\"trait\" href=\"stm32f4xx_hal/spi/trait.Instance.html\" title=\"trait stm32f4xx_hal::spi::Instance\">Instance</a>,&nbsp;</span>","synthetic":false,"types":["stm32f4xx_hal::spi::Spi"]},{"text":"impl&lt;SPI, PINS&gt; <a class=\"trait\" href=\"embedded_hal/spi/blocking/trait.Write.html\" title=\"trait embedded_hal::spi::blocking::Write\">Write</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"stm32f4xx_hal/spi/struct.Spi.html\" title=\"struct stm32f4xx_hal::spi::Spi\">Spi</a>&lt;SPI, PINS, <a class=\"struct\" href=\"stm32f4xx_hal/spi/struct.TransferModeBidi.html\" title=\"struct stm32f4xx_hal::spi::TransferModeBidi\">TransferModeBidi</a>&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: <a class=\"trait\" href=\"embedded_hal/spi/nb/trait.FullDuplex.html\" title=\"trait embedded_hal::spi::nb::FullDuplex\">FullDuplex</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a>, Error = <a class=\"enum\" href=\"stm32f4xx_hal/spi/enum.Error.html\" title=\"enum stm32f4xx_hal::spi::Error\">Error</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SPI: <a class=\"trait\" href=\"stm32f4xx_hal/spi/trait.Instance.html\" title=\"trait stm32f4xx_hal::spi::Instance\">Instance</a>,&nbsp;</span>","synthetic":false,"types":["stm32f4xx_hal::spi::Spi"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
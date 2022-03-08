(function() {var implementors = {};
implementors["byteorder"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"byteorder/enum.BigEndian.html\" title=\"enum byteorder::BigEndian\">BigEndian</a>","synthetic":false,"types":["byteorder::BigEndian"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"byteorder/enum.LittleEndian.html\" title=\"enum byteorder::LittleEndian\">LittleEndian</a>","synthetic":false,"types":["byteorder::LittleEndian"]}];
implementors["embedded_hal"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"embedded_hal/can/enum.ErrorKind.html\" title=\"enum embedded_hal::can::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["embedded_hal::can::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"embedded_hal/i2c/enum.ErrorKind.html\" title=\"enum embedded_hal::i2c::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["embedded_hal::i2c::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"embedded_hal/i2c/enum.NoAcknowledgeSource.html\" title=\"enum embedded_hal::i2c::NoAcknowledgeSource\">NoAcknowledgeSource</a>","synthetic":false,"types":["embedded_hal::i2c::NoAcknowledgeSource"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"embedded_hal/serial/enum.ErrorKind.html\" title=\"enum embedded_hal::serial::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["embedded_hal::serial::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"embedded_hal/spi/enum.ErrorKind.html\" title=\"enum embedded_hal::spi::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["embedded_hal::spi::ErrorKind"]}];
implementors["encode_unicode"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"encode_unicode/struct.Utf8Char.html\" title=\"struct encode_unicode::Utf8Char\">Utf8Char</a>","synthetic":false,"types":["encode_unicode::utf8_char::Utf8Char"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"encode_unicode/struct.Utf16Char.html\" title=\"struct encode_unicode::Utf16Char\">Utf16Char</a>","synthetic":false,"types":["encode_unicode::utf16_char::Utf16Char"]}];
implementors["heapless"] = [{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"heapless/struct.String.html\" title=\"struct heapless::String\">String</a>&lt;N&gt;","synthetic":false,"types":["heapless::string::String"]},{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,&nbsp;</span>","synthetic":false,"types":["heapless::vec::Vec"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"heapless/pool/singleton/arc/struct.Arc.html\" title=\"struct heapless::pool::singleton::arc::Arc\">Arc</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"heapless/pool/singleton/arc/trait.Pool.html\" title=\"trait heapless::pool::singleton::arc::Pool\">Pool</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::<a class=\"associatedtype\" href=\"heapless/pool/singleton/arc/trait.Pool.html#associatedtype.Data\" title=\"type heapless::pool::singleton::arc::Pool::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,&nbsp;</span>","synthetic":false,"types":["heapless::pool::singleton::arc::Arc"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"heapless/pool/singleton/struct.Box.html\" title=\"struct heapless::pool::singleton::Box\">Box</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"heapless/pool/singleton/trait.Pool.html\" title=\"trait heapless::pool::singleton::Pool\">Pool</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::<a class=\"associatedtype\" href=\"heapless/pool/singleton/trait.Pool.html#associatedtype.Data\" title=\"type heapless::pool::singleton::Pool::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,&nbsp;</span>","synthetic":false,"types":["heapless::pool::singleton::Box"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"heapless/pool/struct.Box.html\" title=\"struct heapless::pool::Box\">Box</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,&nbsp;</span>","synthetic":false,"types":["heapless::pool::Box"]},{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"heapless/spsc/struct.Queue.html\" title=\"struct heapless::spsc::Queue\">Queue</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,&nbsp;</span>","synthetic":false,"types":["heapless::spsc::Queue"]}];
implementors["nb"] = [{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"nb/enum.Error.html\" title=\"enum nb::Error\">Error</a>&lt;E&gt;","synthetic":false,"types":["nb::Error"]}];
implementors["stm32f4xx_hal"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"stm32f4xx_hal/signature/struct.Uid.html\" title=\"struct stm32f4xx_hal::signature::Uid\">Uid</a>","synthetic":false,"types":["stm32f4xx_hal::signature::Uid"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"stm32f4xx_hal/timer/struct.Event.html\" title=\"struct stm32f4xx_hal::timer::Event\">Event</a>","synthetic":false,"types":["stm32f4xx_hal::timer::Event"]}];
implementors["time"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/struct.Date.html\" title=\"struct time::Date\">Date</a>","synthetic":false,"types":["time::date::Date"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>","synthetic":false,"types":["time::duration::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/error/struct.ComponentRange.html\" title=\"struct time::error::ComponentRange\">ComponentRange</a>","synthetic":false,"types":["time::error::component_range::ComponentRange"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"time/enum.Month.html\" title=\"enum time::Month\">Month</a>","synthetic":false,"types":["time::month::Month"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/struct.OffsetDateTime.html\" title=\"struct time::OffsetDateTime\">OffsetDateTime</a>","synthetic":false,"types":["time::offset_date_time::OffsetDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>","synthetic":false,"types":["time::primitive_date_time::PrimitiveDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/struct.Time.html\" title=\"struct time::Time\">Time</a>","synthetic":false,"types":["time::time::Time"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"time/struct.UtcOffset.html\" title=\"struct time::UtcOffset\">UtcOffset</a>","synthetic":false,"types":["time::utc_offset::UtcOffset"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"enum\" href=\"time/enum.Weekday.html\" title=\"enum time::Weekday\">Weekday</a>","synthetic":false,"types":["time::weekday::Weekday"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
(function() {var implementors = {};
implementors["encode_unicode"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"encode_unicode/struct.Utf8Char.html\" title=\"struct encode_unicode::Utf8Char\">Utf8Char</a>","synthetic":false,"types":["encode_unicode::utf8_char::Utf8Char"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"encode_unicode/struct.Utf8Char.html\" title=\"struct encode_unicode::Utf8Char\">Utf8Char</a>","synthetic":false,"types":["encode_unicode::utf8_char::Utf8Char"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u16.html\">u16</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"encode_unicode/struct.Utf16Char.html\" title=\"struct encode_unicode::Utf16Char\">Utf16Char</a>","synthetic":false,"types":["encode_unicode::utf16_char::Utf16Char"]}];
implementors["heapless"] = [{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt;","synthetic":false,"types":["heapless::histbuf::HistoryBuffer"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"heapless/struct.String.html\" title=\"struct heapless::String\">String</a>&lt;N&gt;","synthetic":false,"types":["heapless::string::String"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"heapless/struct.String.html\" title=\"struct heapless::String\">String</a>&lt;N&gt;","synthetic":false,"types":["heapless::string::String"]},{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;","synthetic":false,"types":["heapless::vec::Vec"]},{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;","synthetic":false,"types":["heapless::vec::Vec"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;&lt;P as <a class=\"trait\" href=\"heapless/pool/singleton/arc/trait.Pool.html\" title=\"trait heapless::pool::singleton::arc::Pool\">Pool</a>&gt;::<a class=\"associatedtype\" href=\"heapless/pool/singleton/arc/trait.Pool.html#associatedtype.Data\" title=\"type heapless::pool::singleton::arc::Pool::Data\">Data</a>&gt; for <a class=\"struct\" href=\"heapless/pool/singleton/arc/struct.Arc.html\" title=\"struct heapless::pool::singleton::arc::Arc\">Arc</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"heapless/pool/singleton/arc/trait.Pool.html\" title=\"trait heapless::pool::singleton::arc::Pool\">Pool</a>,&nbsp;</span>","synthetic":false,"types":["heapless::pool::singleton::arc::Arc"]},{"text":"impl&lt;P, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"heapless/pool/singleton/struct.Box.html\" title=\"struct heapless::pool::singleton::Box\">Box</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"heapless/pool/singleton/trait.Pool.html\" title=\"trait heapless::pool::singleton::Pool\">Pool</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::<a class=\"associatedtype\" href=\"heapless/pool/singleton/trait.Pool.html#associatedtype.Data\" title=\"type heapless::pool::singleton::Pool::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::pool::singleton::Box"]},{"text":"impl&lt;A, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"heapless/pool/struct.Box.html\" title=\"struct heapless::pool::Box\">Box</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">]</a>&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::pool::Box"]}];
implementors["stm32f4xx_hal"] = [{"text":"impl&lt;USART, PINS, WORD&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"stm32f4xx_hal/serial/struct.Tx.html\" title=\"struct stm32f4xx_hal::serial::Tx\">Tx</a>&lt;USART, WORD&gt;&gt; for <a class=\"struct\" href=\"stm32f4xx_hal/serial/struct.Serial.html\" title=\"struct stm32f4xx_hal::serial::Serial\">Serial</a>&lt;USART, PINS, WORD&gt;","synthetic":false,"types":["stm32f4xx_hal::serial::Serial"]},{"text":"impl&lt;USART, PINS, WORD&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"stm32f4xx_hal/serial/struct.Rx.html\" title=\"struct stm32f4xx_hal::serial::Rx\">Rx</a>&lt;USART, WORD&gt;&gt; for <a class=\"struct\" href=\"stm32f4xx_hal/serial/struct.Serial.html\" title=\"struct stm32f4xx_hal::serial::Serial\">Serial</a>&lt;USART, PINS, WORD&gt;","synthetic":false,"types":["stm32f4xx_hal::serial::Serial"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
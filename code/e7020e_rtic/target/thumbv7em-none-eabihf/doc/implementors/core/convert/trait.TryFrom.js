(function() {var implementors = {};
implementors["fugit"] = [{"text":"impl&lt;const NOM:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u32.html\">u32</a>, const DENOM:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u32.html\">u32</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"fugit/struct.Duration.html\" title=\"struct fugit::Duration\">Duration</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u64.html\">u64</a>, NOM, DENOM&gt;&gt; for <a class=\"struct\" href=\"fugit/struct.Duration.html\" title=\"struct fugit::Duration\">Duration</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u32.html\">u32</a>, NOM, DENOM&gt;","synthetic":false,"types":["fugit::duration::Duration"]},{"text":"impl&lt;const NOM:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u32.html\">u32</a>, const DENOM:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u32.html\">u32</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"fugit/struct.Rate.html\" title=\"struct fugit::Rate\">Rate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u64.html\">u64</a>, NOM, DENOM&gt;&gt; for <a class=\"struct\" href=\"fugit/struct.Rate.html\" title=\"struct fugit::Rate\">Rate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u32.html\">u32</a>, NOM, DENOM&gt;","synthetic":false,"types":["fugit::rate::Rate"]}];
implementors["heapless"] = [{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.slice.html\">&amp;'a [T]</a>&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;","synthetic":false,"types":["heapless::vec::Vec"]}];
implementors["time"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>","synthetic":false,"types":["time::duration::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">StdDuration</a>","synthetic":false,"types":["core::time::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"enum\" href=\"time/error/enum.Error.html\" title=\"enum time::error::Error\">Error</a>&gt; for <a class=\"struct\" href=\"time/error/struct.ComponentRange.html\" title=\"struct time::error::ComponentRange\">ComponentRange</a>","synthetic":false,"types":["time::error::component_range::ComponentRange"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"enum\" href=\"time/error/enum.Error.html\" title=\"enum time::error::Error\">Error</a>&gt; for <a class=\"struct\" href=\"time/error/struct.ConversionRange.html\" title=\"struct time::error::ConversionRange\">ConversionRange</a>","synthetic":false,"types":["time::error::conversion_range::ConversionRange"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"enum\" href=\"time/error/enum.Error.html\" title=\"enum time::error::Error\">Error</a>&gt; for <a class=\"struct\" href=\"time/error/struct.DifferentVariant.html\" title=\"struct time::error::DifferentVariant\">DifferentVariant</a>","synthetic":false,"types":["time::error::different_variant::DifferentVariant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/core/primitive.u8.html\">u8</a>&gt; for <a class=\"enum\" href=\"time/enum.Month.html\" title=\"enum time::Month\">Month</a>","synthetic":false,"types":["time::month::Month"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
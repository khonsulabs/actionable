(function() {var implementors = {};
implementors["actionable"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"actionable/struct.Statement.html\" title=\"struct actionable::Statement\">Statement</a>&gt; for <a class=\"struct\" href=\"actionable/struct.Permissions.html\" title=\"struct actionable::Permissions\">Permissions</a>","synthetic":false,"types":["actionable::permissions::Permissions"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"actionable/struct.Statement.html\" title=\"struct actionable::Statement\">Statement</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"actionable/struct.Permissions.html\" title=\"struct actionable::Permissions\">Permissions</a>","synthetic":false,"types":["actionable::permissions::Permissions"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u64.html\">u64</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Identifier.html\" title=\"enum actionable::Identifier\">Identifier</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::Identifier"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Identifier.html\" title=\"enum actionable::Identifier\">Identifier</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::Identifier"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Identifier.html\" title=\"enum actionable::Identifier\">Identifier</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::Identifier"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Identifier.html\" title=\"enum actionable::Identifier\">Identifier</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::Identifier"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"enum\" href=\"actionable/enum.ActionNameList.html\" title=\"enum actionable::ActionNameList\">ActionNameList</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"actionable/trait.Action.html\" title=\"trait actionable::Action\">Action</a>,&nbsp;</span>","synthetic":false,"types":["actionable::statement::ActionNameList"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"actionable/enum.ActionNameList.html\" title=\"enum actionable::ActionNameList\">ActionNameList</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"actionable/trait.Action.html\" title=\"trait actionable::Action\">Action</a>,&nbsp;</span>","synthetic":false,"types":["actionable::statement::ActionNameList"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u64.html\">u64</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Configuration.html\" title=\"enum actionable::Configuration\">Configuration</a>","synthetic":false,"types":["actionable::statement::Configuration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i64.html\">i64</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Configuration.html\" title=\"enum actionable::Configuration\">Configuration</a>","synthetic":false,"types":["actionable::statement::Configuration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.58.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Configuration.html\" title=\"enum actionable::Configuration\">Configuration</a>","synthetic":false,"types":["actionable::statement::Configuration"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"actionable/enum.Configuration.html\" title=\"enum actionable::Configuration\">Configuration</a>","synthetic":false,"types":["actionable::statement::Configuration"]},{"text":"impl&lt;'b, 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.slice.html\">&amp;'b [</a><a class=\"enum\" href=\"actionable/enum.Identifier.html\" title=\"enum actionable::Identifier\">Identifier</a>&lt;'a&gt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"actionable/struct.ResourceName.html\" title=\"struct actionable::ResourceName\">ResourceName</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::ResourceName"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"actionable/struct.ResourceName.html\" title=\"struct actionable::ResourceName\">ResourceName</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::ResourceName"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"actionable/struct.ResourceName.html\" title=\"struct actionable::ResourceName\">ResourceName</a>&lt;'a&gt;","synthetic":false,"types":["actionable::statement::ResourceName"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
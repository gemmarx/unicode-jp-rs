var searchIndex = {};
searchIndex["converchar_jp"] = {"doc":"","items":[],"paths":[]};
searchIndex["kana"] = {"doc":"Converters of troublesome characters included in Japanese texts.","items":[[3,"Kana","kana","",null,null],[5,"wide2ascii","","Convert Wide-alphanumeric into normal ASCII  [Ａ -&gt; A]\n# Examples\n```\nassert_eq!(&quot;#&amp;Rust-1.6!&quot;, kana::wide2ascii(&quot;＃＆Ｒｕｓｔ－１．６！&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"ascii2wide","","Convert normal ASCII characters into Wide-alphanumeric  [A -&gt; Ａ]\n# Examples\n```\nassert_eq!(&quot;＃＆Ｒｕｓｔ－１．６！&quot;, kana::ascii2wide(&quot;#&amp;Rust-1.6!&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"hira2kata","","Convert Hiragana into Katakana  [あ -&gt; ア]\n# Examples\n```\nassert_eq!(&quot;イロハ&quot;, kana::hira2kata(&quot;いろは&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"kata2hira","","Convert Katakana into Hiragana  [ア -&gt; あ]\n# Examples\n```\nassert_eq!(&quot;いろは&quot;, kana::kata2hira(&quot;イロハ&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"vsmark2half","","Convert all separated Voiced-sound-marks into half-width style &quot;\\u{FF9E}&quot;\n# Examples\n```\nassert_eq!(&quot;ひﾟひﾞんはﾞ&quot;, kana::vsmark2half(&quot;ひﾟひ゛んは ゙&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"vsmark2full","","Convert all separated Voiced-sound-marks into full-width style &quot;\\u{309B}&quot;\n# Examples\n```\nassert_eq!(&quot;ひ゜ひ゛んは゛&quot;, kana::vsmark2full(&quot;ひﾟひ゛んは ゙&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"vsmark2combi","","Convert all separated Voiced-sound-marks into space+combining style &quot;\\u{20}\\u{3099}&quot;\n# Examples\n```\nassert_eq!(&quot;ひ ゚ひ ゙んは ゙&quot;, kana::vsmark2combi(&quot;ひﾟひ゛んは ゙&quot;));\n```",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"nowidespace","","Convert Wide-space into normal space    [&quot;　&quot; -&gt; &quot; &quot;]",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"space2wide","","Convert normal space into Wide-space    [&quot; &quot; -&gt; &quot;　&quot;]",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"nowideyen","","Convert Wide-yen into Half-width-yen    [&quot;￥&quot; -&gt; &quot;¥&quot;]",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"yen2wide","","Convert Half-width-yen into Wide-yen    [&quot;¥&quot; -&gt; &quot;￥&quot;]",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[11,"init","","",0,{"inputs":[],"output":{"name":"kana"}}],[11,"half2full","","Convert Half-width-kana into normal Katakana with diacritical marks separated  [ｱﾞﾊﾟ -&gt; ア゙パ]",0,null],[11,"half2kana","","Convert Half-width-kana into normal Katakana with diacritical marks combined  [ｱﾞﾊﾟ -&gt; アﾞパ]\n# Examples\n```\nuse kana::Kana;\nlet k = Kana::init();\nassert_eq!(&quot;マツオ バショウ ア ゚&quot;, k.half2kana(&quot;ﾏﾂｵ ﾊﾞｼｮｳ ｱﾟ&quot;));\n```",0,null],[11,"combine","","Combine base characters and diacritical marks on Hiragana/Katakana [かﾞハ゜ -&gt; がパ]\n# Examples\n```\nuse kana::Kana;\nlet k = Kana::init();\nassert_eq!(&quot;ぴびんば&quot;, k.combine(&quot;ひ゜ひ゛んは゛&quot;));\n```",0,null]],"paths":[[3,"Kana"]]};
initSearch(searchIndex);
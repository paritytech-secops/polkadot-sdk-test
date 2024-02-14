(function() {var type_impls = {
"pallet_bags_list":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.metadata\" class=\"method\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><h4 class=\"code-header\">pub fn <a href=\"pallet_bags_list/mock/struct.Runtime.html#tymethod.metadata\" class=\"fn\">metadata</a>() -&gt; RuntimeMetadataPrefixed</h4></section><section id=\"method.metadata_at_version\" class=\"method\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><h4 class=\"code-header\">pub fn <a href=\"pallet_bags_list/mock/struct.Runtime.html#tymethod.metadata_at_version\" class=\"fn\">metadata_at_version</a>(version: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;OpaqueMetadata&gt;</h4></section><section id=\"method.metadata_versions\" class=\"method\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><h4 class=\"code-header\">pub fn <a href=\"pallet_bags_list/mock/struct.Runtime.html#tymethod.metadata_versions\" class=\"fn\">metadata_versions</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>&gt;</h4></section></div></details>",0,"pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Config-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#52-55\">source</a><a href=\"#impl-Config-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Config for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Block\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Block\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Block</a> = Block&lt;Header&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u64.html\">u64</a>, BlakeTwo256&gt;, UncheckedExtrinsic&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::AccountId, &lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::RuntimeCall, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>&gt;&gt;</h4></section></summary><div class='docblock'>The Block type used by the runtime. This is used by <code>construct_runtime</code> to retrieve the\nextrinsics or other block specific data as needed.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.AccountData\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.AccountData\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">AccountData</a> = AccountData&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>&gt;</h4></section></summary><div class='docblock'>Data to be associated with an account (other than nonce/transaction counter, which this\npallet does regardless).</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Nonce\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Nonce\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Nonce</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::Nonce</h4></section></summary><div class='docblock'>This stores the number of previous transactions associated with a sender account.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Hash\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Hash\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Hash</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::Hash</h4></section></summary><div class='docblock'>The output of the <code>Hashing</code> function.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Hashing\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Hashing\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Hashing</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::Hashing</h4></section></summary><div class='docblock'>The hashing system (algorithm) being used in the runtime (e.g. Blake2).</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.AccountId\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.AccountId\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">AccountId</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::AccountId</h4></section></summary><div class='docblock'>The user account identifier type for the runtime.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Lookup\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Lookup\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Lookup</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::Lookup</h4></section></summary><div class='docblock'>Converting trait to take a source type and convert to <code>AccountId</code>. <a>Read more</a></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.MaxConsumers\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.MaxConsumers\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">MaxConsumers</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::MaxConsumers</h4></section></summary><div class='docblock'>The maximum number of consumers allowed on a single account.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.OnNewAccount\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.OnNewAccount\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">OnNewAccount</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::OnNewAccount</h4></section></summary><div class='docblock'>Handler for when a new account has just been created.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.OnKilledAccount\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.OnKilledAccount\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">OnKilledAccount</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::OnKilledAccount</h4></section></summary><div class='docblock'>A function that is invoked when an account has been determined to be dead. <a>Read more</a></div></details><section id=\"associatedtype.SystemWeightInfo\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.SystemWeightInfo\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">SystemWeightInfo</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::SystemWeightInfo</h4></section><details class=\"toggle\" open><summary><section id=\"associatedtype.SS58Prefix\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.SS58Prefix\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">SS58Prefix</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::SS58Prefix</h4></section></summary><div class='docblock'>The designated SS58 prefix of this chain. <a>Read more</a></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Version\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Version\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Version</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::Version</h4></section></summary><div class='docblock'>Get the chain’s current version.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.BlockWeights\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.BlockWeights\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">BlockWeights</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::BlockWeights</h4></section></summary><div class='docblock'>Block &amp; extrinsics weights: base values and limits.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.BlockLength\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.BlockLength\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">BlockLength</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::BlockLength</h4></section></summary><div class='docblock'>The maximum length of a block (in bytes).</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.DbWeight\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.DbWeight\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">DbWeight</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::DbWeight</h4></section></summary><div class='docblock'>The weight of runtime database operations the runtime can invoke.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.RuntimeEvent\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeEvent\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeEvent</a> = <a class=\"enum\" href=\"pallet_bags_list/mock/enum.RuntimeEvent.html\" title=\"enum pallet_bags_list::mock::RuntimeEvent\">RuntimeEvent</a></h4></section></summary><div class='docblock'>The aggregated event type of the runtime.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.RuntimeOrigin\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeOrigin\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeOrigin</a> = <a class=\"struct\" href=\"pallet_bags_list/mock/struct.RuntimeOrigin.html\" title=\"struct pallet_bags_list::mock::RuntimeOrigin\">RuntimeOrigin</a></h4></section></summary><div class='docblock'>The <code>RuntimeOrigin</code> type used by dispatchable calls.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.RuntimeCall\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeCall\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeCall</a> = <a class=\"enum\" href=\"pallet_bags_list/mock/enum.RuntimeCall.html\" title=\"enum pallet_bags_list::mock::RuntimeCall\">RuntimeCall</a></h4></section></summary><div class='docblock'>The aggregated <code>RuntimeCall</code> type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.PalletInfo\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.PalletInfo\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">PalletInfo</a> = <a class=\"struct\" href=\"pallet_bags_list/mock/struct.PalletInfo.html\" title=\"struct pallet_bags_list::mock::PalletInfo\">PalletInfo</a></h4></section></summary><div class='docblock'>Provides information about the pallet setup in the runtime. <a>Read more</a></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.RuntimeTask\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeTask\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeTask</a> = <a class=\"enum\" href=\"pallet_bags_list/mock/enum.RuntimeTask.html\" title=\"enum pallet_bags_list::mock::RuntimeTask\">RuntimeTask</a></h4></section></summary><div class='docblock'>The aggregated <code>RuntimeTask</code> type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.BaseCallFilter\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.BaseCallFilter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">BaseCallFilter</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::BaseCallFilter</h4></section></summary><div class='docblock'>The basic call filter to use in Origin. All origins are built with this filter as base,\nexcept Root. <a>Read more</a></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.BlockHashCount\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.BlockHashCount\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">BlockHashCount</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::BlockHashCount</h4></section></summary><div class='docblock'>Maximum number of block number to block hash mappings to keep (oldest pruned first).</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.OnSetCode\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.OnSetCode\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">OnSetCode</a> = &lt;TestDefaultConfig as DefaultConfig&gt;::OnSetCode</h4></section></summary><div class='docblock'>What to do if the runtime wants to change the code to something new. <a>Read more</a></div></details></div></details>","Config","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-PartialEq-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.75.0/src/core/cmp.rs.html#239\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-EnsureInherentsAreFirst%3C%3CRuntime+as+Config%3E::Block%3E-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-EnsureInherentsAreFirst%3C%3CRuntime+as+Config%3E::Block%3E-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl EnsureInherentsAreFirst&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::Block&gt; for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.ensure_inherents_are_first\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.ensure_inherents_are_first\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">ensure_inherents_are_first</a>(\n    block: &amp;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::Block\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>&gt;</h4></section></summary><div class='docblock'>Ensure the position of inherent is correct, i.e. they are before non-inherents. <a>Read more</a></div></details></div></details>","EnsureInherentsAreFirst<<Runtime as Config>::Block>","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TypeInfo-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-TypeInfo-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl TypeInfo for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Identity\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Identity\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Identity</a> = <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h4></section></summary><div class='docblock'>The type identifying for which type info is provided. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_info\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.type_info\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">type_info</a>() -&gt; Type</h4></section></summary><div class='docblock'>Returns the static type identifier for <code>Self</code>.</div></details></div></details>","TypeInfo","pallet_bags_list::mock::AccountId"],["<section id=\"impl-Eq-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-Eq-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section>","Eq","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ScoreProvider%3C%3CRuntime+as+Config%3E::AccountId%3E-for-StakingMock\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#37-49\">source</a><a href=\"#impl-ScoreProvider%3C%3CRuntime+as+Config%3E::AccountId%3E-for-StakingMock\" class=\"anchor\">§</a><h3 class=\"code-header\">impl ScoreProvider&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::AccountId&gt; for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.StakingMock.html\" title=\"struct pallet_bags_list::mock::StakingMock\">StakingMock</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Score\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Score\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Score</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u64.html\">u64</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.score\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#40-42\">source</a><a href=\"#method.score\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">score</a>(id: &amp;<a class=\"type\" href=\"pallet_bags_list/mock/type.AccountId.html\" title=\"type pallet_bags_list::mock::AccountId\">AccountId</a>) -&gt; Self::Score</h4></section></summary><div class='docblock'>Get the current <code>Score</code> of <code>who</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_score_of\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#45-47\">source</a><a href=\"#method.set_score_of\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_score_of</a>(id: &amp;<a class=\"type\" href=\"pallet_bags_list/mock/type.AccountId.html\" title=\"type pallet_bags_list::mock::AccountId\">AccountId</a>, weight: Self::Score)</h4></section></summary><div class='docblock'>For tests, benchmarks and fuzzing, set the <code>score</code>.</div></details></div></details>","ScoreProvider<<Runtime as Config>::AccountId>","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ValidateUnsigned-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-ValidateUnsigned-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl ValidateUnsigned for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Call\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Call\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Call</a> = <a class=\"enum\" href=\"pallet_bags_list/mock/enum.RuntimeCall.html\" title=\"enum pallet_bags_list::mock::RuntimeCall\">RuntimeCall</a></h4></section></summary><div class='docblock'>The call to validate</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pre_dispatch\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.pre_dispatch\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">pre_dispatch</a>(call: &amp;Self::Call) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>, TransactionValidityError&gt;</h4></section></summary><div class='docblock'>Validate the call right before dispatch. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.validate_unsigned\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.validate_unsigned\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">validate_unsigned</a>(\n    source: TransactionSource,\n    call: &amp;Self::Call\n) -&gt; TransactionValidity</h4></section></summary><div class='docblock'>Return the validity of the call <a>Read more</a></div></details></div></details>","ValidateUnsigned","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-Debug-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, fmt: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CallerTrait%3C%3CRuntime+as+Config%3E::AccountId%3E-for-OriginCaller\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-CallerTrait%3C%3CRuntime+as+Config%3E::AccountId%3E-for-OriginCaller\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CallerTrait&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::AccountId&gt; for <a class=\"enum\" href=\"pallet_bags_list/mock/enum.OriginCaller.html\" title=\"enum pallet_bags_list::mock::OriginCaller\">OriginCaller</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_system\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.into_system\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">into_system</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;RawOrigin&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::AccountId&gt;&gt;</h4></section></summary><div class='docblock'>Extract the signer from the message if it is a <code>Signed</code> origin.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_system_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.as_system_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">as_system_ref</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;RawOrigin&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::AccountId&gt;&gt;</h4></section></summary><div class='docblock'>Extract a reference to the system-level <code>RawOrigin</code> if it is that.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_signed\" class=\"method trait-impl\"><a href=\"#method.as_signed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">as_signed</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;AccountId</a>&gt;</h4></section></summary><div class='docblock'>Extract the signer from it if a system <code>Signed</code> origin, <code>None</code> otherwise.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_root\" class=\"method trait-impl\"><a href=\"#method.is_root\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_root</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns <code>true</code> if <code>self</code> is a system <code>Root</code> origin, <code>None</code> otherwise.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_none\" class=\"method trait-impl\"><a href=\"#method.is_none\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_none</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns <code>true</code> if <code>self</code> is a system <code>None</code> origin, <code>None</code> otherwise.</div></details></div></details>","CallerTrait<<Runtime as Config>::AccountId>","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Config-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#61-67\">source</a><a href=\"#impl-Config-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"pallet_bags_list/pallet/trait.Config.html\" title=\"trait pallet_bags_list::pallet::Config\">Config</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.RuntimeEvent\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeEvent\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"pallet_bags_list/pallet/trait.Config.html#associatedtype.RuntimeEvent\" class=\"associatedtype\">RuntimeEvent</a> = <a class=\"enum\" href=\"pallet_bags_list/mock/enum.RuntimeEvent.html\" title=\"enum pallet_bags_list::mock::RuntimeEvent\">RuntimeEvent</a></h4></section></summary><div class='docblock'>The overarching event type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.WeightInfo\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.WeightInfo\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"pallet_bags_list/pallet/trait.Config.html#associatedtype.WeightInfo\" class=\"associatedtype\">WeightInfo</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a></h4></section></summary><div class='docblock'>Weight information for extrinsics in this pallet.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.BagThresholds\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.BagThresholds\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"pallet_bags_list/pallet/trait.Config.html#associatedtype.BagThresholds\" class=\"associatedtype\">BagThresholds</a> = <a class=\"struct\" href=\"pallet_bags_list/mock/struct.BagThresholds.html\" title=\"struct pallet_bags_list::mock::BagThresholds\">BagThresholds</a></h4></section></summary><div class='docblock'>The list of thresholds separating the various bags. <a href=\"pallet_bags_list/pallet/trait.Config.html#associatedtype.BagThresholds\">Read more</a></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.ScoreProvider\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.ScoreProvider\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"pallet_bags_list/pallet/trait.Config.html#associatedtype.ScoreProvider\" class=\"associatedtype\">ScoreProvider</a> = <a class=\"struct\" href=\"pallet_bags_list/mock/struct.StakingMock.html\" title=\"struct pallet_bags_list::mock::StakingMock\">StakingMock</a></h4></section></summary><div class='docblock'>Something that provides the scores of ids.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Score\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Score\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"pallet_bags_list/pallet/trait.Config.html#associatedtype.Score\" class=\"associatedtype\">Score</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>The type used to dictate a node position relative to other nodes.</div></details></div></details>","Config","pallet_bags_list::mock::AccountId"],["<section id=\"impl-StructuralPartialEq-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-StructuralPartialEq-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section>","StructuralPartialEq","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GetRuntimeBlockType-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-GetRuntimeBlockType-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl GetRuntimeBlockType for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.RuntimeBlock\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeBlock\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeBlock</a> = &lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as Config&gt;::Block</h4></section></summary><div class='docblock'>The <code>RuntimeBlock</code> type.</div></details></div></details>","GetRuntimeBlockType","pallet_bags_list::mock::AccountId"],["<section id=\"impl-StructuralEq-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-StructuralEq-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.StructuralEq.html\" title=\"trait core::marker::StructuralEq\">StructuralEq</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section>","StructuralEq","pallet_bags_list::mock::AccountId"],["<section id=\"impl-Copy-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-Copy-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section>","Copy","pallet_bags_list::mock::AccountId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Runtime\" class=\"impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#impl-Clone-for-Runtime\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pallet_bags_list/mock.rs.html#70-75\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.75.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","pallet_bags_list::mock::AccountId"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()
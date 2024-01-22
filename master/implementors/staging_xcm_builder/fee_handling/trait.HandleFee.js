(function() {var implementors = {
"bridge_hub_rococo_runtime":[["impl&lt;AssetTransactor: <a class=\"trait\" href=\"staging_xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait staging_xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a>, DestNetwork: Get&lt;<a class=\"enum\" href=\"staging_xcm/v4/junction/enum.NetworkId.html\" title=\"enum staging_xcm::v4::junction::NetworkId\">NetworkId</a>&gt;, DestParaId: Get&lt;<a class=\"struct\" href=\"polkadot_parachain_primitives/primitives/struct.Id.html\" title=\"struct polkadot_parachain_primitives::primitives::Id\">ParaId</a>&gt;, DestBridgedChainId: Get&lt;<a class=\"type\" href=\"bp_runtime/type.ChainId.html\" title=\"type bp_runtime::ChainId\">ChainId</a>&gt;, BridgeLaneId: Get&lt;<a class=\"struct\" href=\"bp_messages/struct.LaneId.html\" title=\"struct bp_messages::LaneId\">LaneId</a>&gt;&gt; <a class=\"trait\" href=\"staging_xcm_builder/fee_handling/trait.HandleFee.html\" title=\"trait staging_xcm_builder::fee_handling::HandleFee\">HandleFee</a> for <a class=\"struct\" href=\"bridge_hub_rococo_runtime/xcm_config/struct.XcmExportFeeToRelayerRewardAccounts.html\" title=\"struct bridge_hub_rococo_runtime::xcm_config::XcmExportFeeToRelayerRewardAccounts\">XcmExportFeeToRelayerRewardAccounts</a>&lt;AssetTransactor, DestNetwork, DestParaId, DestBridgedChainId, BridgeLaneId&gt;"]],
"snowbridge_runtime_common":[["impl&lt;Balance, AccountId, FeeAssetLocation, EthereumNetwork, AssetTransactor, FeeProvider&gt; HandleFee for <a class=\"struct\" href=\"snowbridge_runtime_common/struct.XcmExportFeeToSibling.html\" title=\"struct snowbridge_runtime_common::XcmExportFeeToSibling\">XcmExportFeeToSibling</a>&lt;Balance, AccountId, FeeAssetLocation, EthereumNetwork, AssetTransactor, FeeProvider&gt;<span class=\"where fmt-newline\">where\n    Balance: <a class=\"trait\" href=\"sp_arithmetic/traits/trait.BaseArithmetic.html\" title=\"trait sp_arithmetic::traits::BaseArithmetic\">BaseArithmetic</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/sign/trait.Unsigned.html\" title=\"trait num_traits::sign::Unsigned\">Unsigned</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u128.html\">u128</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u128.html\">u128</a>&gt;,\n    AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.array.html\">32</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.array.html\">32</a>]&gt;,\n    FeeAssetLocation: Get&lt;<a class=\"struct\" href=\"staging_xcm/v4/location/struct.Location.html\" title=\"struct staging_xcm::v4::location::Location\">Location</a>&gt;,\n    EthereumNetwork: Get&lt;<a class=\"enum\" href=\"staging_xcm/v4/junction/enum.NetworkId.html\" title=\"enum staging_xcm::v4::junction::NetworkId\">NetworkId</a>&gt;,\n    AssetTransactor: TransactAsset,\n    FeeProvider: SendMessageFeeProvider&lt;Balance = Balance&gt;,</span>"]],
"staging_xcm_builder":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
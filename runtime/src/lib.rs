//! A minimal runtime that includes the template [`pezpallet`](`pezpallet_minimal_template`).
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

use alloc::vec::Vec;

// Use statements for external crates (from pezkuwi-sdk, patched via Cargo.toml)
use pezkuwi_sdk::{
	pezkuwi_sdk_frame as pezframe, // Keep the rebranded alias
	*,
};
use pezframe_support::traits::{FixedFee, Get, NoFee};
use pezpallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use pezsp_keyring::Sr25519Keyring;
use pezframe_support::genesis_builder_helper::{build_state, get_preset};

// Corrected imports for types that were previously unresolved
use pezframe_support::runtime::{ExtrinsicInclusionMode, NativeVersion, RuntimeVersion, RUNTIME_API_VERSIONS};
use pezframe_support::{OpaqueMetadata, ApplyExtrinsicResult, CheckInherentsResult};
use pezframe_support::transaction_validity::{TransactionSource, TransactionValidity};
use pezframe_support::inherent::InherentData;
use pezframe_support::ExtrinsicFor;
use pezsp_weights::Weight;
use pezframe::runtime::apis::PresetId;

/// Provides getters for genesis configuration presets.
pub mod genesis_config_presets {
	use super::*;
	use alloc::{vec, vec::Vec};
	use serde_json::Value;

	/// Returns a development genesis config preset.
	pub fn development_config_genesis() -> Value {
		let endowment = <interface::MinimumBalance as Get<interface::Balance>>::get().max(1) * 1000;
		pezframe_support::build_struct_json_patch!(runtime::RuntimeGenesisConfig {
			balances: runtime::BalancesConfig {
				balances: Sr25519Keyring::iter()
					.map(|a| (a.to_account_id(), endowment))
					.collect::<Vec<_>>(),
			},
			sudo: runtime::SudoConfig { key: Some(Sr25519Keyring::Alice.to_account_id()) },
		})
	}

	/// Get the set of the available genesis config presets.
	pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
		let patch = match id.as_ref() {
			pezframe::deps::pezsp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
			_ => return None,
		};
		Some(
			serde_json::to_string(&patch)
				.expect("serialization to json is expected to work. qed.")
				.into_bytes(),
		)
	}

	/// List of supported presets.
	pub fn preset_names() -> Vec<PresetId> {
		vec![PresetId::from(pezframe::deps::pezsp_genesis_builder::DEV_RUNTIME_PRESET)]
	}
}

pezframe_support::runtime_version! {
	pub const VERSION: RuntimeVersion = RuntimeVersion {
		spec_name: alloc::borrow::Cow::Borrowed("pez-minimal-template-runtime"),
		impl_name: alloc::borrow::Cow::Borrowed("pez-minimal-template-runtime"),
		authoring_version: 1,
		spec_version: 0,
		impl_version: 1,
		apis: RUNTIME_API_VERSIONS,
		transaction_version: 1,
		system_version: 1,
	};
}

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

type TxExtension = (
	pezframe_system::CheckNonZeroSender<runtime::Runtime>,
	pezframe_system::CheckSpecVersion<runtime::Runtime>,
	pezframe_system::CheckTxVersion<runtime::Runtime>,
	pezframe_system::CheckGenesis<runtime::Runtime>,
	pezframe_system::CheckEra<runtime::Runtime>,
	pezframe_system::CheckNonce<runtime::Runtime>,
	pezframe_system::CheckWeight<runtime::Runtime>,
	pezpallet_transaction_payment::ChargeTransactionPayment<runtime::Runtime>,
	pezframe_system::WeightReclaim<runtime::Runtime>,
);

pezframe_support::construct_runtime! {
	pub enum Runtime {
		System: pezframe_system,
		Timestamp: pezpallet_timestamp,
		Balances: pezpallet_balances,
		Sudo: pezpallet_sudo,
		TransactionPayment: pezpallet_transaction_payment,
		Template: pezpallet_minimal_template,
	}
}

pub use runtime::{
	Call as RuntimeCall,
	Event as RuntimeEvent,
	Error as RuntimeError,
	Origin as RuntimeOrigin,
	Runtime,
	AllPalletsWithSystem,
	BalancesConfig,
	SudoConfig,
	System,
	Timestamp,
	Balances,
	Sudo,
	TransactionPayment,
	Template,
	RuntimeGenesisConfig,
};

pezframe_support::parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

mod system_config {
	use super::*;
	#[pezframe_support::derive_impl(pezframe_system::config_preludes::SolochainDefaultConfig)]
	impl pezframe_system::Config for Runtime {
		type Block = Block;
		type Version = Version;
		type AccountData = pezpallet_balances::AccountData<<Runtime as pezpallet_balances::Config>::Balance>;
	}
}

mod balances_config {
	use super::*;
	#[pezframe_support::derive_impl(pezpallet_balances::config_preludes::TestDefaultConfig)]
	impl pezpallet_balances::Config for Runtime {
		type AccountStore = System;
	}
}

mod sudo_config {
	use super::*;
	#[pezframe_support::derive_impl(pezpallet_sudo::config_preludes::TestDefaultConfig)]
	impl pezpallet_sudo::Config for Runtime {}
}

mod timestamp_config {
	use super::*;
	#[pezframe_support::derive_impl(pezpallet_timestamp::config_preludes::TestDefaultConfig)]
	impl pezpallet_timestamp::Config for Runtime {}
}

mod transaction_payment_config {
	use super::*;
	#[pezframe_support::derive_impl(pezpallet_transaction_payment::config_preludes::TestDefaultConfig)]
	impl pezpallet_transaction_payment::Config for Runtime {
		type OnChargeTransaction = pezpallet_transaction_payment::FungibleAdapter<Balances, ()>;
		type WeightToFee = NoFee<<Self as pezpallet_balances::Config>::Balance>;
		type LengthToFee = FixedFee<1, <Self as pezpallet_balances::Config>::Balance>;
	}
}

impl pezpallet_minimal_template::Config for Runtime {}

type Block = pezframe::runtime::types_common::BlockOf<Runtime, TxExtension>;
type Header = pezframe::runtime::prelude::HeaderFor<Runtime>;

type RuntimeExecutive = pezframe::runtime::prelude::Executive<Runtime, Block, pezframe_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

pezsp_api::impl_runtime_apis! {
	impl apis::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}
		fn execute_block(block: Block) {
			RuntimeExecutive::execute_block(block)
		}
		fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
			RuntimeExecutive::initialize_block(header)
		}
	}
	impl apis::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}
		fn metadata_versions() -> Vec<u32> {
			Runtime::metadata_versions()
		}
	}
	impl apis::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
			RuntimeExecutive::apply_extrinsic(extrinsic)
		}
		fn finalize_block() -> HeaderFor<Runtime> {
			RuntimeExecutive::finalize_block()
		}
		fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
			data.create_extrinsics()
		}
		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}
	impl apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: ExtrinsicFor<Runtime>,
			block_hash: <Runtime as pezframe_system::Config>::Hash,
		) -> TransactionValidity {
			RuntimeExecutive::validate_transaction(source, tx, block_hash)
		}
	}
	impl apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &HeaderFor<Runtime>) {
			RuntimeExecutive::offchain_worker(header)
		}
	}
	impl apis::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}
		fn decode_session_keys(_encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, pezframe::runtime::apis::KeyTypeId)>> {
			Default::default()
		}
	}
	impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
		fn account_nonce(account: interface::AccountId) -> interface::Nonce {
			System::account_nonce(account)
		}
	}
	impl pezpallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		interface::Balance,
	> for Runtime {
		fn query_info(uxt: ExtrinsicFor<Runtime>, len: u32) -> RuntimeDispatchInfo<interface::Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: ExtrinsicFor<Runtime>, len: u32) -> FeeDetails<interface::Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> interface::Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> interface::Balance {
			TransactionPayment::length_to_fee(length)
		}
	}
	impl apis::GenesisBuilder<Block> for Runtime {
		fn build_state(config: Vec<u8>) -> pezframe::deps::pezsp_genesis_builder::Result {
			build_state::<RuntimeGenesisConfig>(config)
		}
		fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
			get_preset::<RuntimeGenesisConfig>(id, self::genesis_config_presets::get_preset)
		}
		fn preset_names() -> Vec<PresetId> {
			self::genesis_config_presets::preset_names()
		}
	}
}
pub mod interface {
	use super::runtime::Runtime;
	use pezkuwi_sdk::pezkuwi_sdk_pezframe as pezframe;

	pub type Block = super::Block;
	pub use pezframe::runtime::types_common::OpaqueBlock;
	pub type AccountId = <Runtime as pezframe_system::Config>::AccountId;
	pub type Nonce = <Runtime as pezframe_system::Config>::Nonce;
	pub type Hash = <Runtime as pezframe_system::Config>::Hash;
	pub type Balance = <Runtime as pezpallet_balances::Config>::Balance;
	pub type MinimumBalance = <Runtime as pezpallet_balances::Config>::ExistentialDeposit;
}

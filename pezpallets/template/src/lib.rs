//! A shell pezpallet built with [`pezframe`].
//!
//! To get started with this pezpallet, try implementing the guide in
//! <https://github.com/pezkuwichain/pezkuwi-sdk>

#![cfg_attr(not(feature = "std"), no_std)]

use pezkuwi_sdk::pezkuwi_sdk_frame::deps::pezframe_support::pezpallet_prelude::*;

// Re-export all pezpallet parts, this is needed to properly import the pezpallet into the runtime.
pub use pallet::*;

#[pezkuwi_sdk::pezframe_support::pezpallet]
pub mod pallet {
	use super::*;

	#[pezpallet::config]
	pub trait Config: pezkuwi_sdk::pezframe_system::Config {}

	#[pezpallet::pezpallet]
	pub struct Pezpallet<T>(_);

	#[pezpallet::storage]
	pub type Value<T> = StorageValue<Value = u32>;
}

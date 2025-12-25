//! A shell pezpallet built with `pezframe`.
#![cfg_attr(not(feature = "std"), no_std)]

// The preludes must be consistently rebranded as you instructed.
use pezframe_support::pezpallet_prelude::*;
use pezframe_system::pezpallet_prelude::*;

// We export the inner `pezpallet` module.
pub use self::pezpallet::*;

// The main macro is `pezpallet`, and dev_mode is used to handle weight/index warnings.
#[pezframe_support::pezpallet(dev_mode)]
// The module name is `pezpallet`.
pub mod pezpallet {
	use super::*;

	// All inner attributes must be consistently `#[pezpallet::...]`
	#[pezpallet::config]
	pub trait Config: pezkuwi_sdk::pezframe_system::Config {}

	#[pezpallet::pezpallet]
	// The struct name must be `Pezpallet`.
	pub struct Pezpallet<T>(core::marker::PhantomData<T>);

	#[pezpallet::storage]
	#[pezpallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pezpallet::call]
	impl<T: Config> Pezpallet<T> {
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			ensure_signed(origin)?;
			Something::<T>::put(something);
			Ok(())
		}
	}
}
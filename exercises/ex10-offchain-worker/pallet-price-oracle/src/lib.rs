#![cfg_attr(not(feature = "std"), no_std)]

mod types;

mod offchain_worker;

pub use pallet::*;

use frame_support::log;
use sp_std::vec::Vec;

extern crate alloc;
use alloc::string::String;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::FixedI64;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + frame_system::offchain::SendTransactionTypes<Call<Self>>
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: From<Call<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn btc_price)]
	pub(super) type BTCPrice<T: Config> = StorageValue<_, FixedI64, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BtcPriceSet(FixedI64),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_n: BlockNumberFor<T>) {
			// TODO: call `fetch_btc_price_and_send_unsigned_transaction` and log any error
			let btc_price_result = Self::fetch_btc_price_and_send_unsigned_transaction();
			match btc_price_result {
				Err(err_string) => log::info!("{}", err_string),
				_ => (),
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		OriginErro,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_btc_price(origin: OriginFor<T>, btc_price: FixedI64) -> DispatchResult {
			// TODO:
			// - ensure origin is none
			let _ = ensure_none(origin);
			// - set BTCPrice storage
			// - emit `BtcPriceSet` event
			<BTCPrice<T>>::put(btc_price);
			Self::deposit_event(Event::<T>::BtcPriceSet(btc_price));
			

			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// TODO: implemente some kind of validation
			// It should accept calls to `set_btc_price` and refuse any other

			let valid_tx = |provide| {
				ValidTransaction::with_tag_prefix("pallet-oracle")
					.priority(1)
					.and_provides([&provide])
					.longevity(2)
					.propagate(true)
					.build()
			};

			match call {
				Call::set_btc_price { .. } => valid_tx(b"set_btc_price".to_vec()),
				_ => InvalidTransaction::Call.into(),
			}

			// InvalidTransaction::Call.into()
		}
	}
}

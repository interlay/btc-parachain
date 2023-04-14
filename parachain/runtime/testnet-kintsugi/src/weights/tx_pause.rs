
//! Autogenerated weights for tx_pause
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-13, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sander-dell`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-testnet-latest"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-testnet-latest
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 100
// --repeat
// 10
// --output
// parachain/runtime/testnet-kintsugi/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for tx_pause using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> tx_pause::WeightInfo for WeightInfo<T> {
	/// Storage: TxPause PausedCalls (r:2 w:1)
	/// Proof: TxPause PausedCalls (max_values: None, max_size: Some(277), added: 2752, mode: MaxEncodedLen)
	fn pause() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1023`
		//  Estimated: `5504`
		// Minimum execution time: 27_278_000 picoseconds.
		Weight::from_parts(28_045_000, 5504)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: TxPause PausedCalls (r:2 w:1)
	/// Proof: TxPause PausedCalls (max_values: None, max_size: Some(277), added: 2752, mode: MaxEncodedLen)
	fn unpause() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1328`
		//  Estimated: `5504`
		// Minimum execution time: 30_430_000 picoseconds.
		Weight::from_parts(30_984_000, 5504)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
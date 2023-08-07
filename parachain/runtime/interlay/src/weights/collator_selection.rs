
//! Autogenerated weights for collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-07, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-rust-runner-2mz2v-jrrg4`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// parachain/runtime/interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for collator_selection using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> collator_selection::WeightInfo for WeightInfo<T> {

	/// Storage: Session NextKeys (r:100 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn set_invulnerables	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252 + b * (78 ±0)`
		//  Estimated: `1241 + b * (2554 ±0)`
		// Minimum execution time: 28_387_000 picoseconds.
		Weight::from_parts(19_586_047, 1241)
			// Standard Error: 104_765
			.saturating_add(Weight::from_parts(5_569_637, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 17_825_000 picoseconds.
		Weight::from_parts(24_970_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 26_052_000 picoseconds.
		Weight::from_parts(27_986_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointEpoch (r:1 w:0)
	/// Proof: Escrow UserPointEpoch (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointHistory (r:1 w:0)
	/// Proof: Escrow UserPointHistory (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Escrow Reserved (r:1 w:1)
	/// Proof: Escrow Reserved (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 19]`.
	fn register_as_candidate	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `902 + c * (96 ±0)`
		//  Estimated: `49487 + c * (97 ±0)`
		// Minimum execution time: 78_588_000 picoseconds.
		Weight::from_parts(82_310_845, 49487)
			// Standard Error: 88_497
			.saturating_add(Weight::from_parts(487_436, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 97).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: Escrow Reserved (r:1 w:1)
	/// Proof: Escrow Reserved (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[6, 20]`.
	fn leave_intent	(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402 + c * (64 ±0)`
		//  Estimated: `49487`
		// Minimum execution time: 41_573_000 picoseconds.
		Weight::from_parts(42_921_116, 49487)
			// Standard Error: 17_308
			.saturating_add(Weight::from_parts(209_885, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `650`
		//  Estimated: `6170`
		// Minimum execution time: 72_686_000 picoseconds.
		Weight::from_parts(73_928_000, 6170)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:0)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:20 w:0)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointEpoch (r:20 w:0)
	/// Proof: Escrow UserPointEpoch (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Escrow UserPointHistory (r:20 w:0)
	/// Proof: Escrow UserPointHistory (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Escrow Reserved (r:15 w:15)
	/// Proof: Escrow Reserved (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `c` is `[1, 20]`.
	fn new_session	(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `793 + c * (194 ±0) + r * (47 ±0)`
		//  Estimated: `49487 + c * (2563 ±0) + r * (2207 ±23)`
		// Minimum execution time: 51_603_000 picoseconds.
		Weight::from_parts(53_327_000, 49487)
			// Standard Error: 385_035
			.saturating_add(Weight::from_parts(25_609_068, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2563).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2207).saturating_mul(r.into()))
	}
}
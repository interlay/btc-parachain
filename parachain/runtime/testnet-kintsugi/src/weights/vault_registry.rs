
//! Autogenerated weights for vault_registry
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

/// Weights for vault_registry using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> vault_registry::WeightInfo for WeightInfo<T> {
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry PremiumRedeemThreshold (r:1 w:0)
	/// Proof: VaultRegistry PremiumRedeemThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry LiquidationCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry LiquidationCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry MinimumCollateralVault (r:1 w:0)
	/// Proof: VaultRegistry MinimumCollateralVault (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	/// Proof: VaultRegistry SystemCollateralCeiling (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry VaultBitcoinPublicKey (r:1 w:0)
	/// Proof: VaultRegistry VaultBitcoinPublicKey (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:1)
	/// Proof Skipped: VaultCapacity Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultCapacity RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof Skipped: VaultCapacity RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultCapacity TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards Stake (r:1 w:1)
	/// Proof Skipped: VaultRewards Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultRewards RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof Skipped: VaultRewards RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultRewards TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof Skipped: Fee Commission (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof Skipped: VaultStaking Nonce (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	/// Proof Skipped: VaultStaking TotalCurrentStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking Stake (r:1 w:1)
	/// Proof Skipped: VaultStaking Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking SlashPerToken (r:1 w:0)
	/// Proof Skipped: VaultStaking SlashPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking SlashTally (r:1 w:1)
	/// Proof Skipped: VaultStaking SlashTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking TotalStake (r:1 w:1)
	/// Proof Skipped: VaultStaking TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking RewardTally (r:2 w:2)
	/// Proof Skipped: VaultStaking RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultStaking RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards TotalStake (r:1 w:1)
	/// Proof Skipped: VaultRewards TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardCurrencies (r:1 w:0)
	/// Proof Skipped: VaultRewards RewardCurrencies (max_values: None, max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity TotalStake (r:1 w:1)
	/// Proof Skipped: VaultCapacity TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardCurrencies (r:1 w:0)
	/// Proof Skipped: VaultCapacity RewardCurrencies (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn register_vault() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2936`
		//  Estimated: `165317`
		// Minimum execution time: 271_455_000 picoseconds.
		Weight::from_parts(278_743_000, 165317)
			.saturating_add(T::DbWeight::get().reads(40_u64))
			.saturating_add(T::DbWeight::get().writes(21_u64))
	}
	/// Storage: VaultRegistry VaultBitcoinPublicKey (r:1 w:1)
	/// Proof: VaultRegistry VaultBitcoinPublicKey (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	fn register_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1417`
		//  Estimated: `2556`
		// Minimum execution time: 26_006_000 picoseconds.
		Weight::from_parts(27_984_000, 2556)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:0)
	/// Proof Skipped: VaultCapacity Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultCapacity RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof Skipped: VaultCapacity RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultCapacity TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards Stake (r:1 w:0)
	/// Proof Skipped: VaultRewards Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultRewards RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof Skipped: VaultRewards RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultRewards TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof Skipped: Fee Commission (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof Skipped: VaultStaking Nonce (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:0)
	/// Proof Skipped: VaultStaking TotalCurrentStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof Skipped: VaultStaking RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1 w:0)
	/// Proof Skipped: VaultRewards TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	fn accept_new_issues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3402`
		//  Estimated: `108764`
		// Minimum execution time: 178_178_000 picoseconds.
		Weight::from_parts(180_836_000, 108764)
			.saturating_add(T::DbWeight::get().reads(24_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:1)
	/// Proof Skipped: VaultCapacity Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultCapacity RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof Skipped: VaultCapacity RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultCapacity TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards Stake (r:1 w:1)
	/// Proof Skipped: VaultRewards Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultRewards RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof Skipped: VaultRewards RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultRewards TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof Skipped: Fee Commission (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof Skipped: VaultStaking Nonce (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:0)
	/// Proof Skipped: VaultStaking TotalCurrentStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof Skipped: VaultStaking RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards TotalStake (r:1 w:1)
	/// Proof Skipped: VaultRewards TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardCurrencies (r:1 w:0)
	/// Proof Skipped: VaultRewards RewardCurrencies (max_values: None, max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity TotalStake (r:1 w:1)
	/// Proof Skipped: VaultCapacity TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardCurrencies (r:1 w:0)
	/// Proof Skipped: VaultCapacity RewardCurrencies (max_values: None, max_size: None, mode: Measured)
	fn set_custom_secure_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3445`
		//  Estimated: `127169`
		// Minimum execution time: 195_740_000 picoseconds.
		Weight::from_parts(206_734_000, 127169)
			.saturating_add(T::DbWeight::get().reads(27_u64))
			.saturating_add(T::DbWeight::get().writes(15_u64))
	}
	/// Storage: VaultRegistry MinimumCollateralVault (r:0 w:1)
	/// Proof: VaultRegistry MinimumCollateralVault (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	fn set_minimum_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 8_159_000 picoseconds.
		Weight::from_parts(8_542_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry SystemCollateralCeiling (r:0 w:1)
	/// Proof: VaultRegistry SystemCollateralCeiling (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn set_system_collateral_ceiling() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 8_606_000 picoseconds.
		Weight::from_parts(8_845_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry SecureCollateralThreshold (r:0 w:1)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn set_secure_collateral_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 8_436_000 picoseconds.
		Weight::from_parts(8_768_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry PremiumRedeemThreshold (r:0 w:1)
	/// Proof: VaultRegistry PremiumRedeemThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn set_premium_redeem_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 8_633_000 picoseconds.
		Weight::from_parts(8_933_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry LiquidationCollateralThreshold (r:0 w:1)
	/// Proof: VaultRegistry LiquidationCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn set_liquidation_collateral_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `711`
		//  Estimated: `0`
		// Minimum execution time: 8_339_000 picoseconds.
		Weight::from_parts(8_691_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: VaultRegistry LiquidationCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry LiquidationCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof Skipped: VaultStaking Nonce (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	/// Proof Skipped: VaultStaking TotalCurrentStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking Stake (r:1 w:1)
	/// Proof Skipped: VaultStaking Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking SlashPerToken (r:1 w:0)
	/// Proof Skipped: VaultStaking SlashPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking SlashTally (r:1 w:1)
	/// Proof Skipped: VaultStaking SlashTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity Stake (r:1 w:0)
	/// Proof Skipped: VaultCapacity Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultCapacity RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof Skipped: VaultCapacity RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultCapacity TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards Stake (r:1 w:1)
	/// Proof Skipped: VaultRewards Stake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof Skipped: VaultRewards RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof Skipped: VaultRewards RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof Skipped: VaultRewards TotalRewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof Skipped: Fee Commission (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof Skipped: VaultStaking RewardPerToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking TotalStake (r:1 w:1)
	/// Proof Skipped: VaultStaking TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1 w:1)
	/// Proof Skipped: VaultRewards TotalStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultStaking RewardTally (r:2 w:2)
	/// Proof Skipped: VaultStaking RewardTally (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRewards RewardCurrencies (r:1 w:0)
	/// Proof Skipped: VaultRewards RewardCurrencies (max_values: None, max_size: None, mode: Measured)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	/// Proof: VaultRegistry SystemCollateralCeiling (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRegistry LiquidationVault (r:1 w:1)
	/// Proof: VaultRegistry LiquidationVault (max_values: None, max_size: Some(124), added: 2599, mode: MaxEncodedLen)
	fn report_undercollateralized_vault() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5171`
		//  Estimated: `204222`
		// Minimum execution time: 585_357_000 picoseconds.
		Weight::from_parts(609_933_000, 204222)
			.saturating_add(T::DbWeight::get().reads(39_u64))
			.saturating_add(T::DbWeight::get().writes(24_u64))
	}
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	fn recover_vault_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1438`
		//  Estimated: `2735`
		// Minimum execution time: 20_871_000 picoseconds.
		Weight::from_parts(22_098_000, 2735)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
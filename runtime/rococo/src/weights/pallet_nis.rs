// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_nis`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_nis
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nis`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nis::WeightInfo for WeightInfo<T> {
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6278 + l * (48 ±0)`
		//  Estimated: `60718`
		// Minimum execution time: 26_651 nanoseconds.
		Weight::from_ref_time(28_993_951)
			.saturating_add(Weight::from_proof_size(60718))
			// Standard Error: 675
			.saturating_add(Weight::from_ref_time(80_545).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54280`
		//  Estimated: `60718`
		// Minimum execution time: 103_774 nanoseconds.
		Weight::from_ref_time(105_111_000)
			.saturating_add(Weight::from_proof_size(60718))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6278 + l * (48 ±0)`
		//  Estimated: `60718`
		// Minimum execution time: 34_113 nanoseconds.
		Weight::from_ref_time(30_119_340)
			.saturating_add(Weight::from_proof_size(60718))
			// Standard Error: 672
			.saturating_add(Weight::from_ref_time(61_739).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Summary (r:1 w:0)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `3138`
		// Minimum execution time: 31_298 nanoseconds.
		Weight::from_ref_time(31_769_000)
			.saturating_add(Weight::from_proof_size(3138))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `9418`
		// Minimum execution time: 42_385 nanoseconds.
		Weight::from_ref_time(42_901_000)
			.saturating_add(Weight::from_proof_size(9418))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `698`
		//  Estimated: `8792`
		// Minimum execution time: 59_372 nanoseconds.
		Weight::from_ref_time(59_992_000)
			.saturating_add(Weight::from_proof_size(8792))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `759`
		//  Estimated: `12516`
		// Minimum execution time: 66_981 nanoseconds.
		Weight::from_ref_time(67_647_000)
			.saturating_add(Weight::from_proof_size(12516))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `538`
		//  Estimated: `12516`
		// Minimum execution time: 60_920 nanoseconds.
		Weight::from_ref_time(61_706_000)
			.saturating_add(Weight::from_proof_size(12516))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6689`
		//  Estimated: `9635`
		// Minimum execution time: 21_884 nanoseconds.
		Weight::from_ref_time(22_308_000)
			.saturating_add(Weight::from_proof_size(9635))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `50497`
		// Minimum execution time: 4_646 nanoseconds.
		Weight::from_ref_time(4_812_000)
			.saturating_add(Weight::from_proof_size(50497))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nis Receipts (r:0 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_183 nanoseconds.
		Weight::from_ref_time(6_420_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

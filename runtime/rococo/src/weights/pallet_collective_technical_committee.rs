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
//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
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

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalCommittee Members (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:100 w:100)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3233 ±0) + p * (3223 ±0)`
		//  Estimated: `16206 + m * (7809 ±16) + p * (10238 ±16)`
		// Minimum execution time: 17_490 nanoseconds.
		Weight::from_ref_time(17_812_000)
			.saturating_add(Weight::from_proof_size(16206))
			// Standard Error: 52_570
			.saturating_add(Weight::from_ref_time(5_970_628).saturating_mul(m.into()))
			// Standard Error: 52_570
			.saturating_add(Weight::from_ref_time(8_442_965).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_proof_size(7809).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(10238).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139 + m * (32 ±0)`
		//  Estimated: `635 + m * (32 ±0)`
		// Minimum execution time: 16_507 nanoseconds.
		Weight::from_ref_time(15_656_846)
			.saturating_add(Weight::from_proof_size(635))
			// Standard Error: 16
			.saturating_add(Weight::from_ref_time(1_848).saturating_mul(b.into()))
			// Standard Error: 171
			.saturating_add(Weight::from_ref_time(14_642).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139 + m * (32 ±0)`
		//  Estimated: `3250 + m * (64 ±0)`
		// Minimum execution time: 18_348 nanoseconds.
		Weight::from_ref_time(17_412_463)
			.saturating_add(Weight::from_proof_size(3250))
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(1_783).saturating_mul(b.into()))
			// Standard Error: 202
			.saturating_add(Weight::from_ref_time(23_933).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_proof_size(64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `461 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `5880 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 24_209 nanoseconds.
		Weight::from_ref_time(24_797_325)
			.saturating_add(Weight::from_proof_size(5880))
			// Standard Error: 47
			.saturating_add(Weight::from_ref_time(2_637).saturating_mul(b.into()))
			// Standard Error: 499
			.saturating_add(Weight::from_ref_time(18_477).saturating_mul(m.into()))
			// Standard Error: 493
			.saturating_add(Weight::from_ref_time(98_937).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(165).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `911 + m * (64 ±0)`
		//  Estimated: `4790 + m * (128 ±0)`
		// Minimum execution time: 21_974 nanoseconds.
		Weight::from_ref_time(22_456_460)
			.saturating_add(Weight::from_proof_size(4790))
			// Standard Error: 611
			.saturating_add(Weight::from_ref_time(51_856).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(128).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `531 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `5513 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 26_446 nanoseconds.
		Weight::from_ref_time(28_061_456)
			.saturating_add(Weight::from_proof_size(5513))
			// Standard Error: 549
			.saturating_add(Weight::from_ref_time(17_104).saturating_mul(m.into()))
			// Standard Error: 535
			.saturating_add(Weight::from_ref_time(89_314).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(260).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `867 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `8784 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 36_960 nanoseconds.
		Weight::from_ref_time(38_532_650)
			.saturating_add(Weight::from_proof_size(8784))
			// Standard Error: 82
			.saturating_add(Weight::from_ref_time(2_171).saturating_mul(b.into()))
			// Standard Error: 876
			.saturating_add(Weight::from_ref_time(21_279).saturating_mul(m.into()))
			// Standard Error: 854
			.saturating_add(Weight::from_ref_time(120_364).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(4).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(264).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(160).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `551 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `6620 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 29_154 nanoseconds.
		Weight::from_ref_time(29_815_989)
			.saturating_add(Weight::from_proof_size(6620))
			// Standard Error: 394
			.saturating_add(Weight::from_ref_time(25_714).saturating_mul(m.into()))
			// Standard Error: 384
			.saturating_add(Weight::from_ref_time(91_672).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(325).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `887 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `10090 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 39_673 nanoseconds.
		Weight::from_ref_time(41_299_924)
			.saturating_add(Weight::from_proof_size(10090))
			// Standard Error: 82
			.saturating_add(Weight::from_ref_time(1_965).saturating_mul(b.into()))
			// Standard Error: 875
			.saturating_add(Weight::from_ref_time(21_924).saturating_mul(m.into()))
			// Standard Error: 853
			.saturating_add(Weight::from_ref_time(121_244).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(5).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(330).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(200).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + p * (32 ±0)`
		//  Estimated: `1383 + p * (96 ±0)`
		// Minimum execution time: 15_065 nanoseconds.
		Weight::from_ref_time(17_118_339)
			.saturating_add(Weight::from_proof_size(1383))
			// Standard Error: 471
			.saturating_add(Weight::from_ref_time(86_165).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(96).saturating_mul(p.into()))
	}
}

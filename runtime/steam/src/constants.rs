// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// Modifications Copyright (c) 2021 John Whitton
// 2021-03 : Customize for EAVE Protocol

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A set of constant values used in dev runtime.

/// Time and blocks.
pub mod time {
	use acala_primitives::{Balance, BlockNumber, Moment};
	use eave_runtime_common::{dollar, millicent, EAVE};

	pub const SECS_PER_BLOCK: Moment = 6;
	pub const MILLISECS_PER_BLOCK: Moment = SECS_PER_BLOCK * 1000;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	pub fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 2 * dollar(EAVE) + (bytes as Balance) * 10 * millicent(EAVE)
	}
}

/// Fee-related
pub mod fee {
	use frame_support::weights::{
		constants::{ExtrinsicBaseWeight, WEIGHT_PER_SECOND},
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use acala_primitives::Balance;
	use eave_runtime_common::{cent, EAVE};
	use smallvec::smallvec;
	use sp_runtime::Perbill;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	fn base_tx_in_eave() -> Balance {
		cent(EAVE) / 10
	}

	/// Handles converting a weight scalar to a fee value, based on the scale
	/// and granularity of the node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, system::MaximumBlockWeight]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some
	/// examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Eave, extrinsic base weight (smallest non-zero weight) is mapped to 1/10
			// CENT:
			let p = base_tx_in_eave(); // 10_000_000_000;
			let q = Balance::from(ExtrinsicBaseWeight::get()); // 125_000_000
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q), // zero
				coeff_integer: p / q,                         // 80
			}]
		}
	}

	pub fn dot_per_second() -> u128 {
		let base_weight = Balance::from(ExtrinsicBaseWeight::get());
		let base_tx_per_second = (WEIGHT_PER_SECOND as u128) / base_weight;
		let eave_per_second = base_tx_per_second * base_tx_in_eave();
		eave_per_second / 100
	}
}
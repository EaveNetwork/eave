// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// Modifications Copyright (c) 2021 John Whitton
// 2021-03 : Customize for EAVE Protocol

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

//! The precompiles for EVM, includes standard Ethereum precompiles, and more:
//! - MultiCurrency at address `H160::from_low_u64_be(1024)`.

#![allow(clippy::upper_case_acronyms)]

mod mock;
mod tests;

use crate::is_eave_precompile;
use frame_support::log;
use module_evm::{
	precompiles::{
		ECRecover, ECRecoverPublicKey, EvmPrecompiles, Identity, Precompile, Precompiles, Ripemd160, Sha256,
		Sha3FIPS256, Sha3FIPS512,
	},
	Context, ExitError, ExitSucceed,
};
use module_support::PrecompileCallerFilter as PrecompileCallerFilterT;
use acala_primitives::PRECOMPILE_ADDRESS_START;
use sp_core::H160;
use sp_std::{marker::PhantomData, prelude::*};

pub mod dex;
pub mod input;
pub mod multicurrency;
pub mod nft;
pub mod oracle;
pub mod schedule_call;
pub mod state_rent;

pub use dex::DexPrecompile;
pub use multicurrency::MultiCurrencyPrecompile;
pub use nft::NFTPrecompile;
pub use oracle::OraclePrecompile;
pub use schedule_call::ScheduleCallPrecompile;
pub use state_rent::StateRentPrecompile;

pub struct AllPrecompiles<
	PrecompileCallerFilter,
	MultiCurrencyPrecompile,
	NFTPrecompile,
	StateRentPrecompile,
	OraclePrecompile,
	ScheduleCallPrecompile,
	DexPrecompile,
>(
	PhantomData<(
		PrecompileCallerFilter,
		MultiCurrencyPrecompile,
		NFTPrecompile,
		StateRentPrecompile,
		OraclePrecompile,
		ScheduleCallPrecompile,
		DexPrecompile,
	)>,
);

impl<
		PrecompileCallerFilter,
		MultiCurrencyPrecompile,
		NFTPrecompile,
		StateRentPrecompile,
		OraclePrecompile,
		ScheduleCallPrecompile,
		DexPrecompile,
	> Precompiles
	for AllPrecompiles<
		PrecompileCallerFilter,
		MultiCurrencyPrecompile,
		NFTPrecompile,
		StateRentPrecompile,
		OraclePrecompile,
		ScheduleCallPrecompile,
		DexPrecompile,
	> where
	MultiCurrencyPrecompile: Precompile,
	NFTPrecompile: Precompile,
	StateRentPrecompile: Precompile,
	OraclePrecompile: Precompile,
	ScheduleCallPrecompile: Precompile,
	PrecompileCallerFilter: PrecompileCallerFilterT,
	DexPrecompile: Precompile,
{
	#[allow(clippy::type_complexity)]
	fn execute(
		address: H160,
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
	) -> Option<core::result::Result<(ExitSucceed, Vec<u8>, u64), ExitError>> {
		EvmPrecompiles::<ECRecover, Sha256, Ripemd160, Identity, ECRecoverPublicKey, Sha3FIPS256, Sha3FIPS512>::execute(
			address, input, target_gas, context,
		)
		.or_else(|| {
			if is_eave_precompile(address) && !PrecompileCallerFilter::is_allowed(context.caller) {
				log::debug!(target: "evm", "Precompile no permission");
				return Some(Err(ExitError::Other("no permission".into())));
			}

			if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START) {
				Some(MultiCurrencyPrecompile::execute(input, target_gas, context))
			} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 1) {
				Some(NFTPrecompile::execute(input, target_gas, context))
			} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 2) {
				Some(StateRentPrecompile::execute(input, target_gas, context))
			} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 3) {
				Some(OraclePrecompile::execute(input, target_gas, context))
			} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 4) {
				Some(ScheduleCallPrecompile::execute(input, target_gas, context))
			} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 5) {
				Some(DexPrecompile::execute(input, target_gas, context))
			} else {
				None
			}
		})
	}
}
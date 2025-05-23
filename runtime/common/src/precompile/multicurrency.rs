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

use frame_support::log;
use module_evm::{Context, ExitError, ExitSucceed, Precompile};
use module_support::{AddressMapping as AddressMappingT, CurrencyIdMapping as CurrencyIdMappingT};
use sp_core::U256;
use sp_std::{fmt::Debug, marker::PhantomData, prelude::*, result};

use orml_traits::MultiCurrency as MultiCurrencyT;

use super::input::{Input, InputT};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use acala_primitives::{Balance, CurrencyId};

/// The `MultiCurrency` impl precompile.
///
///
/// `input` data starts with `action` and `currency_id`.
///
/// Actions:
/// - Query total issuance.
/// - Query balance. Rest `input` bytes: `account_id`.
/// - Transfer. Rest `input` bytes: `from`, `to`, `amount`.
pub struct MultiCurrencyPrecompile<AccountId, AddressMapping, CurrencyIdMapping, MultiCurrency>(
	PhantomData<(AccountId, AddressMapping, CurrencyIdMapping, MultiCurrency)>,
);

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Action {
	QueryName = 0x06fdde03,
	QuerySymbol = 0x95d89b41,
	QueryDecimals = 0x313ce567,
	QueryTotalIssuance = 0x18160ddd,
	QueryBalance = 0x70a08231,
	Transfer = 0xbeabacc8,
}

impl<AccountId, AddressMapping, CurrencyIdMapping, MultiCurrency> Precompile
	for MultiCurrencyPrecompile<AccountId, AddressMapping, CurrencyIdMapping, MultiCurrency>
where
	AccountId: Debug + Clone,
	AddressMapping: AddressMappingT<AccountId>,
	CurrencyIdMapping: CurrencyIdMappingT,
	MultiCurrency: MultiCurrencyT<AccountId, Balance = Balance, CurrencyId = CurrencyId>,
{
	fn execute(
		input: &[u8],
		_target_gas: Option<u64>,
		context: &Context,
	) -> result::Result<(ExitSucceed, Vec<u8>, u64), ExitError> {
		//TODO: evaluate cost

		log::debug!(target: "evm", "multicurrency: input: {:?}", input);

		let input = Input::<Action, AccountId, AddressMapping, CurrencyIdMapping>::new(input);

		let action = input.action()?;
		let currency_id = CurrencyIdMapping::decode_evm_address(context.caller)
			.ok_or_else(|| ExitError::Other("invalid currency id".into()))?;

		log::debug!(target: "evm", "multicurrency: currency id: {:?}", currency_id);

		match action {
			Action::QueryName => {
				let name =
					CurrencyIdMapping::name(currency_id).ok_or_else(|| ExitError::Other("Get name failed".into()))?;
				log::debug!(target: "evm", "multicurrency: name: {:?}", name);

				Ok((ExitSucceed::Returned, vec_u8_from_str(&name), 0))
			}
			Action::QuerySymbol => {
				let symbol = CurrencyIdMapping::symbol(currency_id)
					.ok_or_else(|| ExitError::Other("Get symbol failed".into()))?;
				log::debug!(target: "evm", "multicurrency: symbol: {:?}", symbol);

				Ok((ExitSucceed::Returned, vec_u8_from_str(&symbol), 0))
			}
			Action::QueryDecimals => {
				let decimals = CurrencyIdMapping::decimals(currency_id)
					.ok_or_else(|| ExitError::Other("Get decimals failed".into()))?;
				log::debug!(target: "evm", "multicurrency: decimals: {:?}", decimals);

				Ok((ExitSucceed::Returned, vec_u8_from_u8(decimals), 0))
			}
			Action::QueryTotalIssuance => {
				let total_issuance = vec_u8_from_balance(MultiCurrency::total_issuance(currency_id));
				log::debug!(target: "evm", "multicurrency: total issuance: {:?}", total_issuance);

				Ok((ExitSucceed::Returned, total_issuance, 0))
			}
			Action::QueryBalance => {
				let who = input.account_id_at(1)?;
				log::debug!(target: "evm", "multicurrency: who: {:?}", who);

				let balance = vec_u8_from_balance(MultiCurrency::total_balance(currency_id, &who));
				log::debug!(target: "evm", "multicurrency: balance: {:?}", balance);

				Ok((ExitSucceed::Returned, balance, 0))
			}
			Action::Transfer => {
				let from = input.account_id_at(1)?;
				let to = input.account_id_at(2)?;
				let amount = input.balance_at(3)?;

				log::debug!(target: "evm", "multicurrency: from: {:?}", from);
				log::debug!(target: "evm", "multicurrency: to: {:?}", to);
				log::debug!(target: "evm", "multicurrency: amount: {:?}", amount);

				MultiCurrency::transfer(currency_id, &from, &to, amount).map_err(|e| {
					let err_msg: &str = e.into();
					ExitError::Other(err_msg.into())
				})?;

				log::debug!(target: "evm", "multicurrency: transfer success!");

				Ok((ExitSucceed::Returned, vec![], 0))
			}
		}
	}
}

fn vec_u8_from_balance(balance: Balance) -> Vec<u8> {
	let mut be_bytes = [0u8; 32];
	U256::from(balance).to_big_endian(&mut be_bytes[..]);
	be_bytes.to_vec()
}

fn vec_u8_from_u8(b: u8) -> Vec<u8> {
	let mut be_bytes = [0u8; 32];
	U256::from(b).to_big_endian(&mut be_bytes[..]);
	be_bytes.to_vec()
}

fn vec_u8_from_str(b: &[u8]) -> Vec<u8> {
	let mut be_bytes = [0u8; 32];
	U256::from_big_endian(b).to_big_endian(&mut be_bytes[..]);
	be_bytes.to_vec()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::precompile::mock::get_function_selector;

	#[test]
	fn function_selector_match() {
		assert_eq!(
			u32::from_be_bytes(get_function_selector("name()")),
			Into::<u32>::into(Action::QueryName)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("symbol()")),
			Into::<u32>::into(Action::QuerySymbol)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("decimals()")),
			Into::<u32>::into(Action::QueryDecimals)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("totalSupply()")),
			Into::<u32>::into(Action::QueryTotalIssuance)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("balanceOf(address)")),
			Into::<u32>::into(Action::QueryBalance)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("transfer(address,address,uint256)")),
			Into::<u32>::into(Action::Transfer)
		);
	}
}
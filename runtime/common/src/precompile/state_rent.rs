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
use num_enum::{IntoPrimitive, TryFromPrimitive};
use sp_core::U256;
use sp_std::{borrow::Cow, fmt::Debug, marker::PhantomData, prelude::*, result};

use module_support::{AddressMapping as AddressMappingT, CurrencyIdMapping as CurrencyIdMappingT, EVMStateRentTrait};

use super::input::{Input, InputT};
use acala_primitives::Balance;

/// The `EVM` impl precompile.
///
/// `input` data starts with `action`.
///
/// Actions:
/// - QueryNewContractExtraBytes.
/// - QueryStorageDepositPerByte.
/// - QueryMaintainer.
/// - QueryDeveloperDeposit.
/// - QueryDeploymentFee.
/// - TransferMaintainer. Rest `input` bytes: `from`, `contract`, `new_maintainer`.
pub struct StateRentPrecompile<AccountId, AddressMapping, CurrencyIdMapping, EVM>(
	PhantomData<(AccountId, AddressMapping, CurrencyIdMapping, EVM)>,
);

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Action {
	QueryNewContractExtraBytes = 0xa23e8b82,
	QueryStorageDepositPerByte = 0x6e043998,
	QueryMaintainer = 0x06ad1355,
	QueryDeveloperDeposit = 0x68a18855,
	QueryDeploymentFee = 0xf2cff57f,
	TransferMaintainer = 0xee0d2e12,
}

impl<AccountId, AddressMapping, CurrencyIdMapping, EVM> Precompile
	for StateRentPrecompile<AccountId, AddressMapping, CurrencyIdMapping, EVM>
where
	AccountId: Clone + Debug,
	AddressMapping: AddressMappingT<AccountId>,
	CurrencyIdMapping: CurrencyIdMappingT,
	EVM: EVMStateRentTrait<AccountId, Balance>,
{
	fn execute(
		input: &[u8],
		_target_gas: Option<u64>,
		_context: &Context,
	) -> result::Result<(ExitSucceed, Vec<u8>, u64), ExitError> {
		log::debug!(target: "evm", "state_rent input: {:?}", input);
		let input = Input::<Action, AccountId, AddressMapping, CurrencyIdMapping>::new(input);

		let action = input.action()?;

		match action {
			Action::QueryNewContractExtraBytes => {
				let bytes = vec_u8_from_u32(EVM::query_new_contract_extra_bytes());
				Ok((ExitSucceed::Returned, bytes, 0))
			}
			Action::QueryStorageDepositPerByte => {
				let deposit = vec_u8_from_balance(EVM::query_storage_deposit_per_byte());
				Ok((ExitSucceed::Returned, deposit, 0))
			}
			Action::QueryMaintainer => {
				let contract = input.evm_address_at(1)?;

				let maintainer =
					EVM::query_maintainer(contract).map_err(|e| ExitError::Other(Cow::Borrowed(e.into())))?;

				let mut address = [0u8; 32];
				address[12..].copy_from_slice(&maintainer.as_bytes().to_vec());

				Ok((ExitSucceed::Returned, address.to_vec(), 0))
			}
			Action::QueryDeveloperDeposit => {
				let deposit = vec_u8_from_balance(EVM::query_developer_deposit());
				Ok((ExitSucceed::Returned, deposit, 0))
			}
			Action::QueryDeploymentFee => {
				let fee = vec_u8_from_balance(EVM::query_deployment_fee());
				Ok((ExitSucceed::Returned, fee, 0))
			}
			Action::TransferMaintainer => {
				let from = input.account_id_at(1)?;
				let contract = input.evm_address_at(2)?;
				let new_maintainer = input.evm_address_at(3)?;

				log::debug!(
					target: "evm",
					"state_rent: from: {:?}, contract: {:?}, new_maintainer: {:?}",
					from, contract, new_maintainer,
				);

				EVM::transfer_maintainer(from, contract, new_maintainer)
					.map_err(|e| ExitError::Other(Cow::Borrowed(e.into())))?;

				Ok((ExitSucceed::Returned, vec![], 0))
			}
		}
	}
}

fn vec_u8_from_balance(b: Balance) -> Vec<u8> {
	let mut be_bytes = [0u8; 32];
	U256::from(b).to_big_endian(&mut be_bytes[..]);
	be_bytes.to_vec()
}

fn vec_u8_from_u32(b: u32) -> Vec<u8> {
	let mut be_bytes = [0u8; 32];
	U256::from(b).to_big_endian(&mut be_bytes[..]);
	be_bytes.to_vec()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::precompile::mock::get_function_selector;

	#[test]
	fn function_selector_match() {
		assert_eq!(
			u32::from_be_bytes(get_function_selector("newContractExtraBytes()")),
			Into::<u32>::into(Action::QueryNewContractExtraBytes)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("storageDepositPerByte()")),
			Into::<u32>::into(Action::QueryStorageDepositPerByte)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("maintainerOf(address)")),
			Into::<u32>::into(Action::QueryMaintainer)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("developerDeposit()")),
			Into::<u32>::into(Action::QueryDeveloperDeposit)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("deploymentFee()")),
			Into::<u32>::into(Action::QueryDeploymentFee)
		);

		assert_eq!(
			u32::from_be_bytes(get_function_selector("transferMaintainer(address,address,address)")),
			Into::<u32>::into(Action::TransferMaintainer)
		);
	}
}

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

/// Local relay chain config (multivalidator Alice + Bob)
// TODO currently steam-local connects to Rococo and this is just for a local parachain
// TODO need to work out whether we need a two validator local test chain
// TODO if so then are steam and noria the same all generated here or do we create
// TODO a new chainspec specifically for noria
// TODO we will also need to create a chainspec for ICE.

use acala_primitives::{AccountId, TokenSymbol};
use hex_literal::hex;
use sc_chain_spec::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde_json::map::Map;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{traits::Zero, FixedPointNumber, FixedU128};

use crate::chain_spec::{
	evm_genesis, get_account_id_from_seed, get_authority_keys_from_seed, Extensions, TELEMETRY_URL,
};

use eave_runtime_common::TokenInfo;

pub type ChainSpec = sc_service::GenericChainSpec<steam_runtime::GenesisConfig, Extensions>;

pub const PARA_ID: u32 = 2000;

/// Used for Steam Local
// TODO remove or replace with testnet config
pub fn steam_local_config() -> Result<ChainSpec, String> {
	let mut properties = Map::new();
	let mut token_symbol: Vec<String> = vec![];
	let mut token_decimals: Vec<u32> = vec![];
	TokenSymbol::get_info().iter().for_each(|(symbol_name, decimals)| {
		token_symbol.push(symbol_name.to_string());
		token_decimals.push(*decimals);
	});
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), token_decimals.into());

	let wasm_binary = steam_runtime::WASM_BINARY.ok_or("Steam runtime wasm binary not available")?;

	Ok(ChainSpec::from_genesis(
		"Steam Local PC",
		"steam-local-pc",
		ChainType::Local,
		move || {
			steam_genesis(
				wasm_binary,
				// Initial PoA authorities sr25519,sr25519,GrandpaId (ed25519 AccountId), BabeId (sr25519 AccountId)
				vec![
					(
						// 5HpztiEG3Wj2fjZZZwBxPcM8KSN9GL2F9aT7YJPm7LLJnMXx - Alioth Validator
						hex!["0ef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55"].into(),
						hex!["fef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55"].into(),
						// 0x6ad519e3c57e44421df2b39849a9927046bf7be1d801f3e36f7ac50fca569c4a - Alioth ed25519 AccountId
						hex!["6ad519e3c57e44421df2b39849a9927046bf7be1d801f3e36f7ac50fca569c4a"].unchecked_into(),
						// 0xfef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55 - Alioth sr25519 AccountId
						hex!["fef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55"].unchecked_into(),
					),
				],
				// Sudo Acccount
				// 5Do24fmH4Md2VuUTVkQfjMg3Zw2AJHzEVTKWsJQcXGe2uVv3 - EAVE Test Account 1
				hex!["4c81d490f0298aedc49279cbeb4ce4f44b5f18dccabb15ea6466976f188fb928"].into(),
				// Endowed Accounts
				vec![
					// 5FZYvTuhSNTjzsQBwnEFZGUegdoofmhpuSCigTy9ruUdb5kv - EAVE Foundation
					hex!["9ab4bd9ec0c0a40fad32077e19ac3a5f6120da0214f02d79f79aefb96a55d74f"].into(),
					// 5Gea9QJbhAKWkZ17S6TmbwSEyvaiaftT5xrNMwXzqzN7k3XP - EAVE Test Account 1
					hex!["4c81d490f0298aedc49279cbeb4ce4f44b5f18dccabb15ea6466976f188fb928"].into(),
				],
			)
		},
		vec![],
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
		},
	))
}

/// Used for Steam Rococo (build stable from here)
// This is similar to  mandala.rs latest_mandala_testnet_config
pub fn steam_latest_config() -> Result<ChainSpec, String> {
	let mut properties = Map::new();
	let mut token_symbol: Vec<String> = vec![];
	let mut token_decimals: Vec<u32> = vec![];
	TokenSymbol::get_info().iter().for_each(|(symbol_name, decimals)| {
		token_symbol.push(symbol_name.to_string());
		token_decimals.push(*decimals);
	});
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), token_decimals.into());

	let wasm_binary = steam_runtime::WASM_BINARY.ok_or("Steam runtime wasm binary not available")?;

	Ok(ChainSpec::from_genesis(
		"Steam PC",
		"steam-pc",
		ChainType::Development,
		// SECRET="..."
		// ./target/debug/subkey inspect "$SECRET//eave//root"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//oracle"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//1//validator"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//1//babe"
		// ./target/debug/subkey --ed25519 inspect "$SECRET//eave//1//grandpa"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//2//validator"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//2//babe"
		// ./target/debug/subkey --ed25519 inspect "$SECRET//eave//2//grandpa"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//3//validator"
		// ./target/debug/subkey --sr25519 inspect "$SECRET//eave//3//babe"
		// ./target/debug/subkey --ed25519 inspect "$SECRET//eave//3//grandpa"
		move || {
			steam_genesis(
				wasm_binary,
				// Initial PoA authorities sr25519,sr25519,GrandpaId (ed25519 AccountId), BabeId (sr25519 AccountId)
				vec![
					(
						// 5HpztiEG3Wj2fjZZZwBxPcM8KSN9GL2F9aT7YJPm7LLJnMXx - Alioth Validator
						hex!["0ef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55"].into(),
						hex!["fef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55"].into(),
						// 0x6ad519e3c57e44421df2b39849a9927046bf7be1d801f3e36f7ac50fca569c4a - Alioth ed25519 AccountId
						hex!["6ad519e3c57e44421df2b39849a9927046bf7be1d801f3e36f7ac50fca569c4a"].unchecked_into(),
						// 0xfef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55 - Alioth sr25519 AccountId
						hex!["fef54f8bd61a2f17626e8cb4a402821de3e5d49235415526b4a3770e9d6c2a55"].unchecked_into(),
					),
				],
				// Sudo Account
				// 5Do24fmH4Md2VuUTVkQfjMg3Zw2AJHzEVTKWsJQcXGe2uVv3 - EAVE Test Account 1
				hex!["4c81d490f0298aedc49279cbeb4ce4f44b5f18dccabb15ea6466976f188fb928"].into(),
				//Endowed Accounts
				vec![
					// 5FZYvTuhSNTjzsQBwnEFZGUegdoofmhpuSCigTy9ruUdb5kv - EAVE Foundation
					hex!["9ab4bd9ec0c0a40fad32077e19ac3a5f6120da0214f02d79f79aefb96a55d74f"].into(),
					// 5Gea9QJbhAKWkZ17S6TmbwSEyvaiaftT5xrNMwXzqzN7k3XP - EAVE Test Account 1
					hex!["4c81d490f0298aedc49279cbeb4ce4f44b5f18dccabb15ea6466976f188fb928"].into(),
				],
			)
		},
		vec![
			"/ip4/54.254.37.221/tcp/30333/p2p/12D3KooWNUPp9ervpypz95DCMHfb3CAbQdfrmmBbYehUaJsFvRvT"
				.parse()
				.unwrap(),
		],
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		Some("steam-pc"),
		Some(properties),
		Extensions {
			relay_chain: "rococo".into(),
			para_id: PARA_ID,
		},
	))
}

/// Used for Steam - Sourced from steam-stable.json file for Rococo 
pub fn steam_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../../../resources/steam-stable.json")[..])
}

/// Development testnet config (single validator Alice)
pub fn development_testnet_config() -> Result<ChainSpec, String> {
	let mut properties = Map::new();
	let mut token_symbol: Vec<String> = vec![];
	let mut token_decimals: Vec<u32> = vec![];
	TokenSymbol::get_info().iter().for_each(|(symbol_name, decimals)| {
		token_symbol.push(symbol_name.to_string());
		token_decimals.push(*decimals);
	});
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), token_decimals.into());

	let wasm_binary = steam_runtime::WASM_BINARY.unwrap_or_default();

	Ok(ChainSpec::from_genesis(
		"Beast Developer",
		"beast-dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![get_authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		vec![],
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo".into(),
			para_id: PARA_ID,
		},
	))
}

/// Local relay chain config (multivalidator Alice + Bob)
// TODO currently steam-local connects to Rococo and this is just for a local parachain
// TODO need to work out whether we need a two validator local test chain
// TODO if so then are steam and noria the same all generated here or do we create
// TODO a new chainspec specifically for noria
// TODO we will also need to create a chainspec for ICE.
pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let mut properties = Map::new();
	let mut token_symbol: Vec<String> = vec![];
	let mut token_decimals: Vec<u32> = vec![];
	TokenSymbol::get_info().iter().for_each(|(symbol_name, decimals)| {
		token_symbol.push(symbol_name.to_string());
		token_decimals.push(*decimals);
	});
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), token_decimals.into());

	let wasm_binary = steam_runtime::WASM_BINARY.ok_or("Dev runtime wasm binary not available")?;

	Ok(ChainSpec::from_genesis(
		"Aqua Local",
		"aqua-local",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				vec![
					get_authority_keys_from_seed("Alice"),
					get_authority_keys_from_seed("Bob"),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
			)
		},
		vec![],
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo".into(),
			para_id: PARA_ID,
		},
	))
}

/// Sourced from json file for Rococo - will be used for Standalone Testnet Beast
pub fn steam_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../../../resources/steam-stable.json")[..])
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> steam_runtime::GenesisConfig {
	use steam_runtime::{
		dollar, get_all_module_accounts, AirDropConfig, Balance, BalancesConfig, CdpEngineConfig, CdpTreasuryConfig,
		CollatorSelectionConfig, DexConfig, EVMConfig, EnabledTradingPairs, GeneralCouncilMembershipConfig,
		HomaCouncilMembershipConfig, HonzonCouncilMembershipConfig, IndicesConfig, NativeTokenExistentialDeposit,
		OperatorMembershipEaveConfig, OperatorMembershipBandConfig, OrmlNFTConfig, ParachainInfoConfig,
		RenVmBridgeConfig, SessionConfig, SessionKeys, StakingPoolConfig, SudoConfig, SystemConfig,
		TechnicalCommitteeMembershipConfig, TokensConfig, TradingPair, VestingConfig, EAVE, EUSD, DOT, LDOT, RENBTC, 
	};
	#[cfg(feature = "std")]
	use sp_std::collections::btree_map::BTreeMap;

	let existential_deposit = NativeTokenExistentialDeposit::get();

	let initial_balance: u128 = 1_000_000 * dollar(EAVE);
	let initial_staking: u128 = 100_000 * dollar(EAVE);

	let evm_genesis_accounts = evm_genesis();

	let balances = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), initial_staking + dollar(EAVE))) // bit more for fee
		.chain(endowed_accounts.iter().cloned().map(|k| (k, initial_balance)))
		.chain(
			get_all_module_accounts()
				.iter()
				.map(|x| (x.clone(), existential_deposit)),
		)
		.fold(
			BTreeMap::<AccountId, Balance>::new(),
			|mut acc, (account_id, amount)| {
				if let Some(balance) = acc.get_mut(&account_id) {
					*balance = balance
						.checked_add(amount)
						.expect("balance cannot overflow when building genesis");
				} else {
					acc.insert(account_id.clone(), amount);
				}
				acc
			},
		)
		.into_iter()
		.collect::<Vec<(AccountId, Balance)>>();

	steam_runtime::GenesisConfig {
		frame_system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_indices: IndicesConfig { indices: vec![] },
		pallet_balances: BalancesConfig { balances },
		pallet_sudo: SudoConfig { key: root_key.clone() },
		pallet_collective_Instance1: Default::default(),
		pallet_membership_Instance1: GeneralCouncilMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_collective_Instance2: Default::default(),
		pallet_membership_Instance2: HonzonCouncilMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_collective_Instance3: Default::default(),
		pallet_membership_Instance3: HomaCouncilMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_collective_Instance4: Default::default(),
		pallet_membership_Instance4: TechnicalCommitteeMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_membership_Instance5: OperatorMembershipEaveConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_membership_Instance6: OperatorMembershipBandConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_treasury: Default::default(),
		orml_tokens: TokensConfig {
			endowed_accounts: endowed_accounts.clone()
				.iter()
				.flat_map(|x| vec![(x.clone(), DOT, initial_balance), (x.clone(), EUSD, initial_balance)])
				.collect(),
		},
		orml_vesting: VestingConfig { vesting: vec![] },
		module_cdp_treasury: CdpTreasuryConfig {
			expected_collateral_auction_size: vec![
				(DOT, dollar(DOT)), // (currency_id, max size of a collateral auction)
				(RENBTC, dollar(RENBTC)),
			],
		},
		module_cdp_engine: CdpEngineConfig {
			collaterals_params: vec![
				(
					DOT,
					Some(FixedU128::zero()),                             // stability fee for this collateral
					Some(FixedU128::saturating_from_rational(150, 100)), // liquidation ratio
					Some(FixedU128::saturating_from_rational(10, 100)),  // liquidation penalty rate
					Some(FixedU128::saturating_from_rational(150, 100)), // required liquidation ratio
					10_000_000 * dollar(EUSD),                           // maximum debit value in eUSD (cap)
				),
				(
					LDOT,
					Some(FixedU128::zero()),
					Some(FixedU128::saturating_from_rational(150, 100)),
					Some(FixedU128::saturating_from_rational(10, 100)),
					Some(FixedU128::saturating_from_rational(180, 100)),
					10_000_000 * dollar(EUSD),
				),
				(
					RENBTC,
					Some(FixedU128::zero()),
					Some(FixedU128::saturating_from_rational(150, 100)),
					Some(FixedU128::saturating_from_rational(10, 100)),
					Some(FixedU128::saturating_from_rational(150, 100)),
					10_000_000 * dollar(EUSD),
				),
			],
			global_interest_rate_per_sec: FixedU128::saturating_from_rational(
				1_547_126_000u128,
				1_000_000_000_000_000_000u128,
			), /* 5% APR */
		},
		module_airdrop: AirDropConfig {
			airdrop_accounts: vec![],
		},
		module_evm: EVMConfig {
			accounts: evm_genesis_accounts,
			treasury: root_key,
		},
		module_staking_pool: StakingPoolConfig {
			staking_pool_params: module_staking_pool::Params {
				target_max_free_unbonded_ratio: FixedU128::saturating_from_rational(10, 100),
				target_min_free_unbonded_ratio: FixedU128::saturating_from_rational(5, 100),
				target_unbonding_to_free_ratio: FixedU128::saturating_from_rational(2, 100),
				unbonding_to_free_adjustment: FixedU128::saturating_from_rational(1, 1000),
				base_fee_rate: FixedU128::saturating_from_rational(2, 100),
			},
		},
		module_dex: DexConfig {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: EnabledTradingPairs::get(),
			initial_added_liquidity_pools: vec![(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(TradingPair::new(EUSD, DOT), (1_000_000u128, 2_000_000u128)),
					(TradingPair::new(EUSD, EAVE), (1_000_000u128, 2_000_000u128)),
				],
			)],
		},
		parachain_info: ParachainInfoConfig {
			parachain_id: PARA_ID.into(),
		},
		ecosystem_renvm_bridge: RenVmBridgeConfig {
			ren_vm_public_key: hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
		},
		orml_nft: OrmlNFTConfig { tokens: vec![] },
		module_collator_selection: CollatorSelectionConfig {
			invulnerables: initial_authorities.iter().cloned().map(|(acc, _, _, _)| acc).collect(),
			candidacy_bond: initial_staking,
			..Default::default()
		},
		pallet_session: SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|(acc, _, _, aura)| {
					(
						acc.clone(),          // account id
						acc,                  // validator id
						SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		pallet_aura: Default::default(),
	}
}

fn steam_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> steam_runtime::GenesisConfig {
	use steam_runtime::{
		cent, dollar, get_all_module_accounts, AirDropConfig, AirDropCurrencyId, Balance, BalancesConfig,
		CdpEngineConfig, CdpTreasuryConfig, CollatorSelectionConfig, DexConfig, EVMConfig, EnabledTradingPairs,
		GeneralCouncilMembershipConfig, HomaCouncilMembershipConfig, HonzonCouncilMembershipConfig, IndicesConfig,
		NativeTokenExistentialDeposit, OperatorMembershipEaveConfig, OperatorMembershipBandConfig, OrmlNFTConfig,
		ParachainInfoConfig, RenVmBridgeConfig, SessionConfig, SessionKeys, StakingPoolConfig, SudoConfig,
		SystemConfig, TechnicalCommitteeMembershipConfig, TokensConfig, UnreleasedNativeVaultAccountId, VestingConfig,
		EAVE, EUSD, DOT, LDOT, RENBTC, 
	};
	#[cfg(feature = "std")]
	use sp_std::collections::btree_map::BTreeMap;

	let existential_deposit = NativeTokenExistentialDeposit::get();

	let initial_balance: u128 = 1_000_000 * dollar(EAVE);
	let initial_staking: u128 = 100_000 * dollar(EAVE);

	let evm_genesis_accounts = evm_genesis();

	let mut unreleased_native = 1_000_000_000 * dollar(EAVE); // 1 billion
	let balances = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), initial_staking + dollar(EAVE))) // bit more for fee
		.chain(endowed_accounts.iter().cloned().map(|k| (k, initial_balance)))
		.chain(
			get_all_module_accounts()
				.iter()
				.map(|x| (x.clone(), existential_deposit)),
		)
		.fold(
			BTreeMap::<AccountId, Balance>::new(),
			|mut acc, (account_id, amount)| {
				if let Some(balance) = acc.get_mut(&account_id) {
					*balance = balance
						.checked_add(amount)
						.expect("balance cannot overflow when building genesis");
				} else {
					acc.insert(account_id.clone(), amount);
				}
				acc
			},
		)
		.into_iter()
		.chain(vec![(UnreleasedNativeVaultAccountId::get(), unreleased_native)])
		.collect::<Vec<(AccountId, Balance)>>();

	steam_runtime::GenesisConfig {
		frame_system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_indices: IndicesConfig { indices: vec![] },
		pallet_balances: BalancesConfig { balances },
		pallet_sudo: SudoConfig { key: root_key.clone() },
		pallet_collective_Instance1: Default::default(),
		pallet_membership_Instance1: GeneralCouncilMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_collective_Instance2: Default::default(),
		pallet_membership_Instance2: HonzonCouncilMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_collective_Instance3: Default::default(),
		pallet_membership_Instance3: HomaCouncilMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_collective_Instance4: Default::default(),
		pallet_membership_Instance4: TechnicalCommitteeMembershipConfig {
			members: vec![root_key.clone()],
			phantom: Default::default(),
		},
		pallet_membership_Instance5: OperatorMembershipEaveConfig {
			members: endowed_accounts.clone(),
			phantom: Default::default(),
		},
		pallet_membership_Instance6: OperatorMembershipBandConfig {
			members: endowed_accounts.clone(),
			phantom: Default::default(),
		},
		pallet_treasury: Default::default(),
		orml_tokens: TokensConfig {
			endowed_accounts: vec![
				(root_key.clone(), DOT, initial_balance),
			],
		},
		orml_vesting: VestingConfig { vesting: vec![] },
	    module_cdp_treasury: CdpTreasuryConfig {
			expected_collateral_auction_size: vec![
				(DOT, dollar(DOT)), // (currency_id, max size of a collateral auction)
				(RENBTC, 5 * cent(RENBTC)),
			],
		},
		module_cdp_engine: CdpEngineConfig {
			collaterals_params: vec![
				(
					DOT,
					Some(FixedU128::zero()),                             // stability fee for this collateral
					Some(FixedU128::saturating_from_rational(105, 100)), // liquidation ratio
					Some(FixedU128::saturating_from_rational(3, 100)),   // liquidation penalty rate
					Some(FixedU128::saturating_from_rational(110, 100)), // required liquidation ratio
					10_000_000 * dollar(EUSD),                           // maximum debit value in eUSD (cap)
				),
				(
					LDOT,
					Some(FixedU128::zero()),
					Some(FixedU128::saturating_from_rational(120, 100)),
					Some(FixedU128::saturating_from_rational(10, 100)),
					Some(FixedU128::saturating_from_rational(130, 100)),
					10_000_000 * dollar(EUSD),
				),
				(
					RENBTC,
					Some(FixedU128::zero()),
					Some(FixedU128::saturating_from_rational(110, 100)),
					Some(FixedU128::saturating_from_rational(4, 100)),
					Some(FixedU128::saturating_from_rational(115, 100)),
					10_000_000 * dollar(EUSD),
				),
			],
			global_interest_rate_per_sec: FixedU128::saturating_from_rational(
				1_547_126_000u128,
				1_000_000_000_000_000_000u128,
			), /* 5% APR */
		},
		module_airdrop: AirDropConfig {
			airdrop_accounts: {
				let eave_airdrop_accounts_json = &include_bytes!("../../../../resources/steam-airdrop-EAVE.json")[..];
				let eave_airdrop_accounts: Vec<(AccountId, Balance)> =
					serde_json::from_slice(eave_airdrop_accounts_json).unwrap();
				let ice_airdrop_accounts_json = &include_bytes!("../../../../resources/steam-airdrop-ICE.json")[..];
				let ice_airdrop_accounts: Vec<(AccountId, Balance)> =
					serde_json::from_slice(ice_airdrop_accounts_json).unwrap();

				eave_airdrop_accounts
					.iter()
					.map(|(account_id, eave_amount)| (account_id.clone(), AirDropCurrencyId::EAVE, *eave_amount))
					.chain(
						ice_airdrop_accounts
							.iter()
							.map(|(account_id, ice_amount)| (account_id.clone(), AirDropCurrencyId::ICE, *ice_amount)),
					)
					.collect::<Vec<_>>()
			},
		},
		module_evm: EVMConfig {
			accounts: evm_genesis_accounts,
			treasury: root_key,
		},
		module_staking_pool: StakingPoolConfig {
			staking_pool_params: module_staking_pool::Params {
				target_max_free_unbonded_ratio: FixedU128::saturating_from_rational(10, 100),
				target_min_free_unbonded_ratio: FixedU128::saturating_from_rational(5, 100),
				target_unbonding_to_free_ratio: FixedU128::saturating_from_rational(2, 100),
				unbonding_to_free_adjustment: FixedU128::saturating_from_rational(1, 1000),
				base_fee_rate: FixedU128::saturating_from_rational(2, 100),
			},
		},
		module_dex: DexConfig {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: EnabledTradingPairs::get(),
			initial_added_liquidity_pools: vec![],
		},
		parachain_info: ParachainInfoConfig {
			parachain_id: PARA_ID.into(),
		},
		ecosystem_renvm_bridge: RenVmBridgeConfig {
			ren_vm_public_key: hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
		},
		orml_nft: OrmlNFTConfig {
			tokens: {
				let nft_airdrop_json = &include_bytes!("../../../../resources/steam-airdrop-NFT.json")[..];
				let nft_airdrop: Vec<(
					AccountId,
					Vec<u8>,
					module_nft::ClassData<Balance>,
					Vec<(Vec<u8>, module_nft::TokenData<Balance>, Vec<AccountId>)>,
				)> = serde_json::from_slice(nft_airdrop_json).unwrap();

				let mut tokens = vec![];
				for (class_owner, class_meta, class_data, nfts) in nft_airdrop {
					let mut tokens_of_class = vec![];
					for (token_meta, token_data, token_owners) in nfts {
						token_owners.iter().for_each(|account_id| {
							tokens_of_class.push((account_id.clone(), token_meta.clone(), token_data.clone()));
						});
					}

					tokens.push((
						class_owner.clone(),
						class_meta.clone(),
						class_data.clone(),
						tokens_of_class,
					));
				}

				tokens
			},
		},
		module_collator_selection: CollatorSelectionConfig {
			invulnerables: initial_authorities.iter().cloned().map(|(acc, _, _, _)| acc).collect(),
			candidacy_bond: initial_staking,
			..Default::default()
		},
		pallet_session: SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|(acc, _, _, aura)| {
					(
						acc.clone(),          // account id
						acc,                  // validator id
						SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		pallet_aura: Default::default(),
	}
}
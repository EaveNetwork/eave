#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_cast)]

pub mod currency;
pub mod evm;
pub mod mocks;

use crate::evm::EvmAddress;

use codec::{Decode, Encode};

use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, RuntimeDebug,
};

use sp_std::{
	convert::{Into, TryFrom, TryInto},
	prelude::*,
};

pub use currency::{CurrencyId, TokenSymbol};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

pub type Decimals = u8;
pub type AssetId = u64;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on
/// the chain.
pub type Signature = MultiSignature;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like
/// the signature, this also isn't a fixed size when encoded, as different
/// cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Alias to the opaque account ID type for this chain, actually a
/// `AccountId32`. This is always 32 bytes.
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of
/// them.
pub type AccountIndex = u32;

/// Index of a transaction in the chain. 32-bit should be plenty.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An instant or duration in time.
pub type Moment = u64;

/// Counter for the number of eras that have passed.
pub type EraIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Signed version of Balance
pub type Amount = i128;

/// Auction ID
pub type AuctionId = u32;

/// Share type
pub type Share = u128;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Opaque, encoded, unchecked extrinsic.
pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AirDropCurrencyId {
	BEAM = 0,
	EAVE,
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AuthoritysOriginId {
	Root,
	EaveTreasury,
	SlipTreasury,
	DSWF,
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DataProviderId {
	Aggregated = 0,
	Eave = 1,
	Band = 2,
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPair(pub CurrencyId, pub CurrencyId);

impl TradingPair {
	pub fn new(currency_id_a: CurrencyId, currency_id_b: CurrencyId) -> Self {
		if currency_id_a > currency_id_b {
			TradingPair(currency_id_b, currency_id_a)
		} else {
			TradingPair(currency_id_a, currency_id_b)
		}
	}

	pub fn from_token_currency_ids(currency_id_0: CurrencyId, currency_id_1: CurrencyId) -> Option<Self> {
		match currency_id_0.is_token_currency_id() && currency_id_1.is_token_currency_id() {
			true if currency_id_0 > currency_id_1 => Some(TradingPair(currency_id_1, currency_id_0)),
			true if currency_id_0 < currency_id_1 => Some(TradingPair(currency_id_0, currency_id_1)),
			_ => None,
		}
	}

	pub fn get_dex_share_currency_id(&self) -> Option<CurrencyId> {
		CurrencyId::join_dex_share_currency_id(self.0, self.1)
	}
}

/// Ethereum precompiles
/// 0 - 0x400
/// Eave precompiles
/// 0x400 - 0x800
pub const PRECOMPILE_ADDRESS_START: u64 = 0x400;
/// Predeployed system contracts (except Mirrored ERC20)
/// 0x800 - 0x1000
pub const PREDEPLOY_ADDRESS_START: u64 = 0x800;
/// Mirrored Tokens
/// 0x01000000
pub const MIRRORED_TOKENS_ADDRESS_START: u64 = 0x01000000;
/// Mirrored NFT
/// 0x02000000
pub const MIRRORED_NFT_ADDRESS_START: u64 = 0x02000000;

pub type NFTBalance = u128;


#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EAVETokenSymbol {
	EAVE = 0,
	EUSD = 1,
	SBTC = 2,
}
impl TryFrom<u8> for EAVETokenSymbol {
	type Error = ();

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(EAVETokenSymbol::EAVE),
			1 => Ok(EAVETokenSymbol::EUSD),
			2 => Ok(EAVETokenSymbol::SBTC),
			_ => Err(()),
		}
	}
}
pub type PoolId = u32;
pub type PoolConfigId = u32;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EAVECurrencyId {
	Token(EAVETokenSymbol),
	DEXShare(EAVETokenSymbol, EAVETokenSymbol),
	ERC20(EvmAddress),
}

impl EAVECurrencyId {
	pub fn is_token_currency_id(&self) -> bool {
		matches!(self, EAVECurrencyId::Token(_))
	}

	pub fn is_dex_share_currency_id(&self) -> bool {
		matches!(self, EAVECurrencyId::DEXShare(_, _))
	}

	pub fn split_dex_share_currency_id(&self) -> Option<(Self, Self)> {
		match self {
			EAVECurrencyId::DEXShare(token_symbol_0, token_symbol_1) => {
				Some((EAVECurrencyId::Token(*token_symbol_0), EAVECurrencyId::Token(*token_symbol_1)))
			}
			_ => None,
		}
	}

	pub fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self> {
		match (currency_id_0, currency_id_1) {
			(EAVECurrencyId::Token(token_symbol_0), EAVECurrencyId::Token(token_symbol_1)) => {
				Some(EAVECurrencyId::DEXShare(token_symbol_0, token_symbol_1))
			}
			_ => None,
		}
	}
}

impl TryFrom<Vec<u8>> for EAVECurrencyId {
	type Error = ();
	fn try_from(v: Vec<u8>) -> Result<EAVECurrencyId, ()> {
		match v.as_slice() {
			b"EAVE" => Ok(EAVECurrencyId::Token(EAVETokenSymbol::EAVE)),
			b"EUSD" => Ok(EAVECurrencyId::Token(EAVETokenSymbol::EUSD)),
			b"SBTC" => Ok(EAVECurrencyId::Token(EAVETokenSymbol::SBTC)),
			_ => Err(()),
		}
	}
}

/// Note the pre-deployed ERC20 contracts depend on `EAVECurrencyId` implementation,
/// and need to be updated if any change.
impl TryFrom<[u8; 32]> for EAVECurrencyId {
	type Error = ();

	fn try_from(v: [u8; 32]) -> Result<Self, Self::Error> {
		if !v.starts_with(&[0u8; 29][..]) {
			return Err(());
		}

		// token
		if v[29] == 0 && v[31] == 0 {
			return v[30].try_into().map(EAVECurrencyId::Token);
		}

		// DEX share
		if v[29] == 1 {
			let left = v[30].try_into()?;
			let right = v[31].try_into()?;
			return Ok(EAVECurrencyId::DEXShare(left, right));
		}

		Err(())
	}
}

/// Note the pre-deployed ERC20 contracts depend on `EAVECurrencyId` implementation,
/// and need to be updated if any change.
impl From<EAVECurrencyId> for [u8; 32] {
	fn from(val: EAVECurrencyId) -> Self {
		let mut bytes = [0u8; 32];
		match val {
			EAVECurrencyId::Token(token) => {
				bytes[30] = token as u8;
			}
			EAVECurrencyId::DEXShare(left, right) => {
				bytes[29] = 1;
				bytes[30] = left as u8;
				bytes[31] = right as u8;
			}
			_ => {}
		}
		bytes
	}
}


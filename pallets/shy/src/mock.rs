//! Mocks for the SHY pallet

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, ord_parameter_types, parameter_types};
use frame_system::{offchain::SendTransactionTypes, EnsureSignedBy};
use orml_traits::parameter_type_with_key;
use primitives::{Balance, TokenSymbol};
use sp_core::H256;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::IdentityLookup,
	FixedPointNumber, ModuleId,
};
use sp_std::cell::RefCell;
use support::{AuctionManager, ExchangeRate, Price, PriceProvider, Rate, Ratio};

mod shy {
	pub use super::super::*;
}

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type AuctionId = u32;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;
pub const EAVE: CurrencyId = CurrencyId::Token(TokenSymbol::EAVE);
pub const EUSD: CurrencyId = CurrencyId::Token(TokenSymbol::EUSD);
pub const BTC: CurrencyId = CurrencyId::Token(TokenSymbol::XBTC);
pub const DOT: CurrencyId = CurrencyId::Token(TokenSymbol::DOT);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Module<Runtime>;
	type MaxLocks = ();
	type WeightInfo = ();
}
pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Amount, BlockNumber>;

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = EAVE;
}

impl orml_currencies::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
	pub const LoansModuleId: ModuleId = ModuleId(*b"eave/loan");
}

impl loans::Config for Runtime {
	type Event = Event;
	type Convert = shy_engine::DebitExchangeRateConvertor<Runtime>;
	type Currency = Tokens;
	type RiskManager = SHYEngineModule;
	type SHYTreasury = SHYTreasuryModule;
	type ModuleId = LoansModuleId;
	type OnUpdateLoan = ();
}

pub struct MockPriceSource;
impl PriceProvider<CurrencyId> for MockPriceSource {
	fn get_relative_price(_base: CurrencyId, _quote: CurrencyId) -> Option<Price> {
		Some(Price::one())
	}

	fn get_price(_currency_id: CurrencyId) -> Option<Price> {
		Some(Price::one())
	}

	fn lock_price(_currency_id: CurrencyId) {}

	fn unlock_price(_currency_id: CurrencyId) {}
}

pub struct MockAuctionManager;
impl AuctionManager<AccountId> for MockAuctionManager {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type AuctionId = AuctionId;

	fn new_collateral_auction(
		_refund_recipient: &AccountId,
		_currency_id: Self::CurrencyId,
		_amount: Self::Balance,
		_target: Self::Balance,
	) -> DispatchResult {
		Ok(())
	}

	fn new_debit_auction(_amount: Self::Balance, _fix: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn new_surplus_auction(_amount: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn cancel_auction(_id: Self::AuctionId) -> DispatchResult {
		Ok(())
	}

	fn get_total_debit_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_target_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_collateral_in_auction(_id: Self::CurrencyId) -> Self::Balance {
		Default::default()
	}

	fn get_total_surplus_in_auction() -> Self::Balance {
		Default::default()
	}
}

thread_local! {
	static IS_SHUTDOWN: RefCell<bool> = RefCell::new(false);
}

pub fn mock_shutdown() {
	IS_SHUTDOWN.with(|v| *v.borrow_mut() = true)
}

pub struct MockEmergencyShutdown;
impl EmergencyShutdown for MockEmergencyShutdown {
	fn is_shutdown() -> bool {
		IS_SHUTDOWN.with(|v| *v.borrow_mut())
	}
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

parameter_types! {
	pub const GetStableCurrencyId: CurrencyId = EUSD;
	pub const MaxAuctionsCount: u32 = 10_000;
	pub const SHYTreasuryModuleId: ModuleId = ModuleId(*b"eave/shyt");
}

impl shy_treasury::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type AuctionManagerHandler = MockAuctionManager;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type DEX = ();
	type MaxAuctionsCount = MaxAuctionsCount;
	type ModuleId = SHYTreasuryModuleId;
	type WeightInfo = ();
}

parameter_types! {
	pub CollateralCurrencyIds: Vec<CurrencyId> = vec![BTC, DOT];
	pub DefaultLiquidationRatio: Ratio = Ratio::saturating_from_rational(3, 2);
	pub DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::one();
	pub DefaultLiquidationPenalty: Rate = Rate::saturating_from_rational(10, 100);
	pub const MinimumDebitValue: Balance = 2;
	pub MaxSlippageSwapWithDEX: Ratio = Ratio::saturating_from_rational(50, 100);
	pub const UnsignedPriority: u64 = 1 << 20;
}

impl shy_engine::Config for Runtime {
	type Event = Event;
	type PriceSource = MockPriceSource;
	type CollateralCurrencyIds = CollateralCurrencyIds;
	type DefaultLiquidationRatio = DefaultLiquidationRatio;
	type DefaultDebitExchangeRate = DefaultDebitExchangeRate;
	type DefaultLiquidationPenalty = DefaultLiquidationPenalty;
	type MinimumDebitValue = MinimumDebitValue;
	type GetStableCurrencyId = GetStableCurrencyId;
	type SHYTreasury = SHYTreasuryModule;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type MaxSlippageSwapWithDEX = MaxSlippageSwapWithDEX;
	type DEX = ();
	type UnsignedPriority = UnsignedPriority;
	type EmergencyShutdown = MockEmergencyShutdown;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
		Shy: shy::{Module, Storage, Call, Event<T>},
		Tokens: orml_tokens::{Module, Storage, Event<T>, Config<T>},
		PalletBalances: pallet_balances::{Module, Call, Storage, Event<T>},
		Currencies: orml_currencies::{Module, Call, Event<T>},
		LoansModule: loans::{Module, Storage, Call, Event<T>},
		SHYTreasuryModule: shy_treasury::{Module, Storage, Call, Event<T>},
		SHYEngineModule: shy_engine::{Module, Storage, Call, Event<T>, Config, ValidateUnsigned},
	}
);

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;

impl<LocalCall> SendTransactionTypes<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
}
pub type ShyModule = Module<Runtime>;

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, BTC, 1000),
				(BOB, BTC, 1000),
				(ALICE, DOT, 1000),
				(BOB, DOT, 1000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}

//! An orml_authority trait implementation.

use crate::{
	EaveTreasuryModuleId, AccountId, AccountIdConversion, AuthoritysOriginId, BadOrigin, BlockNumber, DSWFModuleId,
	DispatchResult, EnsureRoot, EnsureRootOrHalfGeneralCouncil, EnsureRootOrHalfSlipCouncil,
	EnsureRootOrOneThirdsTechnicalCommittee, EnsureRootOrThreeFourthsGeneralCouncil,
	EnsureRootOrTwoThirdsTechnicalCommittee, SlipTreasuryModuleId, OneDay, Origin,
	OriginCaller, SevenDays, ZeroDay, HOURS,
};
pub use frame_support::traits::{schedule::Priority, EnsureOrigin, OriginTrait};
use frame_system::ensure_root;
use orml_authority::EnsureDelayed;

pub struct AuthorityConfigImpl;
impl orml_authority::AuthorityConfig<Origin, OriginCaller, BlockNumber> for AuthorityConfigImpl {
	fn check_schedule_dispatch(origin: Origin, _priority: Priority) -> DispatchResult {
		EnsureRoot::<AccountId>::try_origin(origin)
			.or_else(|o| EnsureRootOrHalfGeneralCouncil::try_origin(o).map(|_| ()))
			.or_else(|o| EnsureRootOrHalfSlipCouncil::try_origin(o).map(|_| ()))
			.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(()))
	}

	fn check_fast_track_schedule(
		origin: Origin,
		_initial_origin: &OriginCaller,
		new_delay: BlockNumber,
	) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if new_delay / HOURS < 12 {
				EnsureRootOrTwoThirdsTechnicalCommittee::ensure_origin(origin)
					.map_or_else(|e| Err(e.into()), |_| Ok(()))
			} else {
				EnsureRootOrOneThirdsTechnicalCommittee::ensure_origin(origin)
					.map_or_else(|e| Err(e.into()), |_| Ok(()))
			}
		})
	}

	fn check_delay_schedule(origin: Origin, _initial_origin: &OriginCaller) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			EnsureRootOrOneThirdsTechnicalCommittee::ensure_origin(origin).map_or_else(|e| Err(e.into()), |_| Ok(()))
		})
	}

	fn check_cancel_schedule(origin: Origin, initial_origin: &OriginCaller) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if origin.caller() == initial_origin
				|| EnsureRootOrThreeFourthsGeneralCouncil::ensure_origin(origin).is_ok()
			{
				Ok(())
			} else {
				Err(BadOrigin.into())
			}
		})
	}
}

impl orml_authority::AsOriginId<Origin, OriginCaller> for AuthoritysOriginId {
	fn into_origin(self) -> OriginCaller {
		match self {
			AuthoritysOriginId::Root => Origin::root().caller().clone(),
			AuthoritysOriginId::EaveTreasury => Origin::signed(EaveTreasuryModuleId::get().into_account())
				.caller()
				.clone(),
			AuthoritysOriginId::SlipTreasury => Origin::signed(SlipTreasuryModuleId::get().into_account())
				.caller()
				.clone(),
			AuthoritysOriginId::DSWF => Origin::signed(DSWFModuleId::get().into_account()).caller().clone(),
		}
	}

	fn check_dispatch_from(&self, origin: Origin) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			match self {
			AuthoritysOriginId::Root => <EnsureDelayed<
				SevenDays,
				EnsureRootOrThreeFourthsGeneralCouncil,
				BlockNumber,
				OriginCaller,
			> as EnsureOrigin<Origin>>::ensure_origin(origin)
			.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(())),
			AuthoritysOriginId::EaveTreasury => {
				<EnsureDelayed<OneDay, EnsureRootOrHalfGeneralCouncil, BlockNumber, OriginCaller> as EnsureOrigin<
					Origin,
				>>::ensure_origin(origin)
				.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(()))
			}
			AuthoritysOriginId::SlipTreasury => {
				<EnsureDelayed<OneDay, EnsureRootOrHalfSlipCouncil, BlockNumber, OriginCaller> as EnsureOrigin<
					Origin,
				>>::ensure_origin(origin)
				.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(()))
			}
			AuthoritysOriginId::DSWF => {
				<EnsureDelayed<ZeroDay, EnsureRoot<AccountId>, BlockNumber, OriginCaller> as EnsureOrigin<
						Origin,
					>>::ensure_origin(origin)
					.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(()))
			}
		}
		})
	}
}

//! Unit tests for the incentives module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};
use orml_rewards::PoolInfo;
use orml_traits::MultiCurrency;
use sp_runtime::{traits::BadOrigin, FixedPointNumber};

#[test]
fn deposit_dex_share_works() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(TokensModule::deposit(BTC_EUSD_LP, &ALICE, 10000));
		assert_eq!(TokensModule::free_balance(BTC_EUSD_LP, &ALICE), 10000);
		assert_eq!(
			TokensModule::free_balance(BTC_EUSD_LP, &IncentivesModule::account_id()),
			0
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexIncentive(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexSaving(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexIncentive(BTC_EUSD_LP), ALICE),
			(0, 0)
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexSaving(BTC_EUSD_LP), ALICE),
			(0, 0)
		);

		assert_ok!(IncentivesModule::deposit_dex_share(
			Origin::signed(ALICE),
			BTC_EUSD_LP,
			10000
		));
		let deposit_dex_share_event = Event::incentives(crate::Event::DepositDEXShare(ALICE, BTC_EUSD_LP, 10000));
		assert!(System::events()
			.iter()
			.any(|record| record.event == deposit_dex_share_event));

		assert_eq!(TokensModule::free_balance(BTC_EUSD_LP, &ALICE), 0);
		assert_eq!(
			TokensModule::free_balance(BTC_EUSD_LP, &IncentivesModule::account_id()),
			10000
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexIncentive(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 10000,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexSaving(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 10000,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexIncentive(BTC_EUSD_LP), ALICE),
			(10000, 0)
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexSaving(BTC_EUSD_LP), ALICE),
			(10000, 0)
		);
	});
}

#[test]
fn withdraw_dex_share_works() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(TokensModule::deposit(BTC_EUSD_LP, &ALICE, 10000));

		assert_noop!(
			IncentivesModule::withdraw_dex_share(Origin::signed(BOB), BTC_EUSD_LP, 10000),
			Error::<Runtime>::NotEnough,
		);

		assert_ok!(IncentivesModule::deposit_dex_share(
			Origin::signed(ALICE),
			BTC_EUSD_LP,
			10000
		));
		assert_eq!(TokensModule::free_balance(BTC_EUSD_LP, &ALICE), 0);
		assert_eq!(
			TokensModule::free_balance(BTC_EUSD_LP, &IncentivesModule::account_id()),
			10000
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexIncentive(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 10000,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexSaving(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 10000,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexIncentive(BTC_EUSD_LP), ALICE),
			(10000, 0)
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexSaving(BTC_EUSD_LP), ALICE),
			(10000, 0)
		);

		assert_ok!(IncentivesModule::withdraw_dex_share(
			Origin::signed(ALICE),
			BTC_EUSD_LP,
			8000
		));
		let withdraw_dex_share_event = Event::incentives(crate::Event::WithdrawDEXShare(ALICE, BTC_EUSD_LP, 8000));
		assert!(System::events()
			.iter()
			.any(|record| record.event == withdraw_dex_share_event));

		assert_eq!(TokensModule::free_balance(BTC_EUSD_LP, &ALICE), 8000);
		assert_eq!(
			TokensModule::free_balance(BTC_EUSD_LP, &IncentivesModule::account_id()),
			2000
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexIncentive(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 2000,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::pools(PoolId::DexSaving(BTC_EUSD_LP)),
			PoolInfo {
				total_shares: 2000,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexIncentive(BTC_EUSD_LP), ALICE),
			(2000, 0)
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::DexSaving(BTC_EUSD_LP), ALICE),
			(2000, 0)
		);
	});
}

#[test]
fn update_loans_incentive_rewards_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_loans_incentive_rewards(Origin::signed(ALICE), vec![]),
			BadOrigin
		);
		assert_eq!(IncentivesModule::loans_incentive_rewards(BTC), 0);
		assert_eq!(IncentivesModule::loans_incentive_rewards(DOT), 0);

		assert_ok!(IncentivesModule::update_loans_incentive_rewards(
			Origin::signed(4),
			vec![(BTC, 200), (DOT, 1000),],
		));
		assert_eq!(IncentivesModule::loans_incentive_rewards(BTC), 200);
		assert_eq!(IncentivesModule::loans_incentive_rewards(DOT), 1000);

		assert_ok!(IncentivesModule::update_loans_incentive_rewards(
			Origin::signed(4),
			vec![(BTC, 100), (BTC, 300), (BTC, 500),],
		));
		assert_eq!(IncentivesModule::loans_incentive_rewards(BTC), 500);
	});
}

#[test]
fn update_dex_incentive_rewards_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_dex_incentive_rewards(Origin::signed(ALICE), vec![]),
			BadOrigin
		);
		assert_noop!(
			IncentivesModule::update_dex_incentive_rewards(Origin::signed(4), vec![(BTC, 200), (DOT, 1000)],),
			Error::<Runtime>::InvalidCurrencyId
		);

		assert_eq!(IncentivesModule::dex_incentive_rewards(BTC_EUSD_LP), 0);
		assert_eq!(IncentivesModule::dex_incentive_rewards(DOT_EUSD_LP), 0);

		assert_ok!(IncentivesModule::update_dex_incentive_rewards(
			Origin::signed(4),
			vec![(BTC_EUSD_LP, 200), (DOT_EUSD_LP, 1000)],
		));
		assert_eq!(IncentivesModule::dex_incentive_rewards(BTC_EUSD_LP), 200);
		assert_eq!(IncentivesModule::dex_incentive_rewards(DOT_EUSD_LP), 1000);

		assert_ok!(IncentivesModule::update_dex_incentive_rewards(
			Origin::signed(4),
			vec![(BTC_EUSD_LP, 100), (BTC_EUSD_LP, 300), (BTC_EUSD_LP, 500),],
		));
		assert_eq!(IncentivesModule::dex_incentive_rewards(BTC_EUSD_LP), 500);
	});
}

#[test]
fn update_slip_incentive_reward_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_slip_incentive_reward(Origin::signed(ALICE), 100),
			BadOrigin
		);
		assert_eq!(IncentivesModule::slip_incentive_reward(), 0);

		assert_ok!(IncentivesModule::update_slip_incentive_reward(Origin::signed(4), 100));
		assert_eq!(IncentivesModule::slip_incentive_reward(), 100);
	});
}

#[test]
fn update_dex_saving_rates_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_dex_saving_rates(Origin::signed(ALICE), vec![]),
			BadOrigin
		);

		assert_noop!(
			IncentivesModule::update_dex_saving_rates(
				Origin::signed(4),
				vec![(BTC, Rate::saturating_from_rational(1, 10000)),],
			),
			Error::<Runtime>::InvalidCurrencyId
		);

		assert_eq!(IncentivesModule::dex_saving_rates(BTC_EUSD_LP), Rate::zero());
		assert_eq!(IncentivesModule::dex_saving_rates(DOT_EUSD_LP), Rate::zero());

		assert_ok!(IncentivesModule::update_dex_saving_rates(
			Origin::signed(4),
			vec![
				(BTC_EUSD_LP, Rate::saturating_from_rational(1, 10000)),
				(DOT_EUSD_LP, Rate::saturating_from_rational(1, 5000)),
			],
		));
		assert_eq!(
			IncentivesModule::dex_saving_rates(BTC_EUSD_LP),
			Rate::saturating_from_rational(1, 10000)
		);
		assert_eq!(
			IncentivesModule::dex_saving_rates(DOT_EUSD_LP),
			Rate::saturating_from_rational(1, 5000)
		);

		assert_ok!(IncentivesModule::update_dex_saving_rates(
			Origin::signed(4),
			vec![
				(BTC_EUSD_LP, Rate::saturating_from_rational(1, 20000)),
				(BTC_EUSD_LP, Rate::saturating_from_rational(1, 30000)),
				(BTC_EUSD_LP, Rate::saturating_from_rational(1, 40000)),
			],
		));
		assert_eq!(
			IncentivesModule::dex_saving_rates(BTC_EUSD_LP),
			Rate::saturating_from_rational(1, 40000)
		);
	});
}

#[test]
fn on_update_loan_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			RewardsModule::pools(PoolId::Loans(BTC)),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::Loans(BTC), ALICE),
			(0, 0)
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::Loans(BTC), BOB),
			(0, 0)
		);

		OnUpdateLoan::<Runtime>::happened(&(ALICE, BTC, 100, 0));
		assert_eq!(
			RewardsModule::pools(PoolId::Loans(BTC)),
			PoolInfo {
				total_shares: 100,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::Loans(BTC), ALICE),
			(100, 0)
		);

		OnUpdateLoan::<Runtime>::happened(&(BOB, BTC, 100, 500));
		assert_eq!(
			RewardsModule::pools(PoolId::Loans(BTC)),
			PoolInfo {
				total_shares: 700,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::Loans(BTC), BOB),
			(600, 0)
		);

		OnUpdateLoan::<Runtime>::happened(&(ALICE, BTC, -50, 100));
		assert_eq!(
			RewardsModule::pools(PoolId::Loans(BTC)),
			PoolInfo {
				total_shares: 650,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::Loans(BTC), ALICE),
			(50, 0)
		);

		OnUpdateLoan::<Runtime>::happened(&(BOB, BTC, -650, 600));
		assert_eq!(
			RewardsModule::pools(PoolId::Loans(BTC)),
			PoolInfo {
				total_shares: 50,
				total_rewards: 0,
				total_withdrawn_rewards: 0
			}
		);
		assert_eq!(
			RewardsModule::share_and_withdrawn_reward(PoolId::Loans(BTC), BOB),
			(0, 0)
		);
	});
}

#[test]
fn pay_out_works_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(TokensModule::deposit(EAVE, &LoansIncentivePool::get(), 10000));
		assert_ok!(TokensModule::deposit(EAVE, &DexIncentivePool::get(), 10000));
		assert_ok!(TokensModule::deposit(EUSD, &DexIncentivePool::get(), 10000));
		assert_ok!(TokensModule::deposit(EAVE, &SlipIncentivePool::get(), 10000));

		assert_eq!(TokensModule::free_balance(EAVE, &LoansIncentivePool::get()), 10000);
		assert_eq!(TokensModule::free_balance(EAVE, &ALICE), 0);
		IncentivesModule::payout(&ALICE, PoolId::Loans(BTC), 1000);
		assert_eq!(TokensModule::free_balance(EAVE, &LoansIncentivePool::get()), 9000);
		assert_eq!(TokensModule::free_balance(EAVE, &ALICE), 1000);

		assert_eq!(TokensModule::free_balance(EAVE, &DexIncentivePool::get()), 10000);
		assert_eq!(TokensModule::free_balance(EAVE, &BOB), 0);
		IncentivesModule::payout(&BOB, PoolId::DexIncentive(BTC), 1000);
		assert_eq!(TokensModule::free_balance(EAVE, &DexIncentivePool::get()), 9000);
		assert_eq!(TokensModule::free_balance(EAVE, &BOB), 1000);

		assert_eq!(TokensModule::free_balance(EUSD, &DexIncentivePool::get()), 10000);
		assert_eq!(TokensModule::free_balance(EUSD, &ALICE), 0);
		IncentivesModule::payout(&ALICE, PoolId::DexSaving(BTC), 1000);
		assert_eq!(TokensModule::free_balance(EUSD, &DexIncentivePool::get()), 9000);
		assert_eq!(TokensModule::free_balance(EUSD, &ALICE), 1000);

		assert_eq!(TokensModule::free_balance(EAVE, &SlipIncentivePool::get()), 10000);
		assert_eq!(TokensModule::free_balance(EAVE, &BOB), 1000);
		IncentivesModule::payout(&BOB, PoolId::Slip, 3000);
		assert_eq!(TokensModule::free_balance(EAVE, &SlipIncentivePool::get()), 7000);
		assert_eq!(TokensModule::free_balance(EAVE, &BOB), 4000);
	});
}

#[test]
fn accumulate_reward_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(IncentivesModule::update_loans_incentive_rewards(
			Origin::signed(4),
			vec![(BTC, 1000), (DOT, 2000),],
		));
		assert_ok!(IncentivesModule::update_dex_incentive_rewards(
			Origin::signed(4),
			vec![(BTC_EUSD_LP, 100), (DOT_EUSD_LP, 200),],
		));
		assert_ok!(IncentivesModule::update_slip_incentive_reward(Origin::signed(4), 30));
		assert_ok!(IncentivesModule::update_dex_saving_rates(
			Origin::signed(4),
			vec![
				(BTC_EUSD_LP, Rate::saturating_from_rational(1, 100)),
				(DOT_EUSD_LP, Rate::saturating_from_rational(1, 100)),
			],
		));

		assert_eq!(IncentivesModule::accumulate_reward(10, |_, _| {}), vec![]);

		RewardsModule::add_share(&ALICE, PoolId::Loans(BTC), 1);
		assert_eq!(IncentivesModule::accumulate_reward(20, |_, _| {}), vec![(EAVE, 1000)]);

		RewardsModule::add_share(&ALICE, PoolId::Loans(DOT), 1);
		assert_eq!(IncentivesModule::accumulate_reward(30, |_, _| {}), vec![(EAVE, 3000)]);

		RewardsModule::add_share(&ALICE, PoolId::DexIncentive(BTC_EUSD_LP), 1);
		RewardsModule::add_share(&ALICE, PoolId::DexSaving(BTC_EUSD_LP), 1);
		assert_eq!(
			IncentivesModule::accumulate_reward(40, |_, _| {}),
			vec![(EAVE, 3100), (EUSD, 5)]
		);

		RewardsModule::add_share(&ALICE, PoolId::DexIncentive(DOT_EUSD_LP), 1);
		RewardsModule::add_share(&ALICE, PoolId::DexSaving(DOT_EUSD_LP), 1);
		assert_eq!(
			IncentivesModule::accumulate_reward(50, |_, _| {}),
			vec![(EAVE, 3300), (EUSD, 9)]
		);

		RewardsModule::add_share(&ALICE, PoolId::Slip, 1);
		assert_eq!(
			IncentivesModule::accumulate_reward(50, |_, _| {}),
			vec![(EAVE, 3330), (EUSD, 9)]
		);

		assert_eq!(IncentivesModule::accumulate_reward(59, |_, _| {}), vec![]);

		mock_shutdown();
		assert_eq!(IncentivesModule::accumulate_reward(60, |_, _| {}), vec![]);
	});
}

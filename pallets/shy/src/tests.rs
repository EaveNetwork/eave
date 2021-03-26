//! Unit tests for the SHY pallet.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};
use orml_traits::Change;
use sp_runtime::FixedPointNumber;
use support::{Rate, Ratio};

#[test]
fn authorize_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(ShyModule::authorize(Origin::signed(ALICE), BTC, BOB));

		let authorization_event = Event::shy(crate::Event::Authorization(ALICE, BOB, BTC));
		assert!(System::events()
			.iter()
			.any(|record| record.event == authorization_event));

		assert_ok!(ShyModule::check_authorization(&ALICE, &BOB, BTC));
	});
}

#[test]
fn unauthorize_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(ShyModule::authorize(Origin::signed(ALICE), BTC, BOB));
		assert_ok!(ShyModule::check_authorization(&ALICE, &BOB, BTC));
		assert_ok!(ShyModule::unauthorize(Origin::signed(ALICE), BTC, BOB));

		let unauthorization_event = Event::shy(crate::Event::UnAuthorization(ALICE, BOB, BTC));
		assert!(System::events()
			.iter()
			.any(|record| record.event == unauthorization_event));

		assert_noop!(
			ShyModule::check_authorization(&ALICE, &BOB, BTC),
			Error::<Runtime>::NoAuthorization
		);
	});
}

#[test]
fn unauthorize_all_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(ShyModule::authorize(Origin::signed(ALICE), BTC, BOB));
		assert_ok!(ShyModule::authorize(Origin::signed(ALICE), DOT, CAROL));
		assert_ok!(ShyModule::unauthorize_all(Origin::signed(ALICE)));

		let unauthorization_all_event = Event::shy(crate::Event::UnAuthorizationAll(ALICE));
		assert!(System::events()
			.iter()
			.any(|record| record.event == unauthorization_all_event));

		assert_noop!(
			ShyModule::check_authorization(&ALICE, &BOB, BTC),
			Error::<Runtime>::NoAuthorization
		);
		assert_noop!(
			ShyModule::check_authorization(&ALICE, &BOB, DOT),
			Error::<Runtime>::NoAuthorization
		);
	});
}

#[test]
fn transfer_loan_from_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(SHYEngineModule::set_collateral_params(
			Origin::signed(1),
			BTC,
			Change::NewValue(Some(Rate::saturating_from_rational(1, 100000))),
			Change::NewValue(Some(Ratio::saturating_from_rational(3, 2))),
			Change::NewValue(Some(Rate::saturating_from_rational(2, 10))),
			Change::NewValue(Some(Ratio::saturating_from_rational(9, 5))),
			Change::NewValue(10000),
		));
		assert_ok!(ShyModule::adjust_loan(Origin::signed(ALICE), BTC, 100, 50));
		assert_ok!(ShyModule::authorize(Origin::signed(ALICE), BTC, BOB));
		assert_ok!(ShyModule::transfer_loan_from(Origin::signed(BOB), BTC, ALICE));
		assert_eq!(LoansModule::positions(BTC, BOB).collateral, 100);
		assert_eq!(LoansModule::positions(BTC, BOB).debit, 50);
	});
}

#[test]
fn transfer_unauthorization_loans_should_not_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			ShyModule::transfer_loan_from(Origin::signed(ALICE), BTC, BOB),
			Error::<Runtime>::NoAuthorization,
		);
	});
}

#[test]
fn adjust_loan_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(SHYEngineModule::set_collateral_params(
			Origin::signed(1),
			BTC,
			Change::NewValue(Some(Rate::saturating_from_rational(1, 100000))),
			Change::NewValue(Some(Ratio::saturating_from_rational(3, 2))),
			Change::NewValue(Some(Rate::saturating_from_rational(2, 10))),
			Change::NewValue(Some(Ratio::saturating_from_rational(9, 5))),
			Change::NewValue(10000),
		));
		assert_ok!(ShyModule::adjust_loan(Origin::signed(ALICE), BTC, 100, 50));
		assert_eq!(LoansModule::positions(BTC, ALICE).collateral, 100);
		assert_eq!(LoansModule::positions(BTC, ALICE).debit, 50);
	});
}

#[test]
fn on_emergency_shutdown_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		mock_shutdown();
		assert_noop!(
			ShyModule::adjust_loan(Origin::signed(ALICE), BTC, 100, 50),
			Error::<Runtime>::AlreadyShutdown,
		);
		assert_noop!(
			ShyModule::transfer_loan_from(Origin::signed(ALICE), BTC, BOB),
			Error::<Runtime>::AlreadyShutdown,
		);
	});
}

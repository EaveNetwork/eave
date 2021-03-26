use super::*;

use frame_support::{assert_err, assert_ok};

#[test]
fn currency_id_to_bytes_works() {
	assert_eq!(Into::<[u8; 32]>::into(CurrencyId::Token(TokenSymbol::EAVE)), [0u8; 32]);

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[0, 1, 0][..]);
	assert_eq!(Into::<[u8; 32]>::into(CurrencyId::Token(TokenSymbol::EUSD)), bytes);

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[0, 5, 0][..]);
	assert_eq!(Into::<[u8; 32]>::into(CurrencyId::Token(TokenSymbol::SBTC)), bytes);

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[1, 0, 1][..]);
	assert_eq!(
		Into::<[u8; 32]>::into(CurrencyId::DEXShare(TokenSymbol::EAVE, TokenSymbol::EUSD)),
		bytes
	);
}

#[test]
fn currency_id_try_from_bytes_works() {
	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[0, 1, 0][..]);
	assert_ok!(bytes.try_into(), CurrencyId::Token(TokenSymbol::EUSD));

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[0, 6, 0][..]);
	assert_err!(TryInto::<CurrencyId>::try_into(bytes), ());

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[1, 0, 1][..]);
	assert_ok!(
		bytes.try_into(),
		CurrencyId::DEXShare(TokenSymbol::EAVE, TokenSymbol::EUSD)
	);

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[1, 6, 0][..]);
	assert_err!(TryInto::<CurrencyId>::try_into(bytes), ());

	let mut bytes = [0u8; 32];
	bytes[29..].copy_from_slice(&[1, 0, 6][..]);
	assert_err!(TryInto::<CurrencyId>::try_into(bytes), ());
}

#[test]
fn currency_id_encode_decode_bytes_works() {
	let currency_id = CurrencyId::Token(TokenSymbol::EUSD);
	let bytes: [u8; 32] = currency_id.into();
	assert_ok!(bytes.try_into(), currency_id)
}

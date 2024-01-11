use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_next_kitty_id() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		assert_eq!(kitty_id, KittiesModule::next_kitty_id());
	});
}

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let kitty_id = 0;
		let account_id = 1;
		assert_eq!(kitty_id, KittiesModule::next_kitty_id());
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let kitty_id = 0;
		let account_id = 1;
		assert_eq!(kitty_id, KittiesModule::next_kitty_id());
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), account_id as u32);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		let new_owner_account_id = 2u64;

		// transfer
		assert_ok!(KittiesModule::transfer(Origin::signed(account_id), 0, new_owner_account_id));

		assert_noop!(
			KittiesModule::transfer(Origin::signed(account_id), 0, new_owner_account_id),
			Error::<Test>::NotOwner
		);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(new_owner_account_id))
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let kitty_id = 0;
		let account_id = 1;

		// parent id 相同
		assert_noop!(
			KittiesModule::breed(Origin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);

		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::breed(Origin::signed(account_id), kitty_id, kitty_id + 1));
	});
}

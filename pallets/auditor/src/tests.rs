use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use codec::Encode;

#[test]
fn save_audit_log_one_item() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		let audit_logs = Auditor::retrieve_audit_log(retrieve_file_name, retrieve_date);

		for result in audit_logs {
			// TODO: Refactor this test
			assert_eq!(&result.get_title(), &"log-title".encode());
			//assert_eq!(&result.get_content(), &"transaction with id 123 is processed".encode());
			//assert_eq!(&result.get_timestamp(), &"2021-10-08 17:30:00 UTC".encode());
			//assert_eq!(&result.get_reporter(), &1);
		} 

		assert_eq!(Auditor::retrieve_audit_log("log-file-name".encode(), "2021-10-08".encode()).len(), 1);
	});
}

#[test]
fn save_audit_log_with_sender_saving_two_items_on_same_logname_it_owns() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		let sender2 = Origin::signed(1);
		let file_name2 = "log-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:45:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		assert_eq!(Auditor::retrieve_audit_log(retrieve_file_name, retrieve_date).len(), 2);
	});
}

#[test]
fn dont_save_audit_log_if_a_sender_saves_on_already_taken_log() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		let sender2 = Origin::signed(2);
		let file_name2 = "log-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:45:00 UTC".encode();
		// Ensure that error is raised when another sender attempts to save with an already taken log name
		assert_noop!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2),Error::<Test>::AuditLogIdentifierCannotBeUsed);
	});
}

#[test]
fn save_audit_log_two_items_but_different_keys() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		let sender2 = Origin::signed(1);
		let file_name2 = "different-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:45:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		assert_eq!(Auditor::retrieve_audit_log(retrieve_file_name, retrieve_date).len(), 1);
	});
}

#[test]
fn open_log_for_ownership_claim() {
	new_test_ext().execute_with(|| {

		// SETUP
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		// ASSERT
		assert_ok!(open_log_for_ownership_claim(Origin::signed(1), "log-file-name".encode(), "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".encode()));

		// --------
	});
}
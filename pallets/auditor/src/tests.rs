use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use codec::Encode;

/*
#[test]
fn save_audit_log_test2() {
	new_test_ext().execute_with(|| {
		
        let content_hash = <Test as frame_system::Config>::Hash::zero();
		let sender = Origin::signed(1);
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, content_hash, timestamp));

		let expected_timestamp = "2021-10-08 17:30:00 UTC".encode();
		let expected_sender: u64 = 1;
        assert_eq!(Auditor::retrieve_audit_log(content_hash).get_timestamp(), expected_timestamp);
		assert_eq!(Auditor::retrieve_audit_log(content_hash).get_reporter(), expected_sender);
	});
}

#[test]
fn cant_save_logs_with_identical_hashes_test2() {
	new_test_ext().execute_with(|| {
		
        let content_hash = <Test as frame_system::Config>::Hash::zero();
		let sender = Origin::signed(1);
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, content_hash, timestamp));

		let second_content_hash = <Test as frame_system::Config>::Hash::zero();
		let second_timestamp = "2021-11-08 17:30:00 UTC".encode();
		let second_sender = Origin::signed(2);
        assert_noop!(Auditor::save_audit_log(second_sender, second_content_hash, second_timestamp), Error::<Test>::AuditLogIdAlreadyExists);
	});
}
*/

#[test]
fn save_audit_log_test() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, content, timestamp));

		let expected_content = "transaction with id 123 is processed".encode();
		let expected_sender: u64 = 1;

		let file_name2 = "log-file-name".encode();
		let timestamp2 = "2021-10-08 17:30:00 UTC".encode();
		let audit_log_result = Auditor::retrieve_audit_log(file_name2, timestamp2);

		let file_name3 = "log-file-name".encode();
		let timestamp3 = "2021-10-08 17:30:00 UTC".encode();
		let audit_log_result2 = Auditor::retrieve_audit_log(file_name3, timestamp3);

        assert_eq!(audit_log_result.get_content(), expected_content);
		assert_eq!(audit_log_result2.get_reporter(), expected_sender);
	});
}

#[test]
fn cant_save_logs_with_identical_hashes_test() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, content, timestamp));

		let second_sender = Origin::signed(2);
		let second_file_name = "log-file-name".encode();
		let second_content = "transaction with id 123 is processed".encode();
		let second_timestamp = "2021-10-08 17:30:00 UTC".encode();
        assert_noop!(Auditor::save_audit_log(second_sender, second_file_name, second_content, second_timestamp), Error::<Test>::AuditLogIdentifierAlreadyExists);
	});
}
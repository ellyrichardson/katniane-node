use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use codec::Encode;

#[test]
fn save_audit_log_test() {
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
fn cant_save_logs_with_identical_hashes_test() {
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

/*
fn create_hash_data(data: &u32) -> <Test as frame_system::Config>::Hash {
    data.using_encoded(<Test as frame_system::Config>::Hash::zero())
}*/

/*
#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}*/
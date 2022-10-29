use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use codec::Encode;
use frame_system::Origin;

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

		// SETUP to have an audit log saved
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		// ASSERT
		assert_ok!(Auditor::open_log_for_ownership_claim(Origin::signed(1), "log-file-name".encode(), [0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]));

		// --------
	});
}

// TODO: Complete this test
#[test]
fn claim_ownership_of_opened_log() {
	new_test_ext().execute_with(|| {

		// SETUP to have an audit log saved
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp));

		// TODO: need to come up with a way to make the claimer id a real one, instead of random [u8; 32]
		// ASSERT
		assert_ok!(Auditor::open_log_for_ownership_claim(Origin::signed(1), "log-file-name".encode(), [0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]));
		// Assert successful claiming of logs here
		
		// --------
	});
}

#[test]
fn retrieve_paginated_audit_log_with_one_as_selected_page_number() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp.clone()));

		let sender2 = Origin::signed(1);
		let file_name2 = "log-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:31:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2.clone()));

		let sender3 = Origin::signed(1);
		let file_name3 = "log-file-name".encode();
		let date3 = "2021-10-08".encode();
		let title3 = "log-title".encode();
		let content3 = "transaction with id 123 is processed".encode();
		let timestamp3 = "2021-10-08 17:32:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender3, file_name3, date3, title3, content3, timestamp3.clone()));

		let sender4 = Origin::signed(1);
		let file_name4 = "log-file-name".encode();
		let date4 = "2021-10-08".encode();
		let title4 = "log-title".encode();
		let content4 = "transaction with id 123 is processed".encode();
		let timestamp4 = "2021-10-08 17:33:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender4, file_name4, date4, title4, content4, timestamp4.clone()));

		let sender5 = Origin::signed(1);
		let file_name5 = "log-file-name".encode();
		let date5 = "2021-10-08".encode();
		let title5 = "log-title".encode();
		let content5 = "transaction with id 123 is processed".encode();
		let timestamp5 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender5, file_name5, date5, title5, content5, timestamp5.clone()));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		let max_result_count = 3;
		let selected_page_number = 1;
		let paginated_audit_logs = Auditor::retrieve_paginated_audit_logs(retrieve_file_name, retrieve_date, max_result_count, selected_page_number);

		assert_eq!(paginated_audit_logs.len(), 3);
		// These assertions ensure that only the first 3 items are returned, which is determined by the timestamp
		for result in paginated_audit_logs {
			if result.clone().get_timestamp() == timestamp.clone() {
				// Pass the assertion
				println!("[*] Audit Log 1 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp2.clone() {
				// Pass the assertion
				println!("[*] Audit Log 2 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp3.clone() {
				// Pass the assertion
				println!("[*] Audit Log 3 retrieved");
				assert_eq!(1, 1);
			} else {
				// Fail the assertion
				println!("[*] Expected Audit Log not retrieved");
				assert_eq!(0, 1);
			}
		} 
	});
}

#[test]
fn retrieve_paginated_audit_log_with_two_as_selected_page_number() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp.clone()));

		let sender2 = Origin::signed(1);
		let file_name2 = "log-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:31:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2.clone()));

		let sender3 = Origin::signed(1);
		let file_name3 = "log-file-name".encode();
		let date3 = "2021-10-08".encode();
		let title3 = "log-title".encode();
		let content3 = "transaction with id 123 is processed".encode();
		let timestamp3 = "2021-10-08 17:32:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender3, file_name3, date3, title3, content3, timestamp3.clone()));

		let sender4 = Origin::signed(1);
		let file_name4 = "log-file-name".encode();
		let date4 = "2021-10-08".encode();
		let title4 = "log-title".encode();
		let content4 = "transaction with id 123 is processed".encode();
		let timestamp4 = "2021-10-08 17:33:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender4, file_name4, date4, title4, content4, timestamp4.clone()));

		let sender5 = Origin::signed(1);
		let file_name5 = "log-file-name".encode();
		let date5 = "2021-10-08".encode();
		let title5 = "log-title".encode();
		let content5 = "transaction with id 123 is processed".encode();
		let timestamp5 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender5, file_name5, date5, title5, content5, timestamp5.clone()));

		let sender6 = Origin::signed(1);
		let file_name6 = "log-file-name".encode();
		let date6 = "2021-10-08".encode();
		let title6 = "log-title".encode();
		let content6 = "transaction with id 123 is processed".encode();
		let timestamp6 = "2021-10-08 17:35:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender6, file_name6, date6, title6, content6, timestamp6.clone()));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		let max_result_count = 3;
		let selected_page_number = 2;
		let paginated_audit_logs = Auditor::retrieve_paginated_audit_logs(retrieve_file_name, retrieve_date, max_result_count, selected_page_number);

		assert_eq!(paginated_audit_logs.len(), 3);
		// These assertions ensure that only the last 3 items are returned, which is determined by the timestamp
		for result in paginated_audit_logs {
			if result.clone().get_timestamp() == timestamp4.clone() {
				// Pass the assertion
				println!("[*] Audit Log 4 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp5.clone() {
				// Pass the assertion
				println!("[*] Audit Log 5 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp6.clone() {
				// Pass the assertion
				println!("[*] Audit Log 6 retrieved");
				assert_eq!(1, 1);
			} else {
				// Fail the assertion
				println!("[*] Expected Audit Log not retrieved");
				assert_eq!(0, 1);
			}
		} 
	});
}

#[test]
fn retrieve_paginated_audit_log_with_two_as_selected_page_number_but_there_is_not_enough_logs_to_reach_max_result_count() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp.clone()));

		let sender2 = Origin::signed(1);
		let file_name2 = "log-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:31:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2.clone()));

		let sender3 = Origin::signed(1);
		let file_name3 = "log-file-name".encode();
		let date3 = "2021-10-08".encode();
		let title3 = "log-title".encode();
		let content3 = "transaction with id 123 is processed".encode();
		let timestamp3 = "2021-10-08 17:32:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender3, file_name3, date3, title3, content3, timestamp3.clone()));

		let sender4 = Origin::signed(1);
		let file_name4 = "log-file-name".encode();
		let date4 = "2021-10-08".encode();
		let title4 = "log-title".encode();
		let content4 = "transaction with id 123 is processed".encode();
		let timestamp4 = "2021-10-08 17:33:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender4, file_name4, date4, title4, content4, timestamp4.clone()));

		let sender5 = Origin::signed(1);
		let file_name5 = "log-file-name".encode();
		let date5 = "2021-10-08".encode();
		let title5 = "log-title".encode();
		let content5 = "transaction with id 123 is processed".encode();
		let timestamp5 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender5, file_name5, date5, title5, content5, timestamp5.clone()));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		let max_result_count = 3;
		let selected_page_number = 2;
		let paginated_audit_logs = Auditor::retrieve_paginated_audit_logs(retrieve_file_name, retrieve_date, max_result_count, selected_page_number);

		assert_eq!(paginated_audit_logs.len(), 2);
		// These assertions ensure that only the last 2 items are returned, which is determined by the timestamp
		for result in paginated_audit_logs {
			if result.clone().get_timestamp() == timestamp4.clone() {
				// Pass the assertion
				println!("[*] Audit Log 4 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp5.clone() {
				// Pass the assertion
				println!("[*] Audit Log 5 retrieved");
				assert_eq!(1, 1);
			} else {
				// Fail the assertion
				println!("[*] Expected Audit Log not retrieved");
				assert_eq!(0, 1);
			}
		} 
	});
}

#[test]
fn retrieve_paginated_audit_log_with_two_as_selected_page_number_even_though_there_can_total_of_three_page_numbers() {
	new_test_ext().execute_with(|| {
		let sender = Origin::signed(1);
		let file_name = "log-file-name".encode();
		let date = "2021-10-08".encode();
		let title = "log-title".encode();
		let content = "transaction with id 123 is processed".encode();
		let timestamp = "2021-10-08 17:30:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender, file_name, date, title, content, timestamp.clone()));

		let sender2 = Origin::signed(1);
		let file_name2 = "log-file-name".encode();
		let date2 = "2021-10-08".encode();
		let title2 = "log-title".encode();
		let content2 = "transaction with id 123 is processed".encode();
		let timestamp2 = "2021-10-08 17:31:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender2, file_name2, date2, title2, content2, timestamp2.clone()));

		let sender3 = Origin::signed(1);
		let file_name3 = "log-file-name".encode();
		let date3 = "2021-10-08".encode();
		let title3 = "log-title".encode();
		let content3 = "transaction with id 123 is processed".encode();
		let timestamp3 = "2021-10-08 17:32:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender3, file_name3, date3, title3, content3, timestamp3.clone()));

		let sender4 = Origin::signed(1);
		let file_name4 = "log-file-name".encode();
		let date4 = "2021-10-08".encode();
		let title4 = "log-title".encode();
		let content4 = "transaction with id 123 is processed".encode();
		let timestamp4 = "2021-10-08 17:33:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender4, file_name4, date4, title4, content4, timestamp4.clone()));

		let sender5 = Origin::signed(1);
		let file_name5 = "log-file-name".encode();
		let date5 = "2021-10-08".encode();
		let title5 = "log-title".encode();
		let content5 = "transaction with id 123 is processed".encode();
		let timestamp5 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender5, file_name5, date5, title5, content5, timestamp5.clone()));

		let sender6 = Origin::signed(1);
		let file_name6 = "log-file-name".encode();
		let date6 = "2021-10-08".encode();
		let title6 = "log-title".encode();
		let content6 = "transaction with id 123 is processed".encode();
		let timestamp6 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender6, file_name6, date6, title6, content6, timestamp6.clone()));

		let sender7 = Origin::signed(1);
		let file_name7 = "log-file-name".encode();
		let date7 = "2021-10-08".encode();
		let title7 = "log-title".encode();
		let content7 = "transaction with id 123 is processed".encode();
		let timestamp7 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender7, file_name7, date7, title7, content7, timestamp7.clone()));

		let sender8 = Origin::signed(1);
		let file_name8 = "log-file-name".encode();
		let date8 = "2021-10-08".encode();
		let title8 = "log-title".encode();
		let content8 = "transaction with id 123 is processed".encode();
		let timestamp8 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender8, file_name8, date8, title8, content8, timestamp8.clone()));

		let sender9 = Origin::signed(1);
		let file_name9 = "log-file-name".encode();
		let date9 = "2021-10-08".encode();
		let title9 = "log-title".encode();
		let content9 = "transaction with id 123 is processed".encode();
		let timestamp9 = "2021-10-08 17:34:00 UTC".encode();
		// Dispatch a signed extrinsic.
		assert_ok!(Auditor::save_audit_log(sender9, file_name9, date9, title9, content9, timestamp9.clone()));

		let retrieve_file_name = "log-file-name".encode();
		let retrieve_date = "2021-10-08".encode();
		let max_result_count = 3;
		let selected_page_number = 2;
		let paginated_audit_logs = Auditor::retrieve_paginated_audit_logs(retrieve_file_name, retrieve_date, max_result_count, selected_page_number);

		assert_eq!(paginated_audit_logs.len(), 3);
		// These assertions ensure that only the middle 3 items are returned, which is determined by the timestamp
		for result in paginated_audit_logs {
			if result.clone().get_timestamp() == timestamp4.clone() {
				// Pass the assertion
				println!("[*] Audit Log 4 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp5.clone() {
				// Pass the assertion
				println!("[*] Audit Log 5 retrieved");
				assert_eq!(1, 1);
			} else if result.clone().get_timestamp() == timestamp6.clone() {
				// Pass the assertion
				println!("[*] Audit Log 6 retrieved");
				assert_eq!(1, 1);
			} else {
				// Fail the assertion
				println!("[*] Expected Audit Log not retrieved");
				assert_eq!(0, 1);
			}
		} 
	});
}
#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	#[api_version(5)]
	pub trait LogsRetrievalApi {
		pub fn retrieve_paginated_audit_log(log_key: Vec<u8>, log_date: Vec<u8>, max_result_count: u32, selected_page_num: u32) -> Vec<AuditLog<T::AccountId>>;
	}
}
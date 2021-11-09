//#![cfg_attr(not(feature = "std"), no_std)]

//use frame_support::dispatch::DispatchResult;
use sp_runtime::AccountId32;
//use sp_api::decl_runtime_apis;

pub use sp_api_proc_macro::decl_runtime_apis;

decl_runtime_apis! {
    /// The API to save and retrieve audit record
    /// 
    pub trait AuditorApi<Hash> where
		Hash: codec::Codec, {
            
		/// save audit record
        fn save_audit_record(account_id: AccountId32, content: Hash, timestamp: String) -> bool;
        fn retrieve_audit_record(content: Hash) -> bool;
    }
}
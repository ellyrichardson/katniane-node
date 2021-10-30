#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
//#[cfg(feature = "std")]
//include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

//use frame_support::dispatch::DispatchResult;
//use sp_runtime::AccountId32;
//use sp_api::decl_runtime_apis;

/*
decl_runtime_apis! {
    /// The API to save and retrieve audit record
    pub trait AuditorApi<Hash> where
		Hash: codec::Codec, {
		/// save audit record
        fn save_audit_record(account_id: AccountId32, content: Hash, timestamp: String) -> bool;
        fn retrieve_audit_record(content: Hash) -> bool;
    }
}*/
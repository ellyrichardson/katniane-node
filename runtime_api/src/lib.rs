#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    /// The API to save and retrieve audit record
    pub trait AuditorApi<Hash> where
		Hash: codec::Codec, {
		/// save audit record
        fn save_audit_record(content: Hash, timestamp: String) -> bool;
        fn retrieve_audit_record(content: Hash) -> bool;
    }
}
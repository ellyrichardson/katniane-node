#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use frame_support::inherent::Vec;
    use std::str;
    use chrono::{DateTime, Utc, Duration};
    use chrono::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug, TypeInfo)]
    pub struct AuditLog<AccountId> {
        // Change the timestamp to a timestamp type handled by Substrate itself
        // Reporter determines which system sent the log
        title: Vec<u8>,
        content: Vec<u8>,
        timestamp: Vec<u8>,
        reporter: AccountId,
    }

    impl <T> AuditLog<T> {
        pub fn get_title(self) -> Vec<u8> {
            self.title
        }

        pub fn get_content(self) -> Vec<u8> {
            self.content
        }

        pub fn get_timestamp(self) -> Vec<u8> {
            self.timestamp
        }

        pub fn get_reporter(self) -> T {
            self.reporter
        }
    }

    // Daily timestamps of a log file will be stored in the blockchain for consensus
    // pub type AuditLogTimestamp = Vec<u8>;
    // pub type AuditLogOf<T> = AuditLog<<T as frame_system::Config>::AccountId>;

    /*#[pallet::storage]
    #[pallet::getter(fn retrieve_audit_log)]
    pub(super) type AuditLogStorage<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::Hash, AuditLogOf<T>, ValueQuery>;*/

    /*
    #[pallet::storage]
    #[pallet::getter(fn some_nmap)]
    pub(super) type AuditLogStorage<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, Vec<u8>>,
            NMapKey<Blake2_128Concat, Vec<u8>>,
        ),
        AuditLog<T::AccountId>,
        ValueQuery,
    >;*/

    /*
    #[pallet::storage]
    #[pallet::getter(fn some_nmap)]
    pub(super) type AuditLogStorage<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        Vec<u8>, // <-- key 1
        Blake2_128Concat,
        Vec<u8>, // <-- key 2
        ValueQuery,
    >;*/

    pub type AuditLogFileName = Vec<u8>;
    pub type AuditLogTimestamp = Vec<u8>;

    #[pallet::storage]
    #[pallet::getter(fn retrieve_audit_log)]
    pub(super) type AuditLogStorage<T: Config> = StorageDoubleMap<_, Blake2_128Concat, AuditLogFileName, Blake2_128Concat, AuditLogTimestamp, AuditLog<T::AccountId>, ValueQuery>;

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive details for event
		/// parameters. [something, who]
		AuditLogInformationStored(AuditLogFileName, AuditLogTimestamp, T::AccountId),
	}

    // Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// This Audit Log ID exists in the chain.
        AuditLogIdentifierAlreadyExists
	}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		/// To add audit log
        #[pallet::weight(0)]
        pub fn save_audit_log(origin: OriginFor<T>, log_file_name: Vec<u8>, log_content: Vec<u8>, log_timestamp: Vec<u8>) -> DispatchResult {

            // The dispatch origin of this call must be a participant.
            let sender = ensure_signed(origin)?;

            // Verify audit log identifer is not taken
            ensure!(!AuditLogStorage::<T>::contains_key(&log_file_name, &log_timestamp), Error::<T>::AuditLogIdentifierAlreadyExists);

            // ------------ (WIP - Begin)
            // Might have to remove this WIP
            let log_timestamp_string = match str::from_utf8(&log_timestamp) {
                Ok(success) => success,
                Err(error) => panic!("Invalid UTF-8 sequence: {}", error),
            };

            // convert the string into DateTime<FixedOffset>
            let truncated_log_timestamp = DateTime::parse_from_rfc3339(log_timestamp_string).unwrap().round_subsecs(0);

            // ------------ (WIP - End)

            let audit_log = AuditLog {
                title: "test_item".encode(),
                content: log_content,
                timestamp: log_timestamp,
                reporter: sender.clone(),
            };

            <AuditLogStorage<T>>::insert(&log_file_name, &truncated_log_timestamp.to_rfc3339().to_string().into_bytes(), audit_log);

            // Emit the event that audit log has been added in chain
            Self::deposit_event(Event::AuditLogInformationStored(log_file_name, truncated_log_timestamp.to_rfc3339().to_string().into_bytes(), sender));

            // Return a successful DispatchResult
            Ok(())
        }
	}
}
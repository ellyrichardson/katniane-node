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

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // type AugitLogReporterOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug, TypeInfo)]
    pub struct AuditLog<AccountId> {
    //pub struct AuditLog {
        // Change the timestamp to a timestamp type handled by Substrate itself
        // Reporter determines which system sent the log
        timestamp: Vec<u8>,
        reporter: AccountId,
    }

    impl <T> AuditLog<T> {
        pub fn get_timestamp(self) -> Vec<u8> {
            self.timestamp
        }

        pub fn get_reporter(self) -> T {
            self.reporter
        }
    }

    /*
    impl codec::EncodeLike<<T as frame_system::Config>::AccountId> for AuditLog<<T as frame_system::Config>::AccountId> {

    }*/

    // Daily timestamps of a log file will be stored in the blockchain for consensus
    pub type AuditLogTimestamp = Vec<u8>;
    pub type AuditLogOf<T> = AuditLog<<T as frame_system::Config>::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn retrieve_audit_log)]
    pub(super) type AuditLogStorage<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::Hash, AuditLogOf<T>, ValueQuery>;

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive details for event
		/// parameters. [something, who]
		AuditLogInformationStored(T::Hash, T::AccountId),
	}

    // Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// This Audit Log ID exists in the chain.
        AuditLogIdAlreadyExists
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
        pub fn save_audit_log(origin: OriginFor<T>, content_hash: T::Hash, timestamp: Vec<u8>) -> DispatchResult {

            // The dispatch origin of this call must be `AugitLogReporter`.
            let sender = ensure_signed(origin)?;

            // Verify audit log id is taken
            ensure!(!AuditLogStorage::<T>::contains_key(&content_hash), Error::<T>::AuditLogIdAlreadyExists);

            let audit_log = AuditLog {
                timestamp,
                reporter: sender.clone(),
            };

            <AuditLogStorage<T>>::insert(content_hash, audit_log);

            // Emit the event that audit log has been added in chain
            Self::deposit_event(Event::AuditLogInformationStored(content_hash, sender));

            // Return a successful DispatchResult
            Ok(())
        }
	}
}
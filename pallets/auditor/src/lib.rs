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

    #[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug, TypeInfo)]
    pub struct AuditLogOpenForClaim<AccountId> {
        filename: Vec<u8>,
        assigned_claimer: AccountId,
        // timestamp is in unix epoch
        opened_for_claim_timestamp: u64,
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

    impl <T> AuditLogOpenForClaim<T> {
        pub fn get_filename(self) -> Vec<u8> {
            self.filename
        }

        pub fn get_assigned_claimer(self) -> T {
            self.assigned_claimer
        }

        pub fn get_opened_for_claim_timestamp(self) -> u64 {
            self.opened_for_claim_timestamp
        }
    }

    pub type AuditLogFileName = Vec<u8>;
    pub type AuditLogDate = Vec<u8>;
    //pub type AuditLogCollection<T> = Vec<AuditLog<T>>; // not used
    //pub type AuditLogOwnerCollection<T> = Vec<T::AccountId>; // not used
    //pub type AuditLogClaimCode<T> = T::AccountId;


    #[pallet::storage]
    #[pallet::getter(fn retrieve_audit_log)]
    pub(super) type AuditLogStorage<T: Config> = StorageDoubleMap<_, Blake2_128Concat, AuditLogFileName, Blake2_128Concat, AuditLogDate, Vec<AuditLog<T::AccountId>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn retrieve_audit_log_owner)]
    pub(super) type AuditLogOwnerStorage<T: Config> = StorageMap<_, Blake2_128Concat, AuditLogFileName, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn retrieve_audit_log_open_for_claim)]
    pub(super) type AuditLogOpenForClaimStorage<T: Config> = StorageMap<_, Blake2_128Concat, AuditLogFileName, AuditLogOpenForClaim<T::AccountId>, ValueQuery>;
   
    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive details for event
		/// parameters. [something, who]
		AuditLogInformationStored(AuditLogFileName, AuditLogDate, T::AccountId),
        // T::AccountId is included to specify who claimed the open log
        AuditLogClaimedForOwnership(AuditLogFileName, T::AccountId),
        // T::AccountId is included to specify who opened the log for claiming
        AuditLogOpenedForOwnershipClaim(AuditLogFileName, T::AccountId),
	}

    // Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
        AuditLogIdentifierCannotBeUsed,
        NoRightsToOpenAuditLogForClaiming,
        NotExistingAuditLogCantBeOpenForClaiming,
        NotAuthorizedToClaimAuditLog,
        AuditLogAlreadyOpenedForClaiming,
        AuditLogCantBeFound,
        AuditLogNotOpenedForClaiming
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
        pub fn save_audit_log(origin: OriginFor<T>, log_file_name: Vec<u8>, log_date: Vec<u8>, log_title: Vec<u8>, log_content: Vec<u8>, log_timestamp: Vec<u8>) -> DispatchResult {

            // The dispatch origin of this call must be a participant.
            let sender = ensure_signed(origin)?;

            let audit_log = AuditLog {
                title: log_title,
                content: log_content,
                timestamp: log_timestamp,
                reporter: sender.clone(),
            };

            let log_owners = AuditLogOwnerStorage::<T>::try_get(&log_file_name);
            match log_owners {
                // log file name is already owned, meaning it is existing
                Ok(owners) => {
                    // check if log file name owner is the transaction sender
                    if owners.contains(&sender) {
                        let mut audit_log_collection = <AuditLogStorage<T>>::get(&log_file_name, &log_date);
                        audit_log_collection.push(audit_log.clone());
                        <AuditLogStorage<T>>::insert(&log_file_name, &log_date, audit_log_collection);
                    } else {
                        frame_support::ensure!(0 == 1, <Error<T>>::AuditLogIdentifierCannotBeUsed);
                    }
                }
                Err(error) => {
                    // No owner for this log name yet, therefore it is not existing and is available
                    let mut new_audit_log_collection = Vec::new();
                    new_audit_log_collection.push(audit_log.clone());
                    <AuditLogStorage<T>>::insert(&log_file_name, &log_date, new_audit_log_collection);

                    // Track that the log name is owned by the sender
                    let mut new_audit_log_owners_collection = Vec::new();
                    new_audit_log_owners_collection.push(sender.clone());
                    <AuditLogOwnerStorage<T>>::insert(&log_file_name, new_audit_log_owners_collection);
                }
            }

            // Emit the event that audit log has been added in chain
            Self::deposit_event(Event::AuditLogInformationStored(log_file_name, log_date, sender));

            // Return a successful DispatchResult
            Ok(())
        }

        // TODO: Clean up the code for this function if possible
        #[pallet::weight(0)]
        pub fn claim_log(origin: OriginFor<T>, log_file_name: Vec<u8>) -> DispatchResult {

            // The dispatch origin of this call must be a participant.
            let claimer = ensure_signed(origin)?;

            

            // Checks if the audit log is open for claiming, and returns the assigned claimer for the log available for claiming
            let audit_log_open_for_claim = AuditLogOpenForClaimStorage::<T>::try_get(&log_file_name);
            match audit_log_open_for_claim {
                Ok(audit_log_for_claim) => {

                    // TODO: Check if the log file name claiming has expired

                    // Checks if claimer is the assigned claimer
                    frame_support::ensure!(&claimer == &audit_log_for_claim.assigned_claimer, <Error<T>>::NotAuthorizedToClaimAuditLog);

                    // Add the claimer as an owner of the audit log
                    let mut audit_log_owners_collection = <AuditLogOwnerStorage<T>>::get(&log_file_name);
                    audit_log_owners_collection.push(claimer.clone());
                    <AuditLogOwnerStorage<T>>::insert(&log_file_name, audit_log_owners_collection);

                    // Remove the audit log as open for claiming so that is not open anymore (delete from the AuditLogOpenForClaimStorage)
                    <AuditLogOpenForClaimStorage<T>>::remove(&log_file_name);

                    // Emit the event that the audit log has been claimed
                    Self::deposit_event(Event::AuditLogClaimedForOwnership(log_file_name, claimer));
                }
                Err(error) => {
                    // Raise error when the log file is not opened for claiming
                    frame_support::ensure!(0 == 1, <Error<T>>::AuditLogNotOpenedForClaiming);
                }
            }

            // Return a successful DispatchResult
            Ok(())
        }

        // TODO: Clean up the code for this function if possible 
        #[pallet::weight(0)]
        pub fn open_log_for_ownership_claim(origin: OriginFor<T>, log_file_name: Vec<u8>, claimer_pubkey: [u8; 32]) -> DispatchResult {

            // The dispatch origin of this call must be a participant.
            let sender = ensure_signed(origin)?;

            // TODO: Add a checker that the claimer_pubkey is length 32
            // Convert u32 raw byte to AccountId of the would be assigned claimer
            let claimer_account_id = T::AccountId::decode(&mut &claimer_pubkey[..]).unwrap_or_default();

            // Check if log is owned by someone
            let log_owners = AuditLogOwnerStorage::<T>::try_get(&log_file_name);
            match log_owners {
                // Log is owned by someone
                Ok(owners) => {
                    // Check if log is owned by the function caller
                    if owners.contains(&sender) {
                        // Check if the file is already open for claiming. Only one claiming can happen at a time
                        let is_log_already_open = AuditLogOpenForClaimStorage::<T>::try_get(&log_file_name);
                        match is_log_already_open {
                            Ok(is_log_already_open) => {
                               // Raise an error that the log is already opened
                                frame_support::ensure!(0 == 1, <Error<T>>::AuditLogAlreadyOpenedForClaiming);
                            }
                            Err(error) => {
                                let audit_log_open_for_claim = AuditLogOpenForClaim {
                                    filename: log_file_name.clone(),
                                    assigned_claimer: claimer_account_id.clone(),
                                    opened_for_claim_timestamp: 123456789, // dummy epoch date
                                };

                                // Open the log for ownership claim 
                                <AuditLogOpenForClaimStorage<T>>::insert(&log_file_name, audit_log_open_for_claim);

                                // Emit the event that an audit log has been opened for claiming
                                Self::deposit_event(Event::AuditLogOpenedForOwnershipClaim(log_file_name, sender));
                            }
                        }
                    } else {
                        frame_support::ensure!(0 == 1, <Error<T>>::NoRightsToOpenAuditLogForClaiming);
                    }

                    
                }
                Err(error) => {
                    // The log opened does not exist since it has no owner, Raise an error
                    frame_support::ensure!(0 == 1, <Error<T>>::AuditLogCantBeFound);
                }
            }
            
            // Return a successful DispatchResult
            Ok(())
        }
	}
}
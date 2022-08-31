//! RPC interface for the transaction payment pallet.

//pub use self::gen_client::Client as TransactionPaymentClient;
//use codec::{Codec, Decode};
//use codec::Codec as AccountId;
use std::sync::Arc;
use auditor_runtime_api::LogsRetrieval;
use auditor_pallet::AuditLog;

/*
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;*/
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, MaybeDisplay, Verify, IdentifyAccount},
    MultiSignature,
};

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[rpc(client, server)]
pub trait AuditorLogsRetrievalApi<BlockHash> {
	#[method(name = "retrieve_paginated_audit_log")]
	fn retrieve_paginated_audit_log(
        &self, 
        log_key: Vec<u8>, 
        log_date: Vec<u8>, 
        max_result_count: u32, 
        selected_page_num: u32,
        at: Option<BlockHash>
    ) -> RpcResult<Vec<AuditLog<AccountId>>>;
}

/// A struct that implements the [`LogRetrievalApi`].
pub struct AuditorLogsRetrieval<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> AuditorLogsRetrieval<C, P> {
	/// Create new `LogRetrievalApi` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}



#[async_trait]
impl<C, Block> AuditorLogsRetrievalApiServer<<Block as BlockT>::Hash>
    for AuditorLogsRetrieval<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: LogsRetrieval<Block, AccountId>,
{
    fn retrieve_paginated_audit_log(
        &self,
        log_key: Vec<u8>, 
        log_date: Vec<u8>, 
        max_result_count: u32, 
        selected_page_num: u32,
        at: Option<<Block as BlockT>::Hash>
    ) -> RpcResult<Vec<AuditLog<AccountId>>> {

        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

        let runtime_api_result = api.retrieve_paginated_audit_log(
            &at,
            log_key.clone(), 
            log_date.clone(), 
            max_result_count.clone(), 
            selected_page_num.clone()
        );
        runtime_api_result.map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query dispatch info.",
				Some(e.to_string()),
			))
			.into()
		})

        /*
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

        let runtime_api_result = api.get_sum(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })*/
    }
}
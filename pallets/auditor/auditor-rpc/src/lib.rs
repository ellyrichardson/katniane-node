use jsonrpc_derive::rpc;
use jsonrpc_core::Result;
use codec::Encode;

#[rpc]
pub trait Rpc<T> {
	/// Adds two numbers and returns a result
	#[rpc(name = "save_hash")]
	fn save_hash(&self, content_hash: Vec<u8>, timestamp: String) -> Result<u64>;
}

pub struct RpcImpl;
impl <T> Rpc<T> for RpcImpl {
	fn save_hash(&self, content_hash: Vec<u8>, timestamp: String) -> Result<u64> {
		// TODO: May have to uncomment in the future
		// auditor_pallet::Pallet::save_audit_log(&self, content_hash, timestamp.encode());
		Ok(0)
	}
}

fn main() {
	let mut io = jsonrpc_core::IoHandler::new();
	// TODO: May have to uncomment in the future
	// io.extend_with(RpcImpl.to_delegate())
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

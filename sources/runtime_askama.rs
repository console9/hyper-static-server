

use crate::hss;


use crate::{
		
		hss::ServerResult,
		hss::Extensions,
		hss::fail_with_message,
		
	};




pub trait StaticAskamaContext
		where Self : Sized
{
	fn new_with_defaults () -> ServerResult<Self> {
		fail_with_message! (0xe41380ce, "context can't be created with defaults!");
	}
	
	fn new_with_extensions (_extensions : &hss::Extensions) -> ServerResult<Self> {
		Self::new_with_defaults ()
	}
}




impl StaticAskamaContext for () {
	
	fn new_with_defaults () -> ServerResult<()> {
		Ok (())
	}
}


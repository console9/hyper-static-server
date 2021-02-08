

use crate::prelude::*;




pub fn main () -> ServerResult {
	
	let _routes = Routes::new ();
	let _handler = Handler::new (_routes);
	
	return hss::main_with_handler_dyn (_handler);
}


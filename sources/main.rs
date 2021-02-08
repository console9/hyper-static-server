

use crate::prelude::*;




pub fn main () -> ServerResult {
	
	let _routes = Routes::new ();
	
	let _handler = Handler::new (_routes);
	
	let _configuration = hss::Configuration::localhost_http ()
			.with_handler_dyn (_handler)
			.build () ?;
	
	return hss::Server::run_and_wait (_configuration);
}


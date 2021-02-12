

use crate::hss;




pub fn main_with_static (_routes : impl Into<hss::Routes>, _configuration : Option<hss::Configuration>) -> hss::ServerResult {
	
	let _routes = _routes.into ();
	let _handler = crate::server::StaticHandler::new (_routes);
	
	return hss::main_with_handler_dyn (_handler, _configuration);
}


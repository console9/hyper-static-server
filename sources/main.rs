

#[ allow (unused_imports) ]
use crate::hss;




#[ cfg (feature = "server") ]
pub fn main_serve_with_static (_routes : impl Into<hss::Routes>, _configuration : Option<hss::Configuration>) -> hss::ServerResult {
	
	let _routes = _routes.into ();
	let _handler = crate::server::StaticHandler::new (_routes);
	
	return hss::main_with_handler_dyn (_handler, _configuration);
}




#[ cfg (feature = "exporter") ]
pub fn main_export_with_static (_routes : impl Into<hss::Routes>) -> hss::ServerResult {
	
	let _routes = _routes.into ();
	
	match "cpio" {
		"debug" =>
			return crate::exporter::export_routes_debug (_routes),
		"cpio" =>
			return crate::exporter::export_routes_cpio (_routes, ::std::io::stdout ()),
		_ =>
			hss::panic_with_code (0x9ebba755),
	}
}


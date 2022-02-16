

#[ allow (unused_imports) ]
use crate::hss;




#[ cfg ( any (feature = "server", feature = "exporter") ) ]
pub fn main (_routes : impl Into<hss::Routes>) -> hss::ServerResult {
	return main_wrapper (_routes, None, None);
}


#[ cfg ( any (feature = "server", feature = "exporter") ) ]
pub fn main_wrapper (_routes : impl Into<hss::Routes>, _configuration : Option<hss::Configuration>, _arguments : Option<hss::CliArguments>) -> hss::ServerResult {
	
	let _arguments = hss::CliArguments::from_args ();
	
	fn _main_serve (_routes : impl Into<hss::Routes>, _configuration : Option<hss::Configuration>, _arguments : hss::CliArguments) -> hss::ServerResult {
			#[ cfg (feature = "server") ]
			return main_serve_with_static (_routes, _configuration, Some (_arguments));
			#[ cfg (not (feature = "server") ) ]
			hss::fail_with_message! (0x2504f6ba, "executable built without `server` feature!");
		}
	
	fn _main_export (_routes : impl Into<hss::Routes>, _arguments : hss::CliArguments) -> hss::ServerResult {
			#[ cfg (feature = "exporter") ]
			return main_export_with_static (_routes, Some (_arguments));
			#[ cfg (not (feature = "exporter") ) ]
			hss::fail_with_message! (0xfc32851f, "executable built without `exporter` feature!");
		}
	
	match _arguments.first_str () {
		
		None =>
			return _main_serve (_routes, _configuration, _arguments),
		Some ("server") | Some ("serve") =>
			return _main_serve (_routes, _configuration, _arguments.without_first ()),
		Some ("exporter") | Some ("export") =>
			return _main_export (_routes, _arguments.without_first ()),
		Some (_mode) if ! _mode.starts_with ("-") =>
			hss::fail_with_format! (0x98f52b4c, "invalid mode `{}`!", _mode),
		Some (_) =>
			return _main_serve (_routes, _configuration, _arguments),
	}
}




#[ cfg (feature = "server") ]
pub fn main_serve_with_static (_routes : impl Into<hss::Routes>, _configuration : Option<hss::Configuration>, _arguments : Option<hss::CliArguments>) -> hss::ServerResult {
	
	let _routes = _routes.into ();
	let _handler = crate::server::StaticHandler::new (_routes);
	
	return hss::main_with_handler_dyn (_handler, _configuration, _arguments);
}




#[ cfg (feature = "exporter") ]
pub fn main_export_with_static (_routes : impl Into<hss::Routes>, _arguments : Option<hss::CliArguments>) -> hss::ServerResult {
	
	let mut _arguments = hss::CliArguments::unwrap_or_args (_arguments);
	let _mode = _arguments.remove_first_str ();
	if ! _arguments.is_empty () {
		hss::fail_with_message! (0x2383b422, "unexpected extra arguments!");
	}
	
	let _routes = _routes.into ();
	
	match _mode.as_deref () {
		
		None | Some ("debug") =>
			return crate::exporter::export_routes_debug (_routes),
		
		Some ("cpio") =>
			return crate::exporter::export_routes_cpio (_routes, ::std::io::stdout ()),
		
		Some (_mode) if ! _mode.starts_with ("-") =>
			hss::fail_with_format! (0xf108089b, "invalid exporter mode `{}`!", _mode),
		
		Some (_) =>
			hss::fail_with_message! (0xf108089b, "unexpected extra arguments!"),
	}
}


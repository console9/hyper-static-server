



use ::std::{
		
		io,
		sync,
		
		io::Write as _,
		
	};


use crate::hss::{
		
		BodyExt as _,
		ResponseExt as _,
		
		ResultExtWrap as _,
		ResultExtPanic as _,
		
		fail_with_format,
		
	};


use crate::hss;




pub fn export_routes (_routes : impl Into<hss::Routes>) -> hss::ServerResult {
	
	let _runtime = hss::runtime_current_thread () ?;
	
	let _routes = _routes.into ();
	
	for _route in _routes.routes () {
		
		if let Some (_debug) = _route.debug.as_ref () {
			eprintln! ("[dd] [cad36aee]  **  {} -> {:?}", _route.path, _debug);
		} else {
			eprintln! ("[dd] [742523c0]  **  {}", _route.path);
		}
		
		let _route_match = match _routes.resolve (&_route.path) {
			Ok (Some (_route_match)) =>
				if sync::Arc::ptr_eq (&_route, &_route_match.route) {
					if _route_match.parameters.is_empty () {
						_route_match
					} else {
						fail_with_format! (0x2c72f7ed, "failed resolving route for `{}` (with parameters)!", _route.path);
					}
				} else {
					fail_with_format! (0x997d3eca, "failed resolving route for `{}` (resolution mismatch)!", _route.path);
				}
			Ok (None) =>
				fail_with_format! (0xa712904b, "failed resolving route for `{}` (resolution failed)!", _route.path),
			Err (_error) =>
				fail_with_format! (0xea0e6963, "failed resolving route for `{}` (resolution failed)!  //  {}", _route.path, _error),
		};
		
		let (_content_type, _buffer) = export_route (_route_match, &_runtime) ?;
		
		eprintln! ("[dd] [4a57dfd1]  >>  {} bytes of type `{}`;", _buffer.len (), _content_type.to_str ());
		
		if false {
			eprintln! ("\n");
			io::stderr () .write_all (&_buffer) .or_panic (0xecb244c3);
			eprintln! ("\n\n");
		}
	}
	
	return Ok (());
}




pub fn export_route (_route_match : hss::RouteMatched, _runtime : &hss::Runtime) -> hss::ServerResult<(hss::ContentType, Vec<u8>)> {
	
	let _route = sync::Arc::clone (&_route_match.route);
	
	let _request = hss::Request::get (_route.path.clone ()) .body (hss::Body::default ()) .or_wrap (0x5fbdc50e) ?;
	
	let _future = _route.handle (_request, _route_match);
	
	let _response = match _runtime.block_on (_future) {
		Ok (_response) =>
			_response,
		Err (_error) =>
			fail_with_format! (0x349294f0, "failed generating response for `{}` (handling failed)!  //  {}", _route.path, _error),
	};
	
	let _status = _response.status ();
	let _content_type = _response.content_type_or_unknown ();
	
	let _body = match _status {
		hss::consts::OK =>
			_response.into_body (),
		_ =>
			fail_with_format! (0xbb5e6327, "failed generating response for `{}` (status {})!", _route.path, _status),
	};
	
	let _buffer = match export_body (_body, _runtime) {
		Ok (_buffer) =>
			_buffer,
		Err (_error) =>
			fail_with_format! (0x622b887a, "failed generating response for `{}` (streaming failed)!  //  {}", _route.path, _error),
	};
	
	return Ok ((_content_type, _buffer));
}




pub fn export_body (_body : hss::BodyDynBox, _runtime : &hss::Runtime) -> hss::ServerResult<Vec<u8>> {
	
	let mut _body = _body;
	return _body.consume_to_vec (Some (_runtime));
}






use ::std::{
		
		io,
		sync,
		time,
		
		collections::HashSet,
		
		io::Write as _,
		
	};


use ::cpio::newc as cpio;


use crate::hss::{
		
		BodyExt as _,
		ResponseExt as _,
		
		ResultExtWrap as _,
		ResultExtPanic as _,
		
		fail_with_format,
		
	};


use crate::hss;




pub fn export_routes_debug (_routes : impl Into<hss::Routes>) -> hss::ServerResult {
	
	let mut _consumer = |_route : &hss::Route, _content_type : hss::ContentType, _content_buffer : Vec<u8>| {
			
			if let Some (_debug) = _route.extensions.get::<hss::RouteDebug> () {
				eprintln! ("[dd] [cad36aee]  **  {} -> {:?}", _route.path, _debug);
			} else {
				eprintln! ("[dd] [742523c0]  **  {}", _route.path);
			}
			
			eprintln! ("[dd] [4a57dfd1]  >>  {} bytes of type `{}`;", _content_buffer.len (), _content_type.to_str ());
			
			if false {
				eprintln! ("\n");
				io::stderr () .write_all (&_content_buffer) .or_panic (0xecb244c3);
				eprintln! ("\n\n");
			}
			
			Ok (())
		};
	
	return export_routes (_routes, &mut _consumer);
}




pub fn export_routes_cpio (_routes : impl Into<hss::Routes>, _output : impl io::Write) -> hss::ServerResult {
	
	let mut _output = _output;
	
	let _timestamp = time::SystemTime::now () .duration_since (time::UNIX_EPOCH) .or_wrap (0x9c419037) ?;
	let _timestamp : u32 = _timestamp.as_secs () .try_into () .or_wrap (0x451e1667) ?;
	
	let mut _paths_seen = HashSet::new ();
	let mut _folders = HashSet::new ();
	
	let mut _consumer = |_route : &hss::Route, _content_type : hss::ContentType, _content_buffer : Vec<u8>| {
			
			let _path = export_route_path (_route, _content_type) ?;
			let _content_size : u32 = _content_buffer.len () .try_into () .or_wrap (0xc3c8d6c2) ?;
			
			if _paths_seen.contains (&_path) {
				eprintln! ("[ww] [94617879]  duplicate path encountered `{}`;  ignoring!", _path);
			}
			_paths_seen.insert (_path.clone ());
			
			{
				let mut _folder = String::new ();
				for _path_component in _path.split ('/') {
					if ! _folder.is_empty () {
						_folder.push ('/');
					}
					_folder.push_str (_path_component);
					if _folder.len () == _path.len () {
						break;
					}
					if _folders.contains (&_folder) {
						continue;
					}
					if _paths_seen.contains (&_folder) {
						eprintln! ("[ww] [fa649895]  duplicate path encountered `{}`;  ignoring!", _folder);
					}
					_folders.insert (_folder.clone ());
					_paths_seen.insert (_folder.clone ());
					cpio::Builder::new (&_folder)
							.mode (0o755 | 0o040000)
							.mtime (_timestamp)
							.write (&mut _output, 0)
							.finish () .or_wrap (0x0bfd8f69) ?
						;
				}
			}
			
			let _cpio_entry = cpio::Builder::new (&_path)
					.mode (0o644 | 0o100000)
					.mtime (_timestamp)
				;
			let mut _cpio_entry = _cpio_entry.write (&mut _output, _content_size);
			_cpio_entry.write_all (&_content_buffer) .or_wrap (0x29b49136) ?;
			_cpio_entry.finish () .or_wrap (0x76be56bc) ?;
			
			Ok (())
		};
	
	export_routes (_routes, &mut _consumer) ?;
	
	cpio::trailer (&mut _output) .or_wrap (0x4975bcae) ?;
	
	return Ok (());
}




fn export_route_path (_route : &hss::Route, _content_type : hss::ContentType) -> hss::ServerResult<String> {
	
	if ! _route.path.starts_with ("/") || _route.path.is_empty () {
		fail_with_format! (0xea316ecf, "failed resolving path for `{}` (missing `/` prefix)!", _route.path);
	}
	let mut _route_path = _route.path[1..].to_owned ();
	
	if _route_path.ends_with ("/") || _route_path.is_empty () {
		match _content_type {
			hss::ContentType::Text =>
				_route_path.push_str ("index.txt"),
			hss::ContentType::Html =>
				_route_path.push_str ("index.html"),
			hss::ContentType::Xml =>
				_route_path.push_str ("index.xml"),
			hss::ContentType::Json =>
				_route_path.push_str ("index.json"),
			_ =>
				_route_path.push_str ("index.data"),
		}
	}
	
	let _extensions : &[&str] = match _content_type {
		hss::ContentType::Text =>
			&[".txt", ".text", ".md"],
		hss::ContentType::Html =>
			&[".html", ".htm", ".xhtml"],
		hss::ContentType::Xml =>
			&[".xml", ".xhtml"],
		hss::ContentType::Json =>
			&[".json"],
		_ =>
			&[],
	};
	
	if ! _extensions.is_empty () {
		let mut _has_extension = false;
		for _extension in _extensions {
			if _route_path.ends_with (_extension) {
				_has_extension = true;
				break;
			}
		}
		if ! _has_extension {
			_route_path.push_str (_extensions[0]);
		}
	}
	
	return Ok (_route_path);
}




pub fn export_routes <Consumer> (_routes : impl Into<hss::Routes>, _consumer : Consumer) -> hss::ServerResult
	where
		Consumer : FnMut (&hss::Route, hss::ContentType, Vec<u8>) -> hss::ServerResult,
{
	let mut _consumer = _consumer;
	
	let _runtime = hss::runtime_current_thread () ?;
	
	let _routes = _routes.into ();
	
	for _route in _routes.routes () {
		
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
		
		let (_content_type, _content_buffer) = export_route (_route_match, &_runtime) ?;
		
		_consumer (&_route, _content_type, _content_buffer) ?
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


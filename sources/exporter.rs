



use ::std::{
		
		io,
		sync,
		time,
		
		collections::HashSet,
		
		io::Write as _,
		
	};


use ::cpio::newc as cpio;


use crate::{
		
		StaticRouteDebug,
		
	};


use crate::hss::{
		
		BodyExt as _,
		ResponseExt as _,
		
		ResultExtWrap as _,
		
		fail_with_format,
		
	};


use crate::hss;




pub fn export_routes_debug (_routes : impl Into<hss::Routes>, _output : impl io::Write) -> hss::ServerResult {
	
	let mut _output = _output;
	
	let mut _consumer = |_route : &hss::Route, _content_type : hss::ContentType, _content_buffer : Vec<u8>| {
			
			if let Some (_debug) = _route.extensions.get::<StaticRouteDebug> () {
				write! (_output, "**  {} -> {:?}\n", _route.path, _debug) .or_wrap (0x99260590) ?;
			} else {
				write! (_output, "**  {}\n", _route.path) .or_wrap (0xb7ff2169) ?;
			}
			
			write! (_output, ">>  {} bytes of type `{}`;\n", _content_buffer.len (), _content_type.to_str ()) .or_wrap (0xaea1edf5) ?;
			
			Ok (())
		};
	
	return export_routes_all (_routes, &mut _consumer);
}




pub fn export_routes_dump (_routes : impl Into<hss::Routes>, _route_path : &str, _output : impl io::Write) -> hss::ServerResult {
	
	let mut _output = _output;
	
	let mut _consumer = |_route : &hss::Route, _content_type : hss::ContentType, _content_buffer : Vec<u8>| {
			_output.write_all (&_content_buffer) .or_wrap (0x2a238b7f)
		};
	
	return export_routes_one (_routes, _route_path, &mut _consumer);
}




pub fn export_routes_cpio (_routes : impl Into<hss::Routes>, _output : impl io::Write) -> hss::ServerResult {
	
	let mut _output = _output;
	
	let _timestamp = time::SystemTime::now () .duration_since (time::UNIX_EPOCH) .or_wrap (0x9c419037) ?;
	let _timestamp : u32 = _timestamp.as_secs () .try_into () .or_wrap (0x451e1667) ?;
	
	let mut _paths_seen = HashSet::new ();
	let mut _folders = HashSet::new ();
	
	let mut _consumer = |_route : &hss::Route, _content_type : hss::ContentType, _content_buffer : Vec<u8>| {
			
			let _path = export_routes_cpio_path (_route, _content_type) ?;
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
	
	export_routes_all (_routes, &mut _consumer) ?;
	
	cpio::trailer (&mut _output) .or_wrap (0x4975bcae) ?;
	
	return Ok (());
}




fn export_routes_cpio_path (_route : &hss::Route, _content_type : hss::ContentType) -> hss::ServerResult<String> {
	
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




pub fn export_routes_all <Consumer> (_routes : impl Into<hss::Routes>, _consumer : Consumer) -> hss::ServerResult
	where
		Consumer : FnMut (&hss::Route, hss::ContentType, Vec<u8>) -> hss::ServerResult,
{
	let _routes = _routes.into ();
	let _runtime = hss::runtime_current_thread () ?;
	
	let mut _consumer = _consumer;
	for _route in _routes.routes () {
		export_route_resolve (&_routes, &_route.path, &mut _consumer, &_runtime) ?;
	}
	
	return Ok (());
}




pub fn export_routes_one <Consumer> (_routes : impl Into<hss::Routes>, _route_path : &str, _consumer : Consumer) -> hss::ServerResult
	where
		Consumer : FnMut (&hss::Route, hss::ContentType, Vec<u8>) -> hss::ServerResult,
{
	let _routes = _routes.into ();
	let _runtime = hss::runtime_current_thread () ?;
	
	export_route_resolve (&_routes, _route_path, _consumer, &_runtime) ?;
	
	return Ok (());
}




pub fn export_route_resolve <Consumer> (_routes : &hss::Routes, _route_path : &str, _consumer : Consumer, _runtime : &hss::Runtime) -> hss::ServerResult
	where
		Consumer : FnOnce (&hss::Route, hss::ContentType, Vec<u8>) -> hss::ServerResult,
{
	let _route_matched = match _routes.resolve (_route_path) {
		Ok (Some (_route_matched)) =>
			_route_matched,
		Ok (None) =>
			fail_with_format! (0xa712904b, "failed resolving route for `{}` (resolution failed)!", _route_path),
		Err (_error) =>
			fail_with_format! (0xea0e6963, "failed resolving route for `{}` (resolution failed)!  //  {}", _route_path, _error),
	};
	
	return export_route_matched (_route_matched, _consumer, _runtime);
}




pub fn export_route_matched <Consumer> (_route_matched : hss::RouteMatched, _consumer : Consumer, _runtime : &hss::Runtime) -> hss::ServerResult
	where
		Consumer : FnOnce (&hss::Route, hss::ContentType, Vec<u8>) -> hss::ServerResult,
{
	let _route = _route_matched.route.clone ();
	
	let (_content_type, _content_buffer) = export_route_matched_0 (_route_matched, &_runtime) ?;
	
	_consumer (&_route, _content_type, _content_buffer) ?;
	
	return Ok (());
}




pub fn export_route_matched_0 (_route_matched : hss::RouteMatched, _runtime : &hss::Runtime) -> hss::ServerResult<(hss::ContentType, Vec<u8>)> {
	
	let _route = sync::Arc::clone (&_route_matched.route);
	
	let _request = hss::Request::get (_route.path.clone ()) .body (hss::Body::default ()) .or_wrap (0x5fbdc50e) ?;
	
	let _future = _route.handle (_request, _route_matched);
	
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




fn export_body (_body : hss::BodyDynBox, _runtime : &hss::Runtime) -> hss::ServerResult<Vec<u8>> {
	
	let mut _body = _body;
	return _body.consume_to_vec (Some (_runtime));
}


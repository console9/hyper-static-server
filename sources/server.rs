

use crate::hss;

use crate::hss::RequestExt as _;
use crate::hss::ResponseExtBuild as _;




pub struct StaticHandler {
	routes : hss::Routes,
	random_token : String,
}


impl StaticHandler {
	
	pub fn new (_routes : hss::Routes) -> Self {
		Self {
				routes : _routes,
				random_token : random_token (),
			}
	}
}




impl StaticHandler {
	
	
	pub fn serve (&self, _request : hss::Request<hss::Body>) -> hss::HandlerFutureDynBox {
		
		if ! _request.is_get () {
			eprintln! ("[ww] [1211a46c]  failing `{}` with 405 (method not allowed)", _request.uri_path ());
			return hss::Response::new_method_not_allowed () .into ();
		}
		
		if _request.uri_path () .starts_with ("/__/") {
			return self.serve_special (_request);
		}
		
		#[ cfg (debug_assertions) ]
		eprintln! ("[ii] [9ada1c36]  serving `{}`...", _request.uri_path ());
		
		match self.routes.try_handle (_request) {
			Ok (_future) =>
				return _future,
			Err (_request) =>
				return self.serve_404 (_request),
		}
	}
	
	
	pub fn serve_special (&self, _request : hss::Request<hss::Body>) -> hss::HandlerFutureDynBox {
		
		match _request.uri_path () {
			"/__/routes" =>
				return self.serve_routes_index (_request),
			"/__/heartbeat" =>
				return hss::Response::new_200 () .into (),
			"/__/reload.txt" =>
				return hss::Response::new_200_with_text (self.random_token.clone ()) .into (),
			"/__/reload.js" =>
				return hss::Response::new_200_with_body (
						Some (hss::ContentType::Js),
						& include_bytes! ("./reload.js") [..],
					) .into (),
			_ =>
				return self.serve_404 (_request),
		}
	}
	
	
	pub fn serve_routes_index (&self, _request : hss::Request<hss::Body>) -> hss::HandlerFutureDynBox {
		
		let mut _buffer = String::with_capacity (128 * 1024);
		
		fn _sanitize (_text : &str, _quote : bool) -> String {
			let _text = _text.replace ("&", "&amp;");
			let _text = _text.replace ("<", "&lt;");
			let _text = _text.replace (">", "&gt;");
			let _text = if _quote {
				_text.replace ("\"", "&quot;")
			} else {
				_text
			};
			return _text;
		}
		
		_buffer.push_str ("<!DOCTYPE html><html lang=\"en\"><head><title>Vaktundur -- routes</title></head>\n");
		_buffer.push_str ("<body><ul>\n");
		for _route in self.routes.routes () {
			_buffer.push_str ("<li><code><a href=\"");
			_buffer.push_str (& _sanitize (&_route.path, true));
			_buffer.push_str ("\">");
			_buffer.push_str (& _sanitize (&_route.path, false));
			_buffer.push_str ("</a> -> <span>");
			_buffer.push_str (& _sanitize (& format! ("{:?}", _route.debug), false));
			_buffer.push_str ("</span></code></li>\n");
		}
		_buffer.push_str ("</ul></body>\n");
		
		return hss::Response::new_200_with_html (_buffer) .into ();
	}
	
	
	pub fn serve_404 (&self, _request : hss::Request<hss::Body>) -> hss::HandlerFutureDynBox {
		eprintln! ("[ee] [4ba52b89]  failing `{}` with 404 (not found)", _request.uri_path ());
		return hss::Response::new_404 () .into ();
	}
}




impl hss::HandlerDyn for StaticHandler {
	
	fn handle (&self, _request : hss::Request<hss::Body>) -> hss::HandlerFutureDynBox {
		return self.serve (_request);
	}
}




fn random_token () -> String {
	use ::rand::Rng as _;
	let mut _rand = ::rand::thread_rng ();
	let _token = _rand.gen::<u128> ();
	format! ("{:0x}", _token)
}


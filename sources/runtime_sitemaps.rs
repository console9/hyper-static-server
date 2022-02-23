

use crate::hss;


use crate::hss::{
		
		ResponseExt as _,
		
	};




pub enum SitemapFrequency {
	Always,
	Hourly,
	Daily,
	Weekly,
	Monthly,
	Yearly,
	Never,
}


pub enum SitemapFormat {
	Xml,
	Text,
}




pub struct RouteSitemapEntry {
	pub included : Option<bool>,
	pub frequency : Option<SitemapFrequency>,
	pub priority : Option<f32>,
}


impl RouteSitemapEntry {
	
	pub fn new () -> Self {
		Self {
				included : None,
				frequency : None,
				priority : None,
			}
	}
}




pub struct RoutesSitemapResource {
	routes : Option<hss::Routes>,
	format : SitemapFormat,
}


impl RoutesSitemapResource {
	
	pub fn new (_format : SitemapFormat) -> Self {
		Self::new_with_routes (None, _format)
	}
	
	pub fn new_xml () -> Self {
		Self::new (SitemapFormat::Xml)
	}
	
	pub fn new_text () -> Self {
		Self::new (SitemapFormat::Text)
	}
	
	pub fn new_with_routes (_routes : Option<hss::Routes>, _format : SitemapFormat) -> Self {
		Self {
				routes : _routes,
				format : _format,
			}
	}
}


impl hss::HandlerSimpleSync for RoutesSitemapResource {
	
	fn handle (&self, _request : &hss::Request<hss::Body>, _response : &mut hss::Response<hss::Body>) -> hss::ServerResult {
		
		let _routes = if let Some (_routes) = &self.routes {
			_routes.clone ()
		} else if let Some (_route_matched) = _request.extensions () .get::<hss::RouteMatched> () {
			_route_matched.routes.clone ()
		} else if let Some (_routes) = _request.extensions () .get::<hss::Routes> () {
			_routes.clone ()
		} else {
			hss::fail_with_code! (0xee535008);
		};
		
		let mut _sitemap_routes = Vec::new ();
		for _route in _routes.routes () {
			if let Some (_entry) = _route.extensions.get::<RouteSitemapEntry> () {
				if _entry.included.unwrap_or (true) {
					_sitemap_routes.push ((_route, _entry));
				}
			}
		}
		
		
		let (_body, _content_type) = match self.format {
			
			SitemapFormat::Xml => {
				// FIXME!
				let mut _buffer = String::with_capacity (16 * 1024);
				(_buffer, hss::ContentType::Xml)
			}
			
			SitemapFormat::Text => {
				let mut _buffer = String::with_capacity (16 * 1024);
				for (_route, _entry) in _sitemap_routes {
					_buffer.push_str (&_route.path);
					_buffer.push_str ("\n");
				}
				(_buffer, hss::ContentType::Text)
			}
		};
		
		_response.set_status_200 ();
		_response.set_content_type (_content_type);
		_response.set_body (_body);
		
		Ok (())
	}
}




use crate::hss;


#[ allow (unused_imports) ]
use ::std::{
		
		str::FromStr as _,
		
	};


#[ allow (unused_imports) ]
use crate::hss::{
		
		ResponseExt as _,
		
		ResultExtWrap as _,
		ResultExtPanic as _,
		
	};




pub enum SitemapFormat {
	#[ cfg (feature = "runtime-sitemaps-xml") ]
	Xml,
	Text,
}


pub enum SitemapFrequency {
	Always,
	Hourly,
	Daily,
	Weekly,
	Monthly,
	Yearly,
	Never,
}


pub struct SitemapPriority (f32);




impl SitemapFormat {
	
	pub fn from_str (_string : &str) -> hss::ServerResult<Self> {
		match _string {
			#[ cfg (feature = "runtime-sitemaps-xml") ]
			"xml" => Ok (Self::Xml),
			"text" => Ok (Self::Text),
			_ => hss::fail_with_code! (0xdf9d3bb7),
		}
	}
	
	pub fn from_str_must (_string : &str) -> Self {
		Self::from_str (_string) .or_panic (0x685370b7)
	}
}




impl SitemapFrequency {
	
	pub fn from_str (_string : &str) -> hss::ServerResult<Option<Self>> {
		match _string {
			"default" => Ok (None),
			"always" => Ok (Some (Self::Always)),
			"hourly" => Ok (Some (Self::Hourly)),
			"daily" => Ok (Some (Self::Daily)),
			"weekly" => Ok (Some (Self::Weekly)),
			"monthly" => Ok (Some (Self::Monthly)),
			"yearly" => Ok (Some (Self::Yearly)),
			"never" => Ok (Some (Self::Never)),
			_ => hss::fail_with_code! (0xb2001a05),
		}
	}
	
	pub fn from_str_must (_string : &str) -> Option<Self> {
		Self::from_str (_string) .or_panic (0x256f6fc2)
	}
}


impl SitemapPriority {
	
	pub fn to_f32 (&self) -> f32 {
		self.0
	}
	
	pub fn from_str (_string : &str) -> hss::ServerResult<Option<Self>> {
		match _string {
			"default" =>
				Ok (None),
			_ =>
				Self::from_f32 (f32::from_str (_string) .or_wrap (0x1dd1e4e7) ?)
		}
	}
	
	pub fn from_f32 (_value : f32) -> hss::ServerResult<Option<Self>> {
		if _value < 0.0 {
			hss::fail_with_code! (0x55b5a84b);
		}
		if _value > 1.0 {
			hss::fail_with_code! (0xa233806b);
		}
		Ok (Some (Self (_value)))
	}
	
	pub fn from_str_must (_string : &str) -> Option<Self> {
		Self::from_str (_string) .or_panic (0xfe4be058)
	}
	
	pub fn from_f32_must (_value : f32) -> Option<Self> {
		Self::from_f32 (_value) .or_panic (0x004c83da)
	}
}




pub struct RouteSitemapEntry {
	pub included : Option<bool>,
	pub frequency : Option<SitemapFrequency>,
	pub priority : Option<SitemapPriority>,
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
	prefix : String,
	format : SitemapFormat,
	routes : Option<hss::Routes>,
}


impl RoutesSitemapResource {
	
	pub fn new (_prefix : String, _format : SitemapFormat) -> Self {
		Self::new_with_routes (_prefix, _format, None)
	}
	
	#[ cfg (feature = "runtime-sitemaps-xml") ]
	pub fn new_xml (_prefix : String) -> Self {
		Self::new (_prefix, SitemapFormat::Xml)
	}
	
	pub fn new_text (_prefix : String) -> Self {
		Self::new (_prefix, SitemapFormat::Text)
	}
	
	pub fn new_with_routes (_prefix : String, _format : SitemapFormat, _routes : Option<hss::Routes>) -> Self {
		Self {
				prefix : _prefix,
				format : _format,
				routes : _routes,
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
			
			#[ cfg (feature = "runtime-sitemaps-xml") ]
			SitemapFormat::Xml => {
				let mut _entries = Vec::with_capacity (_sitemap_routes.len ());
				let mut _url_buffer = String::with_capacity (128);
				for (_route, _route_entry) in _sitemap_routes {
					_url_buffer.clear ();
					_url_buffer.push_str (self.prefix.trim_end_matches ("/"));
					_url_buffer.push_str ("/");
					_url_buffer.push_str (_route.path.trim_start_matches ("/"));
					let _url = _url_buffer.parse () .or_wrap (0x270667a5) ?;
					let mut _builder = ::sitewriter::UrlEntryBuilder::default ();
					_builder.loc (_url);
					if let Some (_frequency) = _route_entry.frequency.as_ref () {
						let _frequency = match _frequency {
							SitemapFrequency::Always => ::sitewriter::ChangeFreq::Always,
							SitemapFrequency::Hourly => ::sitewriter::ChangeFreq::Hourly,
							SitemapFrequency::Daily => ::sitewriter::ChangeFreq::Daily,
							SitemapFrequency::Weekly => ::sitewriter::ChangeFreq::Weekly,
							SitemapFrequency::Monthly => ::sitewriter::ChangeFreq::Monthly,
							SitemapFrequency::Yearly => ::sitewriter::ChangeFreq::Yearly,
							SitemapFrequency::Never => ::sitewriter::ChangeFreq::Never,
						};
						_builder.changefreq (_frequency);
					}
					if let Some (_priority) = _route_entry.priority.as_ref () {
						_builder.priority (_priority.to_f32 ());
					}
					let _entry = _builder.build () .or_wrap (0x155cdb8f) ?;
					_entries.push (_entry);
				}
				let _buffer = ::sitewriter::generate_str (&_entries);
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


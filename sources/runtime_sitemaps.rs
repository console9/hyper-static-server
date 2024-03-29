

use crate::hss;


use ::std::{
		
		str::FromStr as _,
		
	};


use crate::hss::{
		
		ResponseExt as _,
		
		ResultExtWrap as _,
		ResultExtPanic as _,
		
	};


use crate::runtime::{
		
		StaticResource,
		
	};


use crate::errors::*;




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


pub struct SitemapUpdated (u64);




impl SitemapFormat {
	
	pub fn from_str (_string : &str) -> SitemapResult<Self> {
		match _string {
			#[ cfg (feature = "runtime-sitemaps-xml") ]
			"xml" => Ok (Self::Xml),
			"text" => Ok (Self::Text),
			_ => fail! (0xdf9d3bb7),
		}
	}
	
	pub fn from_str_must (_string : &str) -> Self {
		Self::from_str (_string) .else_panic (0x685370b7)
	}
}




impl SitemapFrequency {
	
	pub fn from_str (_string : &str) -> SitemapResult<Option<Self>> {
		match _string {
			"default" => Ok (None),
			"always" => Ok (Some (Self::Always)),
			"hourly" => Ok (Some (Self::Hourly)),
			"daily" => Ok (Some (Self::Daily)),
			"weekly" => Ok (Some (Self::Weekly)),
			"monthly" => Ok (Some (Self::Monthly)),
			"yearly" => Ok (Some (Self::Yearly)),
			"never" => Ok (Some (Self::Never)),
			_ => fail! (0xb2001a05),
		}
	}
	
	pub fn from_str_must (_string : &str) -> Option<Self> {
		Self::from_str (_string) .else_panic (0x256f6fc2)
	}
}


impl SitemapPriority {
	
	pub fn to_f32 (&self) -> f32 {
		self.0
	}
	
	pub fn from_str (_string : &str) -> SitemapResult<Option<Self>> {
		match _string {
			"default" =>
				Ok (None),
			_ =>
				Self::from_f32 (f32::from_str (_string) .else_wrap (0x1dd1e4e7) ?)
		}
	}
	
	pub fn from_f32 (_value : f32) -> SitemapResult<Option<Self>> {
		if _value < 0.0 {
			fail! (0x55b5a84b);
		}
		if _value > 1.0 {
			fail! (0xa233806b);
		}
		Ok (Some (Self (_value)))
	}
	
	pub fn from_str_must (_string : &str) -> Option<Self> {
		Self::from_str (_string) .else_panic (0xfe4be058)
	}
	
	pub fn from_f32_must (_value : f32) -> Option<Self> {
		Self::from_f32 (_value) .else_panic (0x004c83da)
	}
}


impl SitemapUpdated {
	
	pub fn to_u64 (&self) -> u64 {
		self.0
	}
	
	pub fn from_str (_string : &str) -> SitemapResult<Option<Self>> {
		match _string {
			"default" =>
				Ok (None),
			"now" => {
					let _date = ::chrono::Utc::now ();
					Self::from_u64 (_date.timestamp () as u64)
				}
			"today" => {
					let _date = ::chrono::Utc::now () .date () .and_hms (0, 0, 0);
					Self::from_u64 (_date.timestamp () as u64)
				}
			_ => {
					let _date = ::chrono::NaiveDate::parse_from_str (_string, "%Y-%m-%d") .else_wrap (0x9acb71c0) ?;
					let _date = ::chrono::DateTime::<::chrono::Utc>::from_utc (_date.and_hms (0, 0, 0), ::chrono::Utc);
					Self::from_u64 (_date.timestamp () as u64)
				}
		}
	}
	
	pub fn from_u64 (_value : u64) -> SitemapResult<Option<Self>> {
		if _value > 4294967296 {
			fail! (0x47d27218);
		}
		Ok (Some (Self (_value)))
	}
	
	pub fn from_str_must (_string : &str) -> Option<Self> {
		Self::from_str (_string) .else_panic (0xe56c6f8f)
	}
	
	pub fn from_u64_must (_value : u64) -> Option<Self> {
		Self::from_u64 (_value) .else_panic (0x93a18a2e)
	}
}




pub struct RouteSitemapEntry {
	pub included : Option<bool>,
	pub frequency : Option<SitemapFrequency>,
	pub priority : Option<SitemapPriority>,
	pub updated : Option<SitemapUpdated>,
}


impl RouteSitemapEntry {
	
	pub fn new () -> Self {
		Self {
				included : None,
				frequency : None,
				priority : None,
				updated : None,
			}
	}
}




pub struct RoutesSitemapResource {
	prefix : String,
	format : SitemapFormat,
	routes : Option<hss::Routes>,
}


impl RoutesSitemapResource {
	
	pub fn new (_prefix : String, _format : SitemapFormat) -> SitemapResult<Self> {
		Self::new_with_routes (_prefix, _format, None)
	}
	
	#[ cfg (feature = "runtime-sitemaps-xml") ]
	pub fn new_xml (_prefix : String) -> SitemapResult<Self> {
		Self::new (_prefix, SitemapFormat::Xml)
	}
	
	pub fn new_text (_prefix : String) -> SitemapResult<Self> {
		Self::new (_prefix, SitemapFormat::Text)
	}
	
	pub fn new_with_routes (_prefix : String, _format : SitemapFormat, _routes : Option<hss::Routes>) -> SitemapResult<Self> {
		let _self = Self {
				prefix : _prefix,
				format : _format,
				routes : _routes,
			};
		Ok (_self)
	}
	
	pub fn into_handler (self) -> impl hss::Handler {
		hss::HandlerSimpleSyncWrapper::new (self)
	}
}


impl StaticResource for RoutesSitemapResource {
	
	fn content_type (&self) -> hss::ContentType {
		match self.format {
			#[ cfg (feature = "runtime-sitemaps-xml") ]
			SitemapFormat::Xml =>
				hss::ContentType::Xml,
			SitemapFormat::Text =>
				hss::ContentType::Text,
		}
	}
	
	fn description (&self) -> &'static str {
		match self.format {
			#[ cfg (feature = "runtime-sitemaps-xml") ]
			SitemapFormat::Xml =>
				"sitemap (xml)",
			SitemapFormat::Text =>
				"sitemap (text)",
		}
	}
	
	fn into_handler_dyn (self) -> hss::HandlerDynArc {
		let _handler = self.into_handler ();
		let _handler = hss::HandlerDynArc::new (_handler);
		_handler
	}
}


impl hss::HandlerSimpleSync for RoutesSitemapResource {
	
	fn handle (&self, _request : &hss::Request<hss::Body>, _response : &mut hss::Response<hss::Body>) -> HandlerResult {
		
		let _routes = if let Some (_routes) = &self.routes {
			_routes.clone ()
		} else if let Some (_route_matched) = _request.extensions () .get::<hss::RouteMatched> () {
			_route_matched.routes.clone ()
		} else if let Some (_routes) = _request.extensions () .get::<hss::Routes> () {
			_routes.clone ()
		} else {
			fail! (0xee535008);
		};
		
		let mut _sitemap_routes = Vec::new ();
		let mut _url_buffer = String::with_capacity (128);
		for _route in _routes.routes () {
			if let Some (_entry) = _route.extensions.get::<RouteSitemapEntry> () {
				if _entry.included.unwrap_or (true) {
					_url_buffer.clear ();
					_url_buffer.push_str (self.prefix.trim_end_matches ("/"));
					_url_buffer.push_str ("/");
					_url_buffer.push_str (_route.path.trim_start_matches ("/"));
					let _url = ::url::Url::from_str (&_url_buffer) .else_wrap (0x270667a5) ?;
					_sitemap_routes.push ((_url, _entry));
				}
			}
		}
		drop (_url_buffer);
		
		let (_body, _content_type) = match self.format {
			
			#[ cfg (feature = "runtime-sitemaps-xml") ]
			SitemapFormat::Xml => {
				let mut _entries = Vec::with_capacity (_sitemap_routes.len ());
				for (_route_url, _route_entry) in _sitemap_routes {
					let mut _builder = ::sitewriter::UrlEntryBuilder::default ();
					_builder.loc (_route_url);
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
					if let Some (_updated) = _route_entry.updated.as_ref () {
						use ::chrono::TimeZone as _;
						_builder.lastmod (::chrono::Utc.timestamp (_updated.to_u64 () as i64, 0));
					}
					let _entry = _builder.build () .else_wrap (0x155cdb8f) ?;
					_entries.push (_entry);
				}
				let _buffer = ::sitewriter::generate_str (&_entries);
				(_buffer, hss::ContentType::Xml)
			}
			
			SitemapFormat::Text => {
				let mut _buffer = String::with_capacity (16 * 1024);
				for (_route_url, _route_entry) in _sitemap_routes {
					_buffer.push_str (_route_url.as_str ());
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


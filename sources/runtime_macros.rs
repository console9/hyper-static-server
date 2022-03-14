
#![ no_implicit_prelude ]




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama {
	
	
	( $_resource_name : ident, $_template_name : ident, $_context_name : ty, $_content_type : tt, $_template_path : literal, $_description : literal ) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_template_name {
			pub context : $_context_name,
			pub __is_production : bool,
			pub __is_development : bool,
		}
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			template : $_template_name,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						template : $_template_name {
								context : (), // FIXME!
								__is_production : cfg! (feature = "production"),
								__is_development : cfg! (not (feature = "production")),
							},
					}
			}
			
			pub fn render (&self) -> $crate::hss::ServerResult<::std::string::String> {
				::askama::Template::render (&self.template)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x60beda55, _error)))
			}
			
			pub fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::hss::ServerResult {
				use $crate::hss::ResponseExt as _;
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				::std::result::Result::Ok (())
			}
		}
	};
}




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama_with_title_and_body {
	
	
	( $_resource_name : ident, $_template_name : ident, $_context_name : ty, $_content_type : tt, $_template_path : literal, $_title : literal, $_body_path : literal, $_description : literal ) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_template_name {
			pub context : $_context_name,
			pub title : &'static str,
			pub body : &'static str,
			pub __is_production : bool,
			pub __is_development : bool,
		}
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			template : $_template_name,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						template : $_template_name {
								context : (), // FIXME!
								title : $_title,
								body : ::std::include_str! ($_body_path),
								__is_production : cfg! (feature = "production"),
								__is_development : cfg! (not (feature = "production")),
							},
					}
			}
			
			pub fn render (&self) -> $crate::hss::ServerResult<::std::string::String> {
				::askama::Template::render (&self.template)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x28df3421, _error)))
			}
			
			pub fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::hss::ServerResult {
				use $crate::hss::ResponseExt as _;
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				::std::result::Result::Ok (())
			}
		}
	};
}




#[ macro_export ]
macro_rules! resource {
	
	
	( $_resource_name : ident, $_content_type : tt, auto, $_resource_path : tt, $_description : literal ) => {
		#[ cfg (not (feature = "production")) ]
		$crate::resource! ($_resource_name, $_content_type, dynamic, $_resource_path, $_description);
		#[ cfg (feature = "production") ]
		$crate::resource! ($_resource_name, $_content_type, embedded, $_resource_path, $_description);
	};
	
	
	( $_resource_name : ident, $_content_type : tt, embedded, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name ();
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self ()
			}
			
			pub fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				self
			}
			
			pub const RESOURCE : $crate::hss::EmbeddedResource =
					$crate::hss::EmbeddedResource::new_const (
							::std::include_bytes! ($crate::resource_path! ($_resource_path)),
							::std::option::Option::Some ($crate::resource_content_type! ($_content_type)),
						);
		}
		
		impl $crate::hss::Handler for $_resource_name {
			
			type Future = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::Future;
			type ResponseBody = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::hss::Request<$crate::hss::Body>) -> Self::Future {
				Self::RESOURCE.handle (_request)
			}
		}
	};
	
	
	( $_resource_name : ident, $_content_type : tt, dynamic, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			resource : $crate::hss::FileResource,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						resource : $crate::hss::FileResource::new (
								$crate::resource_path! ($_resource_path),
								::std::option::Option::Some ($crate::resource_content_type! ($_content_type)),
								false,
							)
					}
			}
			
			pub fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				self
			}
		}
		
		impl $crate::hss::Handler for $_resource_name {
			
			type Future = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::Future;
			type ResponseBody = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::hss::Request<$crate::hss::Body>) -> Self::Future {
				self.resource.handle (_request)
			}
		}
	};
}




#[ macro_export ]
macro_rules! resource_content_type {
	
	( text ) => { $crate::hss::ContentType::Text };
	( html ) => { $crate::hss::ContentType::Html };
	( css ) => { $crate::hss::ContentType::Css };
	( js ) => { $crate::hss::ContentType::Js };
	
	( json ) => { $crate::hss::ContentType::Json };
	( xml ) => { $crate::hss::ContentType::Xml };
	
	( png ) => { $crate::hss::ContentType::Png };
	( jpeg ) => { $crate::hss::ContentType::Jpeg };
	( svg ) => { $crate::hss::ContentType::Svg };
	( icon ) => { $crate::hss::ContentType::Icon };
	
	( font_ttf ) => { $crate::hss::ContentType::FontTtf };
	( font_otf ) => { $crate::hss::ContentType::FontOtf };
	( font_woff ) => { $crate::hss::ContentType::FontWoff };
	( font_woff2 ) => { $crate::hss::ContentType::FontWoff2 };
	
	( unknown ) => { $crate::hss::ContentType::Unknown };
	
}




#[ macro_export ]
macro_rules! route {
	
	
	( $_route_name : ident, $_resource_name : ty, $_route_path : literal, $_route_extensions : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_route_name ();
		
		impl $_route_name {
			
			pub fn new () -> $crate::hss::Route {
				use ::std::convert::From as _;
				let _resource = <$_resource_name>::new ();
				let _path = ::std::string::String::from ($_route_path);
				let _description = Description (_resource.description ());
				struct Description (&'static str);
				impl ::std::fmt::Debug for Description {
					fn fmt (&self, _formatter : &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
						_formatter.write_str (self.0)
					}
				}
				let _handler = $crate::hss::RouteHandler::HandlerDynArc ($crate::hss::HandlerDynArc::new (_resource.into_handler ()) .into_arc ());
				let mut _extensions = $crate::route_extensions! ($_route_extensions);
				if _extensions.get::<$crate::RouteDebug> () .is_none () {
					_extensions.insert ($crate::RouteDebug::new (_description));
				}
				let mut _route = $crate::hss::Route {
						path : _path,
						handler : _handler,
						extensions : _extensions,
					};
				_route
			}
		}
	};
}




#[ cfg (feature = "runtime-sitemaps") ]
#[ macro_export ]
macro_rules! route_sitemap {
	
	
	( $_route_name : ident, $_route_path : literal, $_prefix : literal, $_format : ident, $_route_extensions : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_route_name ();
		
		impl $_route_name {
			
			pub fn new () -> $crate::hss::Route {
				use ::std::convert::From as _;
				use $crate::hss::HandlerSimpleSync as _;
				let _prefix = ::std::string::String::from ($_prefix);
				let _format = $crate::SitemapFormat::from_str_must (::std::stringify! ($_format));
				let _resource = $crate::RoutesSitemapResource::new (_prefix, _format);
				let _path = ::std::string::String::from ($_route_path);
				let _description = Description ();
				struct Description ();
				impl ::std::fmt::Debug for Description {
					fn fmt (&self, _formatter : &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
						_formatter.write_fmt (::std::format_args! ("sitemap ({})", ::std::stringify! ($_format)))
					}
				}
				let _handler = $crate::hss::RouteHandler::HandlerDynArc ($crate::hss::HandlerDynArc::new (_resource.wrap ()) .into_arc ());
				let mut _extensions = $crate::route_extensions! ($_route_extensions);
				if _extensions.get::<$crate::RouteDebug> () .is_none () {
					_extensions.insert ($crate::RouteDebug::new (_description));
				}
				let mut _route = $crate::hss::Route {
						path : _path,
						handler : _handler,
						extensions : _extensions,
					};
				_route
			}
		}
	};
}




#[ macro_export ]
macro_rules! route_extensions {
	
	( $_extensions : tt ) => {
		{
			let mut _extensions = $crate::hss::Extensions::new ();
			$crate::route_extensions_insert! (_extensions, $_extensions);
			_extensions
		}
	};
}


#[ macro_export ]
macro_rules! route_extensions_insert {
	
	( $_extensions : ident, () ) => {
	};
	
	( $_extensions : ident, {} ) => {
	};
	
	( $_extensions : ident, { $_key : ident $(, $( $_rest : tt )* )? } ) => {
		$crate::route_extensions_insert_one! ($_extensions, $_key);
		$crate::route_extensions_insert! ($_extensions, { $( $( $_rest )* )? });
	};
	
	( $_extensions : ident, { $_key : ident : $_value : tt $(, $( $_rest : tt )* )? } ) => {
		$crate::route_extensions_insert_one! ($_extensions, $_key, $_value);
		$crate::route_extensions_insert! ($_extensions, { $( $( $_rest )* )? });
	};
}


#[ macro_export ]
macro_rules! route_extensions_insert_one {
	
	( $_extensions : ident, debug, $_debug : expr ) => {
		$_extensions.insert ($crate::RouteDebug::new ($_debug));
	};
	
	( $_extensions : ident, sitemap ) => {
		$_extensions.insert ($crate::RouteSitemapEntry::new ());
	};
	( $_extensions : ident, sitemap, { frequency : $_frequency : ident } ) => {
		{
			let mut _entry = $crate::RouteSitemapEntry::new ();
			_entry.frequency = $crate::SitemapFrequency::from_str_must (::std::stringify! ($_frequency));
			$_extensions.insert (_entry);
		}
	};
	( $_extensions : ident, sitemap, { frequency : $_frequency : ident, priority : $_priority : ident } ) => {
		{
			let mut _entry = $crate::RouteSitemapEntry::new ();
			_entry.frequency = $crate::SitemapFrequency::from_str_must (::std::stringify! ($_frequency));
			_entry.priority = $crate::SitemapPriority::from_str_must (::std::stringify! ($_priority));
			$_extensions.insert (_entry);
		}
	};
	( $_extensions : ident, sitemap, { frequency : $_frequency : ident, priority : $_priority : literal } ) => {
		{
			let mut _entry = $crate::RouteSitemapEntry::new ();
			_entry.frequency = $crate::SitemapFrequency::from_str_must (::std::stringify! ($_frequency));
			_entry.priority = $crate::SitemapPriority::from_str_must (::std::stringify! ($_priority));
			$_extensions.insert (_entry);
		}
	};
}




#[ macro_export ]
macro_rules! routes {
	
	
	( $_name : ident, [ $( $_route : ty, )* ] ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_name ();
		
		impl $_name {
			
			pub fn new () -> $crate::hss::Routes {
				use ::std::iter::IntoIterator as _;
				use $crate::hss::ResultExtPanic as _;
				let mut _routes = $crate::hss::RoutesBuilder::new ();
				for _route in Self::routes () .into_iter () {
					_routes = _routes.with_route_object (_route);
				}
				let _routes = _routes.build () .or_panic (0x630a415a);
				_routes
			}
		}
		
		impl $_name {
			
			pub fn routes () -> ::std::vec::Vec<$crate::hss::Route> {
				::std::vec! (
					$( <$_route>::new (), )*
				)
			}
		}
		
		impl $_name {
			
			pub fn eprintln () -> () {
				use ::std::iter::IntoIterator as _;
				for _route in Self::routes () .into_iter () {
					if let ::std::option::Option::Some (_debug) = _route.extensions.get::<$crate::RouteDebug> () {
						::std::eprintln! ("[dd] [825798f8]  **  {} -> {:?}", _route.path, _debug);
					} else {
						::std::eprintln! ("[dd] [df531ca1]  **  {}", _route.path);
					}
				}
			}
		}
	};
}




#[ macro_export ]
macro_rules! dependencies {
	
	
	( $_name : ident, [ $( $_dependency : literal, )* ] ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_name ();
		
		impl $_name {
			
			pub fn dependencies () -> ::std::vec::Vec<&'static ::std::path::Path> {
				::std::vec! (
					$( ::std::path::Path::new ($_dependency), )*
				)
			}
		}
		
		impl $_name {
			
			pub fn eprintln () -> () {
				use ::std::iter::IntoIterator as _;
				for _dependency in Self::dependencies () .into_iter () {
					::std::eprintln! ("[dd] [402419e4]  !!  {}", _dependency.display ());
				}
			}
		}
	};
}




#[ macro_export ]
macro_rules! builder_generated {
	
	
	() => {
		::std::include! (::std::concat! (::std::env! ("OUT_DIR"), "/hss-builder-generated-default.in"));
	};
}




#[ macro_export ]
macro_rules! resource_path {
	
	
	( ( relative_to_crate, $_path : literal ) ) => {
		::std::concat! (::std::env! ("CARGO_MANIFEST_DIR"), "/", $_path)
	};
	
	
	( ( relative_to_cwd, $_path : literal ) ) => {
		$_path
	};
	
	
	( $_path : literal ) => {
		$crate::resource_path! ( ( relative_to_crate, $_path ) )
	};
}


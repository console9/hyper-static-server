
#![ no_implicit_prelude ]




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama {
	
	
	(
			$_resource_name : ident,
			$_template_name : ident,
			$_context_descriptor : tt,
			$_content_type : tt,
			$_template_path : literal,
			$_description : literal
	) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_template_name {
			pub context : $crate::askama_context_type! ($_context_descriptor),
			pub __is_production : bool,
			pub __is_development : bool,
		}
		
		$crate::cfg_builder_askama_dynamic_disabled! {
			#[ allow (non_camel_case_types) ]
			pub(crate) struct $_resource_name {
				template : $_template_name,
			}
		}
		
		$crate::cfg_builder_askama_dynamic_enabled! {
			#[ allow (non_camel_case_types) ]
			pub(crate) struct $_resource_name {}
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _self = Self {
							template : Self::template_build (_extensions) ?,
						};
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _self = Self {};
				}
				$crate::hss::ServerResult::Ok (_self)
			}
			
			pub fn render (&self) -> $crate::hss::ServerResult<::std::string::String> {
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _template = &self.template;
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					// FIXME:  Somehow use the extensions from the resource constructor!
					let _template = Self::template_build (&$crate::hss::Extensions::new ()) ?;
					let _template = &_template;
				}
				::askama::Template::render (_template)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x60beda55, _error)))
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
			
			fn template_build (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<$_template_name> {
				let _context = $crate::askama_context_new! ($_context_descriptor, _extensions) ?;
				let _template = $_template_name {
						context : _context,
						__is_production : $crate::cfg_if_production! ({ true } | { false }),
						__is_development : $crate::cfg_if_production! ({ false } | { true }),
					};
				$crate::hss::ServerResult::Ok (_template)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::hss::ServerResult {
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::hss::ServerResult::Ok (())
			}
		}
	};
}




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama_document {
	
	
	(
			$_resource_name : ident,
			$_template_name : ident,
			$_context_descriptor : tt,
			$_content_type : tt,
			$_template_path : literal,
			$_body_path : literal,
			$_title_path : literal,
			$_metadata_path : literal,
			$( $_refresher_name : ident, )?
			$_description : literal
	) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_template_name {
			pub context : $crate::askama_context_type! ($_context_descriptor),
			pub body : ::std::string::String,
			pub title : ::std::string::String,
			pub metadata : $crate::AskamaDocumentMetadata,
			pub __is_production : bool,
			pub __is_development : bool,
		}
		
		$crate::cfg_builder_askama_dynamic_disabled! {
			#[ allow (non_camel_case_types) ]
			pub(crate) struct $_resource_name {
				template : $_template_name,
			}
		}
		
		$crate::cfg_builder_askama_dynamic_enabled! {
			#[ allow (non_camel_case_types) ]
			pub(crate) struct $_resource_name {}
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _self = Self {
							template : Self::template_build (_extensions) ?,
						};
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _self = Self {};
				}
				$crate::hss::ServerResult::Ok (_self)
			}
			
			pub fn render (&self) -> $crate::hss::ServerResult<::std::string::String> {
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _template = &self.template;
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					// FIXME:  Somehow use the extensions from the resource constructor!
					let _template = Self::template_build (&$crate::hss::Extensions::new ()) ?;
					let _template = &_template;
				}
				::askama::Template::render (_template)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x28df3421, _error)))
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
			
			fn template_build (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<$_template_name> {
				use ::std::convert::From as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					$(
						::std::compile_error ("`refresher` not supported without dynamic feature!");
						type _refresher_type = $_refresher_name;
					)?
					let _body = ::std::string::String::from (::std::include_str! ($_body_path));
					let _title = ::std::string::String::from (::std::include_str! ($_title_path));
					let _metadata = ::std::include_str! ($_metadata_path);
					let _metadata = $crate::AskamaDocumentMetadata::load_from_json (_metadata) ?;
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					$( $_refresher_name::refresh () ?; )?
					use $crate::hss::ResultExtWrap as _;
					let _body = ::std::fs::read_to_string ($_body_path) .or_wrap (0x222c7659) ?;
					let _title = ::std::fs::read_to_string ($_title_path) .or_wrap (0x32c4e114) ?;
					let _metadata = ::std::fs::read_to_string ($_metadata_path) .or_wrap (0xc07d6b78) ?;
					let _metadata = $crate::AskamaDocumentMetadata::load_from_json (&_metadata) ?;
				}
				let _context = $crate::askama_context_new! ($_context_descriptor, _extensions) ?;
				let _template = $_template_name {
						context : _context,
						body : _body,
						title : _title,
						metadata : _metadata,
						__is_production : $crate::cfg_if_production! ({ true } | { false }),
						__is_development : $crate::cfg_if_production! ({ false } | { true }),
					};
				$crate::hss::ServerResult::Ok (_template)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::hss::ServerResult {
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::hss::ServerResult::Ok (())
			}
		}
	};
}




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama_context_type {
	
	( $_context_type : ty ) => {
		$crate::askama_context_type! ({ type : $_context_type })
	};
	
	( { type : $_context_type : ty $( , $( $_ : tt )+ )* } ) => {
		$_context_type
	};
}


#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama_context_new {
	
	( $_context_type : ty, $_extensions : expr ) => {
		$crate::askama_context_new! ({ type : $_context_type }, $_extensions)
	};
	
	( { type : $_context_type : ty }, $_extensions : expr ) => {
		{
			let _extensions : &$crate::hss::Extensions = $_extensions;
			<$_context_type as $crate::AskamaContext>::new_with_extensions (_extensions)
		}
	};
	
	( { type : $_context_type : ty, json : $_context_path : literal }, $_extensions : expr) => {
		$crate::askama_context_new! ({ type : $_context_type, deserialize : ("json", $_context_path) }, $_extensions)
	};
	( { type : $_context_type : ty, toml : $_context_path : literal }, $_extensions : expr) => {
		$crate::askama_context_new! ({ type : $_context_type, deserialize : ("toml", $_context_path) }, $_extensions)
	};
	( { type : $_context_type : ty, yaml : $_context_path : literal }, $_extensions : expr) => {
		$crate::askama_context_new! ({ type : $_context_type, deserialize : ("yaml", $_context_path) }, $_extensions)
	};
	
	( { type : $_context_type : ty, deserialize : ( $_context_encoding : literal, $_context_path : literal ) }, $_extensions : expr ) => {
		{
			$crate::cfg_builder_askama_dynamic_disabled! {
				let _context = $crate::askama_context_new! ({ type : $_context_type, (deserialize, embedded) : ($_context_encoding, $_context_path)}, $_extensions);
			}
			$crate::cfg_builder_askama_dynamic_enabled! {
				let _context = $crate::askama_context_new! ({ type : $_context_type, (deserialize, dynamic) : ($_context_encoding, $_context_path)}, $_extensions);
			}
			_context
		}
	};
	
	( { type : $_context_type : ty, (deserialize, embedded) : ( $_context_encoding : literal, $_context_path : literal ) }, $_extensions : expr ) => {
		{
			let _encoding : &str = $_context_encoding;
			let _data : &[u8] = ::std::include_bytes! ($_context_path);
			let _extensions : &$crate::hss::Extensions = $_extensions;
			<$_context_type as $crate::AskamaContext>::new_with_deserialization (_encoding, _data, _extensions)
		}
	};
	
	( { type : $_context_type : ty, (deserialize, dynamic) : ( $_context_encoding : literal, $_context_path : literal ) }, $_extensions : expr ) => {
		{
			use $crate::hss::ResultExtWrap as _;
			let _encoding : &str = $_context_encoding;
			let _data = ::std::fs::read ($_context_path) .or_wrap (0x98ea260c) ?;
			let _extensions : &$crate::hss::Extensions = $_extensions;
			<$_context_type as $crate::AskamaContext>::new_with_deserialization (_encoding, &_data, _extensions)
		}
	};
}




#[ macro_export ]
macro_rules! resource {
	
	
	( $_resource_name : ident, $_content_type : tt, auto, $_resource_path : tt, $_description : literal ) => {
		$crate::cfg_if_production! {{
			$crate::resource! ($_resource_name, $_content_type, embedded, $_resource_path, $_description);
		} | {
			$crate::resource! ($_resource_name, $_content_type, dynamic, $_resource_path, $_description);
		}}
	};
	
	
	( $_resource_name : ident, $_content_type : tt, embedded, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name ();
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				let _self = Self ();
				$crate::hss::ServerResult::Ok (_self)
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				self
			}
			
			const RESOURCE : $crate::hss::EmbeddedResource =
					$crate::hss::EmbeddedResource::new_const (
							::std::include_bytes! ($crate::resource_path! ($_resource_path)),
							::std::option::Option::Some ($crate::resource_content_type! ($_content_type)),
						);
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
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
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				let _self = Self {
						resource : $crate::hss::FileResource::new (
								$crate::resource_path! ($_resource_path),
								::std::option::Option::Some ($crate::resource_content_type! ($_content_type)),
								false,
							)
					};
				$crate::hss::ServerResult::Ok (_self)
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				self
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::Handler for $_resource_name {
			
			type Future = <$crate::hss::FileResource as $crate::hss::Handler>::Future;
			type ResponseBody = <$crate::hss::FileResource as $crate::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::hss::FileResource as $crate::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::hss::Request<$crate::hss::Body>) -> Self::Future {
				self.resource.handle (_request)
			}
		}
	};
}




#[ macro_export ]
#[ cfg (all (feature = "builder-assets-sass-dynamic", not (feature = "production"))) ]
macro_rules! resource_sass_dynamic {
	
	( $_resource_name : ident, $_content_type : tt, $_source_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			source : &'static ::std::path::Path,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				let _self = Self {
						source : ::std::path::Path::new ($crate::resource_path! ($_source_path)),
					};
				$crate::hss::ServerResult::Ok (_self)
			}
			
			pub fn render (&self) -> $crate::hss::ServerResult<::std::string::String> {
				$crate::support_sass::compile_sass (self.source)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x548c29d4, _error)))
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::hss::ServerResult {
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::hss::ServerResult::Ok (())
			}
		}
	};
}




#[ macro_export ]
#[ cfg (all (feature = "builder-markdown-dynamic", not (feature = "production"))) ]
macro_rules! resource_markdown_dynamic {
	
	( $_resource_name : ident, $_content_type : tt, $_source_path : tt, $_header_path : tt, $_footer_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			source : &'static ::std::path::Path,
			header : ::std::option::Option<&'static ::std::path::Path>,
			footer : ::std::option::Option<&'static ::std::path::Path>,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				let _self = Self {
						source : ::std::path::Path::new ($crate::resource_path! ($_source_path)),
						header : $crate::resource_path! ($_header_path) .map (::std::path::Path::new::<str>),
						footer : $crate::resource_path! ($_footer_path) .map (::std::path::Path::new::<str>),
					};
				$crate::hss::ServerResult::Ok (_self)
			}
			
			pub fn render (&self) -> $crate::hss::ServerResult<::std::string::String> {
				$crate::support_markdown::compile_markdown_html_from_path (self.source, self.header, self.footer, ::std::option::Option::None)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x35c91763, _error)))
			}
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::hss::ServerResult {
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::hss::ServerResult::Ok (())
			}
		}
	};
}




#[ macro_export ]
#[ cfg (all (feature = "builder-markdown-dynamic", not (feature = "production"))) ]
macro_rules! resource_markdown_refresher {
	
	( $_refresher_name : ident, $_source_path : tt, $_body_path : tt, $_title_path : tt, $_metadata_path : tt, $_frontmatter_path : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_refresher_name {}
		
		#[ allow (dead_code) ]
		impl $_refresher_name {
			
			fn refresh () -> $crate::hss::ServerResult {
				$crate::support_markdown::compile_markdown_from_path_to_paths (
						::std::path::Path::new ($crate::resource_path! ($_source_path)),
						::std::option::Option::None,
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_body_path))),
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_title_path))),
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_metadata_path))),
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_frontmatter_path))),
					)
					.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0xec077645, _error)))
			}
		}
	};
}




#[ macro_export ]
macro_rules! route {
	
	
	( $_route_name : ident, $_resource_name : ty, $_route_path : literal, $_route_extensions : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_route_name ($crate::hss::Route);
		
		impl $_route_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				use ::std::convert::From as _;
				use $crate::StaticResource as _;
				let _resource = <$_resource_name>::new (_extensions) ?;
				// let _ : &dyn $crate::StaticResource = &_resource;
				let _path = ::std::string::String::from ($_route_path);
				let _description = _resource.description ();
				let _handler = $crate::hss::RouteHandler::HandlerDynArc (_resource.into_handler_dyn () .into_arc ());
				let mut _extensions = $crate::route_extensions! ($_route_extensions);
				if _extensions.get::<$crate::StaticRouteDebug> () .is_none () {
					_extensions.insert ($crate::StaticRouteDebug::from_str_static (_description));
				}
				let mut _route = $crate::hss::Route {
						path : _path,
						handler : _handler,
						extensions : _extensions,
					};
				let _self = Self (_route);
				$crate::hss::ServerResult::Ok (_self)
			}
		}
		
		impl $crate::StaticRoute for $_route_name {
			
			fn into_route (self) -> $crate::hss::Route {
				self.0
			}
		}
		
		impl ::std::convert::Into<$crate::hss::Route> for $_route_name {
			
			fn into (self) -> $crate::hss::Route {
				use $crate::StaticRoute as _;
				self.into_route ()
			}
		}
	};
}




#[ cfg (feature = "runtime-sitemaps") ]
#[ macro_export ]
macro_rules! route_sitemap {
	
	
	( $_route_name : ident, $_route_path : literal, $_prefix : literal, $_format : ident, $_route_extensions : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_route_name ($crate::hss::Route);
		
		impl $_route_name {
			
			pub fn new (_extensions : &$crate::hss::Extensions) -> $crate::hss::ServerResult<Self> {
				use ::std::convert::From as _;
				use $crate::StaticResource as _;
				let _prefix = ::std::string::String::from ($_prefix);
				let _format = $crate::SitemapFormat::from_str_must (::std::stringify! ($_format));
				let _resource = $crate::RoutesSitemapResource::new (_prefix, _format, ::std::option::Option::Some (_extensions)) ?;
				// let _ : &dyn $crate::StaticResource = &_resource;
				let _path = ::std::string::String::from ($_route_path);
				let _description = _resource.description ();
				let _handler = $crate::hss::RouteHandler::HandlerDynArc (_resource.into_handler_dyn () .into_arc ());
				let mut _extensions = $crate::route_extensions! ($_route_extensions);
				if _extensions.get::<$crate::StaticRouteDebug> () .is_none () {
					_extensions.insert ($crate::StaticRouteDebug::from_str_static (_description));
				}
				let mut _route = $crate::hss::Route {
						path : _path,
						handler : _handler,
						extensions : _extensions,
					};
				let _self = Self (_route);
				$crate::hss::ServerResult::Ok (_self)
			}
		}
		
		impl $crate::StaticRoute for $_route_name {
			
			fn into_route (self) -> $crate::hss::Route {
				self.0
			}
		}
		
		impl ::std::convert::Into<$crate::hss::Route> for $_route_name {
			
			fn into (self) -> $crate::hss::Route {
				use $crate::StaticRoute as _;
				self.into_route ()
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
	
	( $_extensions : ident, { $_key : ident $( , $( $_rest : tt )+ )* } ) => {
		$crate::route_extensions_insert_one! ($_extensions, $_key);
		$crate::route_extensions_insert! ($_extensions, { $( $( $_rest )* ),* });
	};
	
	( $_extensions : ident, { $_key : ident : $_value : tt $( , $( $_rest : tt )+ )* } ) => {
		$crate::route_extensions_insert_one! ($_extensions, $_key, $_value);
		$crate::route_extensions_insert! ($_extensions, { $( $( $_rest )* ),* });
	};
}


#[ macro_export ]
macro_rules! route_extensions_insert_one {
	
	( $_extensions : ident, debug, $_debug : expr ) => {
		$_extensions.insert ($crate::StaticRouteDebug::new ($_debug));
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
		pub(crate) struct $_name ($crate::hss::Routes);
		
		impl $_name {
			
			pub fn new () -> $crate::hss::ServerResult<Self> {
				Self::new_with_extensions (::std::option::Option::None)
			}
			
			pub fn new_with_extensions (_extensions : ::std::option::Option<&$crate::hss::Extensions>) -> $crate::hss::ServerResult<Self> {
				use ::std::iter::IntoIterator as _;
				let _routes = Self::routes_with_extensions (_extensions) ?;
				let mut _builder = $crate::hss::RoutesBuilder::new ();
				for _route in _routes.into_iter () {
					_builder = _builder.with_route_object (_route);
				}
				let _routes = _builder.build () ?;
				let _self = Self (_routes);
				$crate::hss::ServerResult::Ok (_self)
			}
		}
		
		impl $_name {
			
			pub fn routes () -> $crate::hss::ServerResult<::std::vec::Vec<$crate::hss::Route>> {
				Self::routes_with_extensions (::std::option::Option::None)
			}
			
			pub fn routes_with_extensions (_extensions : ::std::option::Option<&$crate::hss::Extensions>) -> $crate::hss::ServerResult<::std::vec::Vec<$crate::hss::Route>> {
				use $crate::StaticRoute as _;
				let _extensions_none = $crate::hss::Extensions::new ();
				let _extensions = _extensions.unwrap_or (&_extensions_none);
				let _routes = ::std::vec! (
						$(
							{
								let _route : $_route = <$_route>::new (_extensions) ?;
								// let _ : &dyn $crate::StaticRoute = &_route;
								let _route = _route.into_route ();
								_route
							},
						)*
					);
				$crate::hss::ServerResult::Ok (_routes)
			}
		}
		
		impl $crate::StaticRoutes for $_name {
			
			fn into_routes (self) -> $crate::hss::Routes {
				self.0
			}
		}
		
		impl ::std::convert::Into<$crate::hss::Routes> for $_name {
			
			fn into (self) -> $crate::hss::Routes {
				use $crate::StaticRoutes as _;
				self.into_routes ()
			}
		}
		
		impl $_name {
			
			pub fn eprintln () -> $crate::hss::ServerResult {
				use ::std::iter::IntoIterator as _;
				let _routes = Self::routes () ?;
				for _route in _routes.into_iter () {
					if let ::std::option::Option::Some (_debug) = _route.extensions.get::<$crate::StaticRouteDebug> () {
						::std::eprintln! ("[dd] [825798f8]  **  {} -> {:?}", _route.path, _debug);
					} else {
						::std::eprintln! ("[dd] [df531ca1]  **  {}", _route.path);
					}
				}
				$crate::hss::ServerResult::Ok (())
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
macro_rules! builder_generated {
	
	
	() => {
		::std::include! (::std::concat! (::std::env! ("OUT_DIR"), "/hss-builder-generated-default.in"));
	};
}




#[ macro_export ]
macro_rules! resource_path {
	
	
	( ( relative_to_crate, None ) ) => {
		::std::option::Option::None
	};
	( ( relative_to_cwd, None ) ) => {
		::std::option::Option::None
	};
	( None ) => {
		::std::option::Option::None
	};
	
	
	( ( relative_to_crate, Some ( $_path : literal ) ) ) => {
		::std::option::Option::Some ( $crate::resource_path! ( ( relative_to_crate, $_path ) ) )
	};
	( ( relative_to_cwd, Some ( $_path : literal ) ) ) => {
		::std::option::Option::Some ( $crate::resource_path! ( ( relative_to_cwd, $_path ) ) )
	};
	( Some ( $_path : literal ) ) => {
		::std::option::Option::Some ( $crate::resource_path! ( $_path ) )
	};
	
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




#[ macro_export ]
#[ cfg (feature = "production") ]
macro_rules! cfg_if_production {
	( { $( $_then_token : tt )* } | { $( $_else_token : tt )* } ) => { $( $_then_token )* };
	( { $( $_then_token : tt )* } ) => { $( $_then_token )* };
}

#[ macro_export ]
#[ cfg (not (feature = "production")) ]
macro_rules! cfg_if_production {
	( { $( $_then_token : tt )* } | { $( $_else_token : tt )* } ) => { $( $_else_token )* };
	( { $( $_then_token : tt )* } ) => {};
}




#[ macro_export ]
#[ cfg (any (not (feature = "builder-askama-dynamic"), feature = "production")) ]
macro_rules! cfg_builder_askama_dynamic_disabled {
	( $( $_token : tt )* ) => { $( $_token )* };
}

#[ macro_export ]
#[ cfg (all (feature = "builder-askama-dynamic", not (feature = "production"))) ]
macro_rules! cfg_builder_askama_dynamic_disabled {
	( $( $_token : tt )* ) => {};
}


#[ macro_export ]
#[ cfg (all (feature = "builder-askama-dynamic", not (feature = "production"))) ]
macro_rules! cfg_builder_askama_dynamic_enabled {
	( $( $_token : tt )* ) => {
		$( $_token )*
	};
}

#[ macro_export ]
#[ cfg (any (not (feature = "builder-askama-dynamic"), feature = "production")) ]
macro_rules! cfg_builder_askama_dynamic_enabled {
	( $( $_token : tt )* ) => {};
}


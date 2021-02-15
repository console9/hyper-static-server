



#[ macro_export ]
macro_rules! askama {
	
	
	( $_resource_name : ident, $_template_name : ident, $_content_type : ident, $_template_path : literal, $_description : literal ) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_template_name ();
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			template : $_template_name,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						template : $_template_name {},
					}
			}
			
			pub fn render (&self) -> $crate::ServerResult<::std::string::String> {
				::askama::Template::render (&self.template)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x60beda55, _error)))
			}
			
			pub fn content_type (&self) -> $crate::ContentType {
				$crate::ContentType::$_content_type
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::Handler {
				$crate::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::Request<$crate::Body>, _response : &mut $crate::Response<$crate::Body>) -> $crate::ServerResult {
				use $crate::ResponseExt as _;
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
	
	
	( $_resource_name : ident, $_content_type : ident, auto, $_resource_path : tt, $_description : literal ) => {
		#[ cfg (debug_assertions) ]
		$crate::resource! ($_resource_name, $_content_type, dynamic, $_resource_path, $_description);
		#[ cfg (not (debug_assertions)) ]
		$crate::resource! ($_resource_name, $_content_type, embedded, $_resource_path, $_description);
	};
	
	
	( $_resource_name : ident, $_content_type : ident, embedded, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name ();
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self ()
			}
			
			pub fn content_type (&self) -> $crate::ContentType {
				$crate::ContentType::$_content_type
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::Handler {
				self
			}
			
			pub const RESOURCE : $crate::EmbeddedResource =
					$crate::EmbeddedResource::new (
							::std::option::Option::Some ($crate::ContentType::$_content_type),
							::std::include_bytes! ($crate::resource_path! ($_resource_path)),
						);
		}
		
		impl $crate::Handler for $_resource_name {
			
			type Future = <$crate::EmbeddedResource as $crate::Handler>::Future;
			type ResponseBody = <$crate::EmbeddedResource as $crate::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::EmbeddedResource as $crate::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::Request<$crate::Body>) -> Self::Future {
				Self::RESOURCE.handle (_request)
			}
		}
	};
	
	
	( $_resource_name : ident, $_content_type : ident, dynamic, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			resource : $crate::FileResource,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						resource : $crate::FileResource::new (
								$crate::resource_path! ($_resource_path),
								::std::option::Option::Some ($crate::ContentType::$_content_type),
								false,
							)
					}
			}
			
			pub fn content_type (&self) -> $crate::ContentType {
				$crate::ContentType::$_content_type
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::Handler {
				self
			}
		}
		
		impl $crate::Handler for $_resource_name {
			
			type Future = <$crate::EmbeddedResource as $crate::Handler>::Future;
			type ResponseBody = <$crate::EmbeddedResource as $crate::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::EmbeddedResource as $crate::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::Request<$crate::Body>) -> Self::Future {
				self.resource.handle (_request)
			}
		}
	};
}




#[ macro_export ]
macro_rules! route {
	
	
	( $_route_name : ident, $_resource_name : ty, $_route_path : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_route_name ();
		
		impl $_route_name {
			
			pub fn new () -> $crate::Route {
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
				$crate::Route {
						path : _path,
						handler : $crate::HandlerDynArc::new (_resource.into_handler ()),
						debug : ::std::option::Option::Some (::std::boxed::Box::new (_description)),
					}
			}
		}
	};
}




#[ macro_export ]
macro_rules! routes {
	
	
	( $_name : ident, [ $( $_route : ty, )* ] ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_name ();
		
		impl $_name {
			
			pub fn new () -> $crate::Routes {
				use ::std::iter::IntoIterator as _;
				let mut _routes = $crate::RoutesBuilder::new ();
				for _route in Self::routes () .into_iter () {
					_routes = _routes.with_route_object (_route);
				}
				let _routes = _routes.build () .expect ("[630a415a]");
				_routes
			}
		}
		
		impl $_name {
			
			pub fn routes () -> ::std::vec::Vec<$crate::Route> {
				::std::vec! (
					$( <$_route>::new (), )*
				)
			}
		}
		
		impl $_name {
			
			pub fn eprintln_routes () -> () {
				use ::std::iter::IntoIterator as _;
				for _route in Self::routes () .into_iter () {
					if let ::std::option::Option::Some (_debug) = _route.debug.as_ref () {
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
			
			pub fn eprintln_dependencies () -> () {
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


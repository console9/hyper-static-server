

use crate::prelude::hss::ResponseExt as _;
use ::std::convert::From as _;




macro_rules! template {
	
	
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
			
			pub fn render (&self) -> $crate::prelude::ServerResult<::std::string::String> {
				::askama::Template::render (&self.template)
						.map_err (|_error| ::std::io::Error::new (::std::io::ErrorKind::Other, ::std::format! ("[{:08x}]  {}", 0x60beda55, _error)))
			}
			
			pub fn content_type (&self) -> $crate::prelude::hss::ContentType {
				$crate::prelude::hss::ContentType::$_content_type
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::prelude::hss::Handler {
				$crate::prelude::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::prelude::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::prelude::Request, _response : &mut $crate::prelude::Response) -> $crate::prelude::ServerResult {
				let _body = self.render () ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				::std::result::Result::Ok (())
			}
		}
	};
}




macro_rules! resource {
	
	
	( $_resource_name : ident, $_content_type : ident, embedded, $_resource_path : literal, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name ();
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self ()
			}
			
			pub fn content_type (&self) -> $crate::prelude::hss::ContentType {
				$crate::prelude::hss::ContentType::$_content_type
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::prelude::hss::Handler {
				self
			}
			
			pub const RESOURCE : $crate::prelude::hss::EmbeddedResource =
					$crate::prelude::hss::EmbeddedResource::new (
							::std::option::Option::Some ($crate::prelude::hss::ContentType::$_content_type),
							::std::include_bytes! ($_resource_path),
						);
		}
		
		impl $crate::prelude::hss::Handler for $_resource_name {
			
			type Future = <$crate::prelude::hss::EmbeddedResource as $crate::prelude::hss::Handler>::Future;
			type ResponseBody = <$crate::prelude::hss::EmbeddedResource as $crate::prelude::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::prelude::hss::EmbeddedResource as $crate::prelude::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::prelude::Request) -> Self::Future {
				Self::RESOURCE.handle (_request)
			}
		}
	};
	
	
	( $_resource_name : ident, $_content_type : ident, dynamic, $_resource_path : literal, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			resource : $crate::prelude::hss::FileResource,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						resource : $crate::prelude::hss::FileResource::new (
								$_resource_path,
								::std::option::Option::Some ($crate::prelude::hss::ContentType::$_content_type),
								false,
							)
					}
			}
			
			pub fn content_type (&self) -> $crate::prelude::hss::ContentType {
				$crate::prelude::hss::ContentType::$_content_type
			}
			
			pub fn description (&self) -> &'static str {
				$_description
			}
			
			pub fn into_handler (self) -> impl $crate::prelude::hss::Handler {
				self
			}
		}
		
		impl $crate::prelude::hss::Handler for $_resource_name {
			
			type Future = <$crate::prelude::hss::EmbeddedResource as $crate::prelude::hss::Handler>::Future;
			type ResponseBody = <$crate::prelude::hss::EmbeddedResource as $crate::prelude::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::prelude::hss::EmbeddedResource as $crate::prelude::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::prelude::Request) -> Self::Future {
				self.resource.handle (_request)
			}
		}
	};
}




macro_rules! route {
	
	
	( $_route_name : ident, $_resource_name : ty, $_route_path : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_route_name ();
		
		impl $_route_name {
			
			pub fn new () -> $crate::prelude::hss::Route {
				let _resource = <$_resource_name>::new ();
				let _path = ::std::string::String::from ($_route_path);
				let _description = ::std::string::String::from (_resource.description ());
				$crate::prelude::hss::Route {
						path : _path,
						handler : $crate::prelude::hss::HandlerDynArc::new (_resource.into_handler ()),
						debug : ::std::option::Option::Some (::std::boxed::Box::new (_description)),
					}
			}
		}
	};
}




macro_rules! routes {
	
	
	( $_name : ident, [ $( $_route : ty, )* ] ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_name ();
		
		impl $_name {
			
			pub fn new () -> $crate::prelude::hss::Routes {
				
				let mut _routes = $crate::prelude::hss::RoutesBuilder::new ();
				
				$( {
					let _route = <$_route>::new ();
					_routes = _routes.with_route_object (_route);
				} )*
				
				let _routes = _routes.build () .expect ("[630a415a]");
				
				_routes
			}
		}
	};
}




::std::include! ("./resources.in");


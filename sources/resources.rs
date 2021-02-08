

use crate::prelude::hss::ResponseExt as _;
use crate::prelude::hss::HandlerSimpleSync as _;
use ::std::convert::From as _;




macro_rules! template {
	( $_resource_name : ident, $_template_name : ident, $_content_type : ident, $_template_path : literal, $_description : literal ) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_template_name {}
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			template : $_template_name,
			content_type : $crate::prelude::hss::ContentType,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						template : $_template_name {},
						content_type : $crate::prelude::hss::ContentType::$_content_type,
					}
			}
			
			pub fn content_type (&self) -> $crate::prelude::hss::ContentType {
				self.content_type
			}
			
			fn description (&self) -> &str {
				$_description
			}
		}
		
		impl $_resource_name {
			
			pub fn render (&self) -> ::std::string::String {
				::askama::Template::render (&self.template) .expect ("[30355857]")
			}
		}
		
		impl $crate::prelude::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::prelude::Request, _response : &mut $crate::prelude::Response) -> $crate::prelude::ServerResult {
				let _body = self.render ();
				_response.set_status_200 ();
				_response.set_content_type (self.content_type);
				_response.set_body (_body);
				::std::result::Result::Ok (())
			}
		}
	}
}




macro_rules! resource {
	( $_resource_name : ident, $_content_type : ident, $_resource_path : literal, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_resource_name {
			data : &'static [u8],
			content_type : $crate::prelude::hss::ContentType,
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn new () -> Self {
				Self {
						data : Self::data_static,
						content_type : $crate::prelude::hss::ContentType::$_content_type,
					}
			}
			
			pub fn content_type (&self) -> $crate::prelude::hss::ContentType {
				self.content_type
			}
			
			fn description (&self) -> &str {
				$_description
			}
		}
		
		#[ allow (dead_code) ]
		impl $_resource_name {
			
			pub fn data (&self) -> &[u8] {
				self.data
			}
			
			#[ allow (non_upper_case_globals) ]
			const data_static : &'static [u8] = include_bytes! ($_resource_path);
		}
		
		impl $crate::prelude::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::prelude::Request, _response : &mut $crate::prelude::Response) -> $crate::prelude::ServerResult {
				_response.set_status_200 ();
				_response.set_content_type (self.content_type);
				_response.set_body (self.data);
				::std::result::Result::Ok (())
			}
		}
	}
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
						handler : $crate::prelude::hss::HandlerDynArc::new (_resource.wrap ()),
						debug : ::std::option::Option::Some (::std::boxed::Box::new (_description)),
					}
			}
		}
	}
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
	}
}




::std::include! ("./resources.in");


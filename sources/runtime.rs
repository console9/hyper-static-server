

use crate::hss;


use ::std::{
		
		fmt,
		
		ops::Deref as _,
		
	};


use crate::{
		
		hss::ServerResult,
		hss::Extensions,
		hss::ContentType,
		hss::HandlerDynArc,
		
	};




pub trait StaticResource
		where Self : Sized
{
	fn content_type (&self) -> ContentType;
	fn description (&self) -> &'static str;
	fn into_handler_dyn (self) -> HandlerDynArc;
}


pub trait StaticRoute
		where Self : Sized
{
	fn into_route (self) -> hss::Route;
}


pub trait StaticRoutes
		where Self : Sized
{
	fn into_routes (self) -> hss::Routes;
}




pub struct StaticRouteDebug {
	pub debug : Box<dyn fmt::Debug + Send + Sync>,
}


impl StaticRouteDebug {
	
	pub fn new (_debug : impl fmt::Debug + Send + Sync + 'static) -> Self {
		Self {
				debug : Box::new (_debug),
			}
	}
	
	pub fn from_str_static (_description : &'static str) -> Self {
		struct Description (&'static str);
		impl fmt::Debug for Description {
			fn fmt (&self, _formatter : &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
				_formatter.write_str (self.0)
			}
		}
		let _description = Description (_description);
		Self::new (_description)
	}
}


impl fmt::Debug for StaticRouteDebug {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.debug.deref () .fmt (_formatter)
	}
}


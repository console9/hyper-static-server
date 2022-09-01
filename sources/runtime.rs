

use crate::hss;


use ::std::{
		
		fmt,
		
		sync::Arc,
		
		ops::Deref as _,
		
	};


use crate::{
		
		hss::Extensions,
		hss::ContentType,
		hss::HandlerDynArc,
		
	};


use crate::errors::*;




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








pub struct Singleton <T : Send + Sync + 'static> {
	cell : ::once_cell::sync::OnceCell<SingletonResult<T>>,
	builder : fn () -> SingletonResult<T>,
}


impl <T : Send + Sync + 'static> Singleton<T> {
	
	pub const fn new (_builder : fn () -> SingletonResult<T>) -> Self {
		Self {
				cell : ::once_cell::sync::OnceCell::new (),
				builder : _builder,
			}
	}
	
	pub fn get (&self) -> SingletonResult<&T> {
		match self.cell.get_or_init (|| (self.builder) ()) {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.clone ()),
		}
	}
}




pub type SingletonArc <T> = Singleton<Arc<T>>;


impl <T : Send + Sync + 'static> Singleton<Arc<T>> {
	
	pub fn get_arc (&self) -> SingletonResult<Arc<T>> {
		match self.get () {
			Ok (_arc) =>
				Ok (Arc::clone (_arc)),
			Err (_error) =>
				Err (_error.clone ()),
		}
	}
}


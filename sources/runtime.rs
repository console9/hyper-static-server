

use ::std::{
		
		fmt,
		
		ops::Deref as _,
		
	};




pub trait StaticResource {
}


pub trait StaticRoute {
}


pub trait StaticRoutes {
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
}


impl fmt::Debug for StaticRouteDebug {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.debug.deref () .fmt (_formatter)
	}
}


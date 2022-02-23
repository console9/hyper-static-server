

use ::std::{
		
		fmt,
		
		ops::Deref as _,
		
	};




pub struct RouteDebug {
	pub debug : Box<dyn fmt::Debug + Send + Sync>,
}


impl RouteDebug {
	
	pub fn new (_debug : impl fmt::Debug + Send + Sync + 'static) -> Self {
		Self {
				debug : Box::new (_debug),
			}
	}
}


impl fmt::Debug for RouteDebug {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.debug.deref () .fmt (_formatter)
	}
}


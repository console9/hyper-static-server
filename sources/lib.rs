




#[ cfg (feature = "runtime") ]
pub(crate) mod main;

#[ cfg (feature = "runtime") ]
pub(crate) mod server;

#[ cfg (feature = "runtime") ]
pub(crate) mod macros;


#[ cfg (feature = "runtime") ]
pub use main::main_with_static;


#[ cfg (feature = "runtime") ]
pub use ::hyper_simple_server as hss;

#[ cfg (feature = "runtime") ]
pub use ::hyper_simple_server::*;




#[ cfg (feature = "builder") ]
pub(crate) mod builder;

#[ cfg (feature = "builder") ]
pub use builder::*;


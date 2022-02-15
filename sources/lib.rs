




#[ cfg (feature = "server") ]
pub(crate) mod main;

#[ cfg (feature = "server") ]
pub(crate) mod server;

#[ cfg (feature = "server") ]
pub(crate) mod macros_runtime;


#[ cfg (feature = "server") ]
pub use main::main_with_static;




#[ cfg (feature = "builder") ]
pub(crate) mod builder;

#[ cfg (feature = "builder") ]
pub(crate) mod macros_builder;

#[ cfg (feature = "builder") ]
pub use builder::*;




#[ cfg (any (feature = "server", feature = "exporter", feature = "builder") ) ]
pub use ::hyper_simple_server as hss;

#[ cfg (feature = "server") ]
pub use ::hyper_simple_server::*;


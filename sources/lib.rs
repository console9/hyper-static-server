



#[ cfg (feature = "server") ]
pub(crate) mod server;

#[ cfg (feature = "server") ]
pub use crate::main::main_serve_with_static;

#[ cfg (feature = "server") ]
pub use crate::server::*;




#[ cfg (feature = "exporter") ]
pub(crate) mod exporter;

#[ cfg (feature = "exporter") ]
pub use crate::main::main_export_with_static;

#[ cfg (feature = "exporter") ]
pub use crate::exporter::*;




#[ cfg (feature = "builder") ]
pub(crate) mod builder;

#[ cfg (feature = "builder") ]
pub(crate) mod macros_builder;

#[ cfg (feature = "builder") ]
pub use builder::*;




pub(crate) mod main;

#[ cfg (any (feature = "server", feature = "exporter") ) ]
pub(crate) mod macros_runtime;




#[ cfg (any (feature = "server", feature = "exporter", feature = "builder") ) ]
pub use ::hyper_simple_server as hss;

#[ cfg (any (feature = "server", feature = "exporter") ) ]
pub use ::hyper_simple_server::*;




#![ allow (warnings) ]
// #![ cfg_attr (feature = "features-fuzzing", deny (warnings)) ]




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
pub(crate) mod builder_macros;

#[ cfg (feature = "builder") ]
pub use crate::builder::*;




#[ cfg ( any (feature = "server", feature = "exporter") ) ]
pub(crate) mod main;

#[ cfg ( any (feature = "server", feature = "exporter") ) ]
pub use crate::main::{
		main,
		main_wrapper,
	};




#[ cfg (feature = "runtime") ]
pub(crate) mod runtime;

#[ cfg (feature = "runtime") ]
pub(crate) mod runtime_macros;

#[ cfg (feature = "runtime") ]
pub use crate::runtime::*;


#[ cfg (feature = "runtime") ]
#[ cfg (feature = "runtime-sitemaps") ]
pub(crate) mod runtime_sitemaps;

#[ cfg (feature = "runtime") ]
#[ cfg (feature = "runtime-sitemaps") ]
pub use crate::runtime_sitemaps::*;




#[ cfg (feature = "runtime") ]
pub use ::hyper_simple_server as hss;

#[ cfg (feature = "runtime") ]
#[ cfg (feature = "runtime-hss-exports") ]
pub use ::hyper_simple_server::*;


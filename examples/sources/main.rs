
#![ no_implicit_prelude ]


::hyper_static_server::builder_generated! ();

// NOTE:  Needed for broken askama...
use ::std::format;

use ::hyper_static_server::hss::ResultExtWrap as _;


fn main () -> ::hyper_static_server::hss::MainResult {
	
	let _routes = Routes::new () .else_wrap (0x6836acf2) ?;
	
	return ::hyper_static_server::main (_routes);
}


#[ derive (::std::fmt::Debug) ]
#[ derive (::serde::Deserialize) ]
#[ allow (dead_code) ]
struct ExampleAskamaContext {
	some_map : ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}

impl ::hyper_static_server::AskamaContextSerde for ExampleAskamaContext {}


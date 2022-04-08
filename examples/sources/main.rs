
#![ no_implicit_prelude ]


::hyper_static_server::builder_generated! ();

// NOTE:  Needed for broken askama...
use ::std::format;


fn main () -> ::hyper_static_server::hss::ServerResult {
	
	let _routes = Routes::new () ?;
	
	return ::hyper_static_server::main (_routes);
}


#[ derive (::std::fmt::Debug) ]
#[ derive (::serde::Deserialize) ]
#[ allow (dead_code) ]
struct ExampleAskamaContext {
	some_map : ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}

impl ::hyper_static_server::AskamaContextSerde for ExampleAskamaContext {}



#![ no_implicit_prelude ]


::hyper_static_server::builder_generated! ();


fn main () -> ::hyper_static_server::errors::MainResult {
	
	use ::hyper_static_server::errors::ResultExtWrap as _;
	
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


#[ allow (dead_code) ]
trait ExampleAskamaTrait {
	fn some_fn (&self) -> ::std::string::String {
		use ::std::convert::From as _;
		::std::string::String::from ("hello")
	}
}


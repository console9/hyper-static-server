
#![ no_implicit_prelude ]




pub fn main () -> ::hyper_static_server::errors::MainResult {
	
	use ::hyper_static_server::Resource as _;
	use ::hyper_static_server::errors::ResultExtWrap as _;
	
	let _routes = crate::generated::Routes::new_with_defaults () .else_wrap (0x6836acf2) ?;
	
	return ::hyper_static_server::main (_routes);
}




pub mod model {
	
	
	#[ derive (::std::fmt::Debug) ]
	#[ derive (::serde::Deserialize) ]
	pub struct ExampleAskamaContext {
		pub some_map : ::std::collections::HashMap<::std::string::String, ::std::string::String>,
	}
	
	impl ::hyper_static_server::AskamaContextSerde for ExampleAskamaContext {}
	
	
	pub trait ExampleAskamaTrait : ::hyper_static_server::AskamaTrait<Context = ()> {
		fn some_fn (&self) -> ::std::string::String {
			use ::std::convert::From as _;
			::std::string::String::from ("hello")
		}
	}
}




pub mod generated {
	
	
	use crate::model::{
			ExampleAskamaContext,
			ExampleAskamaTrait,
		};
	
	
	::hyper_static_server::builder_generated! ();
}




pub mod tests;


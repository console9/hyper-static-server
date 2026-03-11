



pub mod resources {
	
	use ::std::prelude::v1::*;
	
	::hyper_static_server::askama_resource! (ExampleTemplateResource, ExampleTemplate, !, !, "_empty.txt", "txt");
}




pub mod contexts {
	
	
	::hyper_static_server::context! (ExampleContextResource, ExampleContext);
	
	
	#[ derive (::hyper_static_server::serde::Deserialize) ]
	#[ serde (crate = "::hyper_static_server::serde") ]
	pub struct ExampleContext {}
	
	
	impl ::hyper_static_server::ContextSerde for ExampleContext {
		
		fn hook_initialize (&mut self) -> ::hyper_static_server::errors::ContextResult {
			::hyper_static_server::errors::ContextResult::Ok (())
		}
	}
}


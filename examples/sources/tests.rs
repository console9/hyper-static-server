



pub mod resources {
	
	::hyper_static_server::askama_resource! (ExampleTemplateResource, ExampleTemplate, !, !, "_empty.txt");
}




pub mod contexts {
	
	
	::hyper_static_server::context! (ExampleContextResource, ExampleContext);
	
	
	#[ derive (::serde::Deserialize) ]
	pub struct ExampleContext {}
	
	
	impl ::hyper_static_server::ContextSerde for ExampleContext {
		
		fn hook_initialize (&mut self) -> ::hyper_static_server::errors::ContextResult {
			::hyper_static_server::errors::ContextResult::Ok (())
		}
	}
}


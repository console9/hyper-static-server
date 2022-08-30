



pub mod resources {
	
	::hyper_static_server::askama_template! (ExampleResource, ExampleTemplate, (), (), "_empty.txt");
}




pub mod contexts {
	
	
	::hyper_static_server::context! (ExampleContextBuilder, ExampleContext);
	
	
	#[ derive (::serde::Deserialize) ]
	pub struct ExampleContext {}
	
	
	impl ::hyper_static_server::AskamaContextSerde for ExampleContext {
		
		fn hook_initialize (&mut self) -> ::hyper_static_server::errors::HandlerResult {
			::hyper_static_server::errors::HandlerResult::Ok (())
		}
	}
}


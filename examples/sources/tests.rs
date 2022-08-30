
pub mod resources {
	::hyper_static_server::askama_template! (ExampleResource, ExampleTemplate, (), (), "_empty.txt");
}

pub mod contexts {
	::hyper_static_server::context! (ExampleContextBuilder, ());
}


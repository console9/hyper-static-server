
#![ no_implicit_prelude ]


use ::hyper_static_server as hss;


hss::builder_generated! ();


fn main () -> hss::ServerResult {
	
	Routes::eprintln ();
	Dependencies::eprintln ();
	
	let _routes = Routes::new ();
	
	return hss::main_with_static (_routes, ::std::option::Option::None);
}


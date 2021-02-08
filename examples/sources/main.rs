
#![ no_implicit_prelude ]


use ::hyper_static_server as hss;


hss::builder_generated! ();


fn main () -> hss::ServerResult {
	
	let _routes = Routes::new ();
	
	return hss::main_with_static (_routes);
}


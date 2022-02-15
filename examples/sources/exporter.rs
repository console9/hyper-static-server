
#![ no_implicit_prelude ]


use ::hyper_static_server as hss;


hss::builder_generated! ();


fn main () -> hss::ServerResult {
	
	let _routes = Routes::new ();
	
	return hss::export_routes (_routes);
}


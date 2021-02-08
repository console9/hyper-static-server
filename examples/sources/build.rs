
#![ no_implicit_prelude ]


use ::hyper_static_server as hss;


fn main () -> () {
	
	let mut _builder = hss::Builder::new_with_defaults ();
	
	hss::builder_macros! (_builder);
	
	::std::include! ("./routes.in");
	
	_builder.generate ();
}


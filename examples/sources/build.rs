
#![ no_implicit_prelude ]


fn main () -> () {
	
	::hyper_static_server::build_with_defaults! ("./routes.in");
}


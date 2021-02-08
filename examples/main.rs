
#![ no_implicit_prelude ]



use ::hyper_static_server as hss;




hss::askama! (HomeResource, HomeTemplate, Html, "home.html", "home");
hss::route! (HomeRoute, HomeResource, "/");

hss::resource! (FaviconResource, Icon, embedded, "../examples/files/favicon.ico", "favicon");
hss::route! (FaviconRoute, FaviconResource, "/favicon.ico");

hss::resource! (RobotsResource, Text, embedded, "../examples/files/robots.txt", "robots");
hss::route! (RobotsRoute, RobotsResource, "/robots.txt");

hss::resource! (DataResource, Text, dynamic, "./examples/files/data.txt", "data");
hss::route! (DataRoute, DataResource, "/data.txt");

hss::routes! (Routes, [
		HomeRoute,
		FaviconRoute,
		RobotsRoute,
		DataRoute,
	]);




fn main () -> hss::ServerResult {
	
	let _routes = Routes::new ();
	
	return hss::main_with_static (_routes);
}


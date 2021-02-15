
#![ no_implicit_prelude ]




#[ macro_export ]
macro_rules! builder_macros {
	
	
	( $_builder : ident ) => {
		
		
		#[ allow (unused_macros) ]
		macro_rules! askama {
			( $_source : literal => $_route : literal ) => {
				$_builder.route_askama ($_source, $_route);
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! route_path_builder {
			(default) => {
				&()
			};
			( exact, $_route : literal ) => {
				&(false, $_route as &'static str)
			};
			( prefix, $_route : literal ) => {
				&(true, $_route as &'static str)
			};
			( perhaps ($_type : ident), $_route : literal ) => {
				route_path_builder! ($_type, $_route)
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_css {
			( $_source : literal ) => {
				$_builder.route_css ($_source, route_path_builder! (default));
			};
			( $_source : literal => $_route : literal ) => {
				$_builder.route_css ($_source, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! asset_sass {
			( $_source : literal ) => {
				$_builder.route_sass ($_source, route_path_builder! (default));
			};
			( $_source : literal => $_route : literal ) => {
				$_builder.route_sass ($_source, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! asset_js {
			( $_source : literal ) => {
				$_builder.route_js ($_source, route_path_builder! (default));
			};
			( $_source : literal => $_route : literal ) => {
				$_builder.route_js ($_source, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_image {
			( $_sources : literal ) => {
				$_builder.route_image ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_image ($_sources, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_images {
			( $_sources : literal ) => {
				$_builder.route_images ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_images ($_sources, route_path_builder! (perhaps (prefix), $_route));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_icon {
			( $_sources : literal ) => {
				$_builder.route_icon ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_icon ($_sources, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_icons {
			( $_sources : literal ) => {
				$_builder.route_icons ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_icons ($_sources, route_path_builder! (perhaps (prefix), $_route));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_favicon {
			( $_sources : literal ) => {
				$_builder.route_favicon ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_favicon ($_sources, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_favicons {
			( $_sources : literal ) => {
				$_builder.route_favicons ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_favicons ($_sources, route_path_builder! (perhaps (prefix), $_route));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_font {
			( $_sources : literal ) => {
				$_builder.route_font ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_font ($_sources, route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_fonts {
			( $_sources : literal ) => {
				$_builder.route_fonts ($_sources, route_path_builder! (default));
			};
			( $_sources : literal => $_route : literal ) => {
				$_builder.route_fonts ($_sources, route_path_builder! (perhaps (prefix), $_route));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset {
			( $_source : literal => $_route : literal ) => {
				$_builder.route_asset ($_source, ::std::option::Option::None, route_path_builder! (perhaps (exact), $_route));
			};
			( $_source : literal : $_content_type : ident => $_route : literal ) => {
				$_builder.route_asset ($_source, ::std::option::Option::Some (::std::stringify! ($_content_type)), route_path_builder! (perhaps (exact), $_route));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets {
			( $_source : literal => $_route : literal ) => {
				$_builder.route_assets ($_source, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route));
			};
			( $_source : literal : $_content_type : ident => $_route : literal ) => {
				$_builder.route_assets ($_source, ::std::option::Option::Some (::std::stringify! ($_content_type)), route_path_builder! (perhaps (prefix), $_route));
			};
		}
		
		
	};
}



#![ no_implicit_prelude ]




#[ macro_export ]
macro_rules! builder_macros {
	
	
	( $_builder : ident ) => {
		
		
		
		
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
				$crate::builder_call! ($_builder, route_css, ($_source, route_path_builder! (default)), (0x76ec81d1));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_css, ($_source, route_path_builder! (perhaps (exact), $_route)), (0x16cfdc28));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! asset_sass {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, route_sass, ($_source, route_path_builder! (default)), (0x714c403a));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_sass, ($_source, route_path_builder! (perhaps (exact), $_route)), (0xf8cdbb33));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! asset_js {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, route_js, ($_source, route_path_builder! (default)), (0xd7b6a398));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_js, ($_source, route_path_builder! (perhaps (exact), $_route)), (0x039b17d3));
			};
		}
		
		
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_image {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, route_image, ($_source, route_path_builder! (default)), (0x3c6853f7));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_image, ($_source, route_path_builder! (perhaps (exact), $_route)), (0xa4be0f67));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_images {
			( $_sources : literal ) => {
				$crate::builder_call! ($_builder, route_images, ($_sources, ::std::option::Option::None, route_path_builder! (default)), (0xc2f0c669));
			};
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_images, ($_sources, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0xa2ee2c3c));
			};
			( $_sources : literal, glob : $_glob : literal ) => {
				$crate::builder_call! ($_builder, route_images, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (default)), (0x274e0974));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_images, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (perhaps (prefix), $_route)), (0xf4033023));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_icon {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, route_icon, ($_source, route_path_builder! (default)), (0xcb47ada1));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_icon, ($_source, route_path_builder! (perhaps (exact), $_route)), (0x6e4fe9ac));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_icons {
			( $_sources : literal ) => {
				$crate::builder_call! ($_builder, route_icons, ($_sources, ::std::option::Option::None, route_path_builder! (default)), (0x5ed5f8c5));
			};
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_icons, ($_sources, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0x301b41df));
			};
			( $_sources : literal, glob : $_glob : literal ) => {
				$crate::builder_call! ($_builder, route_icons, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (default)), (0x6838ad5b));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_icons, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (perhaps (prefix), $_route)), (0x5e6b3ee8));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_favicon {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, route_favicon, ($_source, route_path_builder! (default)), (0x9ba89b0a));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_favicon, ($_source, route_path_builder! (perhaps (exact), $_route)), (0x3c20acca));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_favicons {
			( $_sources : literal ) => {
				$crate::builder_call! ($_builder, route_favicons, ($_sources, ::std::option::Option::None, route_path_builder! (default)), (0xd8657b51));
			};
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_favicons, ($_sources, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0x112340a3));
			};
			( $_sources : literal, glob : $_glob : literal ) => {
				$crate::builder_call! ($_builder, route_favicons, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (default)), (0xa1b7b0ce));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_favicons, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (perhaps (prefix), $_route)), (0x1681a399));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_font {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, route_font, ($_source, route_path_builder! (default)), (0x8470b423));
			};
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_font, ($_source, route_path_builder! (perhaps (exact), $_route)), (0x5e41011e));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_fonts {
			( $_sources : literal ) => {
				$crate::builder_call! ($_builder, route_fonts, ($_sources, ::std::option::Option::None, route_path_builder! (default)), (0xda42d3e9));
			};
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_fonts, ($_sources, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0x9ad64b09));
			};
			( $_sources : literal, glob : $_glob : literal ) => {
				$crate::builder_call! ($_builder, route_fonts, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (default)), (0xb700bad6));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_fonts, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (perhaps (prefix), $_route)), (0x06f49685));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset {
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_asset, ($_source, ::std::option::Option::None, route_path_builder! (perhaps (exact), $_route)), (0x5cdb13b5));
			};
			( $_source : literal, content_type : $_content_type : ident => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_asset, ($_source, ::std::option::Option::Some (::std::stringify! ($_content_type)), route_path_builder! (perhaps (exact), $_route)), (0xd03ccf86));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets {
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_assets, ($_sources, ::std::option::Option::None, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0xb51be306));
			};
			( $_sources : literal, content_type : $_content_type : ident => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_assets, ($_sources, ::std::option::Option::None, ::std::option::Option::Some (::std::stringify! ($_content_type)), route_path_builder! (perhaps (prefix), $_route)), (0xc6966c8b));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_assets, ($_sources, ::std::option::Option::Some ($_glob), ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0x4acc2c15));
			};
			( $_sources : literal, glob : $_glob : literal, content_type : $_content_type : ident => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_assets, ($_sources, ::std::option::Option::Some ($_glob), ::std::option::Option::Some (::std::stringify! ($_content_type)), route_path_builder! (perhaps (prefix), $_route)), (0x9c99e92d));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! asset_watch {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, watch_asset, ($_source), (0x764afafe));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! assets_watch {
			( $_sources : literal ) => {
				$crate::builder_call! ($_builder, watch_assets, ($_sources, ::std::option::Option::None), (0x4f90b5b1));
			};
			( $_sources : literal, glob : $_glob : literal ) => {
				$crate::builder_call! ($_builder, watch_assets, ($_sources, ::std::option::Option::Some ($_glob)), (0xdbfc9a0b));
			};
		}
		
		
		
		
		#[ allow (unused_macros) ]
		macro_rules! askama {
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_askama, ($_source, route_path_builder! (perhaps (exact), $_route)), (0xe3a36527));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! askamas {
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_askamas, ($_sources, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0x2a28230c));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_askamas, ($_sources, ::std::option::Option::Some ($_glob), route_path_builder! (perhaps (prefix), $_route)), (0xdb8e73df));
			};
		}
		
		
		#[ allow (unused_macros) ]
		macro_rules! askama_watch {
			( $_source : literal ) => {
				$crate::builder_call! ($_builder, watch_askama, ($_source), (0xd7c76ec9));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! askamas_watch {
			( $_sources : literal ) => {
				$crate::builder_call! ($_builder, watch_askamas, ($_sources, ::std::option::Option::None), (0x4fe8df42));
			};
			( $_sources : literal, glob : $_glob : literal ) => {
				$crate::builder_call! ($_builder, watch_askamas, ($_sources, ::std::option::Option::Some ($_glob)), (0x59d44dee));
			};
		}
		
		
		
		
		#[ allow (unused_macros) ]
		macro_rules! markdown_askama {
			( $_source_markdown : literal, template : $_source_template : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdown_askama, ($_source_markdown, $_source_template, route_path_builder! (perhaps (exact), $_route)), (0x0045ece1));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! markdowns_askama {
			( $_sources_markdown : literal, template : $_source_template : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdowns_askama, ($_sources_markdown, ::std::option::Option::None, $_source_template, route_path_builder! (perhaps (prefix), $_route)), (0x84a5049c));
			};
			( $_sources_markdown : literal, glob : $_glob : literal, template : $_source_template : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdowns_askama, ($_sources_markdown, ::std::option::Option::Some ($_glob), $_source_template, route_path_builder! (perhaps (prefix), $_route)), (0x273fbd0e));
			};
		}
		
		
		
		
		#[ allow (unused_macros) ]
		macro_rules! markdown {
			( $_source : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdown, ($_source, ::std::option::Option::None, ::std::option::Option::None, route_path_builder! (perhaps (exact), $_route)), (0x97238678));
			};
			( $_source : literal, header : $_header : literal, footer : $_footer : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdown, ($_source, ::std::option::Option::Some ($_header), ::std::option::Option::Some ($_footer), route_path_builder! (perhaps (exact), $_route)), (0x0be4beba));
			};
		}
		
		#[ allow (unused_macros) ]
		macro_rules! markdowns {
			( $_sources : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdowns, ($_sources, ::std::option::Option::None, ::std::option::Option::None, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0xf73441ed));
			};
			( $_sources : literal, header : $_header : literal, footer : $_footer : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdowns, ($_sources, ::std::option::Option::None, ::std::option::Option::Some ($_header), ::std::option::Option::Some ($_footer), route_path_builder! (perhaps (prefix), $_route)), (0x3664d302));
			};
			( $_sources : literal, glob : $_glob : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdowns, ($_sources, ::std::option::Option::Some ($_glob), ::std::option::Option::None, ::std::option::Option::None, route_path_builder! (perhaps (prefix), $_route)), (0xa5bd4849));
			};
			( $_sources : literal, glob : $_glob : literal, header : $_header : literal, footer : $_footer : literal => $_route : literal ) => {
				$crate::builder_call! ($_builder, route_markdowns, ($_sources, ::std::option::Option::Some ($_glob), ::std::option::Option::Some ($_header), ::std::option::Option::Some ($_footer), route_path_builder! (perhaps (prefix), $_route)), (0xf712c3cf));
			};
		}
		
		
	};
}




#[ macro_export ]
macro_rules! builder_call {
	
	( $_builder : ident, $_method : ident, ( $( $_argument : expr ),* ), ( $_code : literal ) ) => {
		{
			use $crate::hss::internals::ResultExtPanic as _;
			$_builder .$_method ( $( $_argument, )* ) .or_panic ($_code);
		}
	};
}




#[ macro_export ]
macro_rules! build_with_defaults {
	
	( $_resources : literal ) => {
		$crate::build_with_defaults! ({
			::std::include! ($_resources)
		});
	};
	
	( $_resources : block ) => {
		{
			let mut _builder = $crate::Builder::new_with_defaults ();
			
			$crate::builder_macros! (_builder);
			
			{
				$_resources
			}
			
			$crate::builder_call! (_builder, generate, (), (0x9e52f017));
		}
	};
}


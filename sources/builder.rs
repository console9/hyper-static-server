



#[ allow (unused_imports) ]
use ::std::{
		
		cell,
		env,
		ffi,
		fmt,
		fs,
		io,
		iter,
		mem,
		path,
		rc,
		
		collections::BTreeSet,
		iter::Iterator,
		path::{Path, PathBuf},
		
		str::FromStr as _,
		fmt::{Write as _},
		io::{Write as _},
		
	};


use ::globset;
use ::walkdir;
use ::blake2;
use ::proc_macro2;

#[ cfg (feature = "sass-rs") ]
use ::sass_rs as sass;

#[ cfg (feature = "sass-alt") ]
use ::sass_alt as sass;

#[ cfg (feature = "pulldown-cmark") ]
use ::pulldown_cmark as cmark;

#[ allow (unused_imports) ]
use ::hyper_simple_server::{
		error_with_code,
		error_with_format,
		error_with_message,
		ResultExtWrap as _,
		ResultExtPanic as _,
	};




pub type BuilderResult<V = ()> = Result<V, BuilderError>;
pub type BuilderError = io::Error;




#[ derive (Clone, Debug) ]
pub struct BuilderConfiguration {
	
	pub sources : Option<PathBuf>,
	pub assets_sources : Option<PathBuf>,
	pub templates_sources : Option<PathBuf>,
	
	pub assets_route_base : Option<PathBuf>,
	
	pub css_route_base : Option<PathBuf>,
	pub js_route_base : Option<PathBuf>,
	
	pub images_route_base : Option<PathBuf>,
	pub icons_route_base : Option<PathBuf>,
	pub favicons_route_base : Option<PathBuf>,
	pub fonts_route_base : Option<PathBuf>,
	
	pub outputs : PathBuf,
	pub generated : PathBuf,
}


impl Default for BuilderConfiguration {
	
	fn default () -> Self {
		
		let _sources = Self::resolve_sources () .or_panic (0xc6e7914d);
		let _outputs = Self::resolve_outputs () .or_panic (0x344e2c9e);
		
		let _assets_sources = _sources.join ("./assets");
		let _assets_sources = if _assets_sources.exists () { Some (_assets_sources) } else { None };
		
		let _templates_sources = _sources.join ("./templates");
		let _templates_sources = if _templates_sources.exists () { Some (_templates_sources) } else { None };
		
		let _generated = _outputs.join ("./hss-builder-generated-default.in");
		
		let _assets_route_base = Some (PathBuf::from ("/assets"));
		
		let _css_route_base = Some (PathBuf::from ("/assets/css"));
		let _js_route_base = Some (PathBuf::from ("/assets/js"));
		
		let _images_route_base = Some (PathBuf::from ("/assets/images"));
		let _icons_route_base = Some (PathBuf::from ("/assets/icons"));
		let _favicons_route_base = Some (PathBuf::from ("/assets/favicons"));
		let _fonts_route_base = Some (PathBuf::from ("/assets/fonts"));
		
		Self {
				
				sources : Some (_sources),
				assets_sources : _assets_sources,
				templates_sources : _templates_sources,
				
				assets_route_base : _assets_route_base,
				
				css_route_base : _css_route_base,
				js_route_base : _js_route_base,
				
				images_route_base : _images_route_base,
				icons_route_base : _icons_route_base,
				favicons_route_base : _favicons_route_base,
				fonts_route_base : _fonts_route_base,
				
				outputs : _outputs,
				generated : _generated,
			}
	}
}


impl BuilderConfiguration {
	
	pub fn minimal () -> BuilderResult<Self> {
		
		let _sources = Self::resolve_sources () ?;
		let _outputs = Self::resolve_outputs () ?;
		let _generated = _outputs.join ("./hss-builder-generated-default.in");
		
		let _builder = Self {
				sources : Some (_sources),
				assets_sources : None,
				templates_sources : None,
				
				assets_route_base : None,
				
				css_route_base : None,
				js_route_base : None,
				
				images_route_base : None,
				icons_route_base : None,
				favicons_route_base : None,
				fonts_route_base : None,
				
				outputs : _outputs,
				generated : _generated,
			};
		
		Ok (_builder)
	}
	
	pub fn resolve_sources () -> BuilderResult<PathBuf> {
		let _sources = PathBuf::from (env::var ("CARGO_MANIFEST_DIR") .or_wrap (0x4c6c04d8) ?);
		if ! _sources.is_dir () {
			return Err (error_with_code (0x148bc689));
		}
		normalize_path (&_sources)
	}
	
	pub fn resolve_outputs () -> BuilderResult<PathBuf> {
		let _outputs = PathBuf::from (env::var ("OUT_DIR") .or_wrap (0xf039039f) ?);
		if ! _outputs.is_dir () {
			return Err (error_with_code (0xfcee6b4d));
		}
		normalize_path (&_outputs)
	}
}




pub struct Builder {
	configuration : BuilderConfiguration,
	generated : String,
	counter : u32,
	route_names : Vec<String>,
	dependencies : BTreeSet<PathBuf>,
}


impl Builder {
	
	pub fn new (_configuration : BuilderConfiguration) -> Self {
		Self {
				configuration : _configuration,
				generated : String::with_capacity (1024 * 1024),
				counter : 0,
				route_names : Vec::new (),
				dependencies : BTreeSet::new (),
			}
	}
	
	pub fn new_with_defaults () -> Self {
		Self::new (BuilderConfiguration::default ())
	}
}




impl Builder {
	
	
	
	
	fn route_asset_raw (&mut self, _relative : &Path, _source : &Path, _content_type : &str, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _macro : &str, _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _route = _route_builder.build (_relative, &_source, _route_base, None) ?;
		let _extensions = _extensions_builder.build () ?;
		
		let _id = self.generate_id ();
		
		let _description = if let Some (_relative) = _source_relative {
			format! ("{} ({}, from = `{}`, file = `...{}`)", _macro, _content_type, _source_0, _relative.display ())
		} else {
			format! ("{} ({}, file = `{}`)", _macro, _content_type, _source_0)
		};
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies_include (&_source) ?;
		
		let _mode = "auto";
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, {}, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _mode, _source, _description) .infallible (0x5fa962ac);
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0x46de4cc9);
		
		Ok (())
	}
	
	
	
	
	pub fn route_askama (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_templates_sources, _source_0) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		self.route_askama_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	pub fn route_askamas (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_templates_sources, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_askama_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	fn route_askama_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _relative_1 = _relative.with_extension ("");
		
		let _template = _relative.strip_prefix ("/") .infallible (0x7285dc26);
		
		let _route = _route_builder.build (&_relative_1, &_source, _route_base, None) ?;
		let _extensions = _extensions_builder.build () ?;
		
		let _id = self.generate_id ();
		
		let _content_type = "Html";
		let _description = if let Some (_relative) = _source_relative {
			format! ("askama ({}, from = `{}`, file = `...{}`)", _content_type, _source_0, _relative.display ())
		} else {
			format! ("askama ({}, file = `{}`)", _content_type, _source_0)
		};
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies_include (&_source) ?;
		
		writeln! (self.generated, "::hyper_static_server::askama! (Resource_{}, Template_{}, {}, {:?}, {:?});", _id, _id, _content_type, _template, _description) .infallible (0x35966385);
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0x41a5ee4c);
		
		Ok (())
	}
	
	
	pub fn watch_askama (&mut self, _source : &str) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_templates_sources, _source) ?;
		
		self.dependencies_include (&_source) ?;
		
		Ok (())
	}
	
	pub fn watch_askamas (&mut self, _sources : &str, _glob : Option<&str>) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_templates_sources, _sources, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		self.dependencies_include_all (_files.iter () .map (|_pair| _pair.1.as_path ())) ?;
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "pulldown-cmark") ]
	pub fn route_markdown_askama (&mut self, _source_markdown_0 : &str, _source_template_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative_template, _source_template) = self.resolve_file (_templates_sources, _source_template_0) ?;
		
		let _template = _relative_template.strip_prefix ("/") .infallible (0xda5e5ad4);
		
		self.dependencies_include (&_source_template) ?;
		
		let (_relative_markdown, _source_markdown) = self.resolve_file (None, _source_markdown_0) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		self.route_markdown_askama_0 (&_relative_markdown, &_source_markdown, &_template, _route_base, _route_builder, _extensions_builder, _source_markdown_0, None)
	}
	
	#[ cfg (feature = "pulldown-cmark") ]
	pub fn route_markdowns_askama (&mut self, _sources_markdown_0 : &str, _glob : Option<&str>, _source_template_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative_template, _source_template) = self.resolve_file (_templates_sources, _source_template_0) ?;
		
		let _template = _relative_template.strip_prefix ("/") .infallible (0xe0168bd3);
		
		self.dependencies_include (&_source_template) ?;
		
		let (_files_markdown, _folders_markdown) = self.resolve_files (None, _sources_markdown_0, _glob) ?;
		
		self.dependencies_include_all (_folders_markdown.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		for (_relative_markdown, _source_markdown) in _files_markdown.into_iter () {
			
			self.route_markdown_askama_0 (&_relative_markdown, &_source_markdown, &_template, _route_base, _route_builder, _extensions_builder, _sources_markdown_0, Some (_relative_markdown.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "pulldown-cmark") ]
	fn route_markdown_askama_0 (&mut self, _relative_markdown : &Path, _source_markdown : &Path, _template : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _relative_markdown_1 = _relative_markdown.with_extension ("");
		
		self.dependencies_include (_source_markdown) ?;
		
		let (_markdown_title, _markdown_body) = self.compile_markdown (_source_markdown, false, true) ?;
		let _markdown_title = _markdown_title.unwrap_or (String::new ());
		
		let _output_markdown = self.configuration.outputs.join (fingerprint_data (&_markdown_body)) .with_extension ("html");
		create_file_from_str (&_output_markdown, &_markdown_body) ?;
		
		let _route = _route_builder.build (&_relative_markdown_1, _source_markdown, _route_base, None) ?;
		let _extensions = _extensions_builder.build () ?;
		
		let _id = self.generate_id ();
		
		let _content_type = "Html";
		let _description = if let Some (_relative) = _source_relative {
			format! ("markdown_askama ({}, from = `{}`, file = `...{}`)", _content_type, _source_0, _relative.display ())
		} else {
			format! ("markdown_askama ({}, file = `{}`)", _content_type, _source_0)
		};
		
		self.route_names.push (format! ("Route_{}", _id));
		
		writeln! (self.generated, "::hyper_static_server::askama_with_title_and_body! (Resource_{}, Template_{}, {}, {:?}, {:?}, {:?}, {:?});", _id, _id, _content_type, _template, _markdown_title, _output_markdown, _description) .infallible (0xd64341cb);
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0xafb30ea0);
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "pulldown-cmark") ]
	pub fn route_markdown (&mut self, _source_0 : &str, _header_source : Option<&str>, _footer_source : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let (_header_data, _footer_data) = self.route_markdown_brackets (_header_source, _footer_source) ?;
		
		let (_relative, _source) = self.resolve_file (None, _source_0) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		self.route_markdown_0 (&_relative, &_source, _header_data.as_ref (), _footer_data.as_ref (), _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	#[ cfg (feature = "pulldown-cmark") ]
	pub fn route_markdowns (&mut self, _sources_0 : &str, _glob : Option<&str>, _header_source : Option<&str>, _footer_source : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let (_header_data, _footer_data) = self.route_markdown_brackets (_header_source, _footer_source) ?;
		
		let (_files, _folders) = self.resolve_files (None, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		for (_relative, _source) in _files.into_iter () {
			
			if _source.extension () .or_wrap (0xc1ecda55) ? != "md" {
				return Err (error_with_format (0x393ea45d, format_args! ("{}", _source.display ())));
			}
			
			self.route_markdown_0 (&_relative, &_source, _header_data.as_ref (), _footer_data.as_ref (), _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "pulldown-cmark") ]
	fn route_markdown_brackets (&mut self, _header_source : Option<&str>, _footer_source : Option<&str>) -> BuilderResult<(Option<String>, Option<String>)> {
		
		let _header_source = _header_source.map (|_source| BuilderResult::Ok (self.resolve_file (None, _source) ? .1)) .transpose () ?;
		let _footer_source = _footer_source.map (|_source| BuilderResult::Ok (self.resolve_file (None, _source) ? .1)) .transpose () ?;
		
		let _header_data = _header_source.as_ref () .map (
				|_source| {
					let _data = fs::read_to_string (_source) ?;
					self.dependencies_include (_source) ?;
					BuilderResult::Ok (_data)
				})
				.transpose () ?;
		let _footer_data = _footer_source.as_ref () .map (
				|_source| {
					let _data = fs::read_to_string (_source) ?;
					self.dependencies_include (_source) ?;
					BuilderResult::Ok (_data)
				})
				.transpose () ?;
		
		Ok ((_header_data, _footer_data))
	}
	
	
	#[ cfg (feature = "pulldown-cmark") ]
	fn route_markdown_0 (&mut self, _relative : &Path, _source : &Path, _header_data : Option<&String>, _footer_data : Option<&String>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _relative_1 = _relative.with_extension ("");
		
		self.dependencies_include (&_source) ?;
		
		let _compiled = if _header_data.is_some () || _footer_data.is_some () {
			
			let _header_data = _header_data.map (String::as_str) .unwrap_or ("");
			let _footer_data = _footer_data.map (String::as_str) .unwrap_or ("");
			
			let (_title, _contents_data) = self.compile_markdown (&_source, false, true) ?;
			let _title = _title.as_ref () .map (String::as_str) .unwrap_or ("");
			let _title = {
				let mut _buffer = String::with_capacity (_title.len () * 3 / 2);
				cmark::escape::escape_html (&mut _buffer, &_title) .infallible (0xef399d64);
				_buffer
			};
			
			let mut _buffer = String::with_capacity (_header_data.len () + _contents_data.len () + _footer_data.len ());
			_buffer.push_str (&_header_data.replace ("@@{{HSS::Markdown::Title}}", &_title));
			_buffer.push_str (&_contents_data);
			_buffer.push_str (&_footer_data.replace ("@@{{HSS::Markdown::Title}}", &_title));
			_buffer
			
		} else {
			let (_title, _contents_data) = self.compile_markdown (&_source, true, true) ?;
			_contents_data
		};
		
		let _output = self.configuration.outputs.join (fingerprint_data (&_compiled)) .with_extension ("html");
		create_file_from_str (&_output, &_compiled) ?;
		
		// FIXME:  Here the second argument should be `_source`.
		self.route_asset_raw (&_relative_1, &_output, "Html", _route_base, _route_builder, _extensions_builder, "markdown", _source_0, _source_relative) ?;
		
		self.dependencies_exclude (&_output) ?;
		
		Ok (())
	}
	
	
	
	
	pub fn route_css (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _css_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_css_sources, _source_0) ?;
		
		let _route_base = self.configuration.css_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_raw (&_relative, &_source, "Css", _route_base, _route_builder, _extensions_builder, "resource_css", _source_0, None)
	}
	
	
	#[ cfg ( any (feature = "sass-rs", feature = "sass-alt") ) ]
	pub fn route_sass (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _css_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_css_sources, _source_0) ?;
		
		let _relative_1 = _relative.with_extension ("css");
		
		self.dependencies_include (&_source) ?;
		
		let _compiled = self.compile_sass (&_source) ?;
		
		let _output = self.configuration.outputs.join (fingerprint_data (&_compiled)) .with_extension ("css");
		create_file_from_str (&_output, &_compiled) ?;
		
		let _route_base = self.configuration.css_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		// FIXME:  Here the second argument should be `_source`.
		self.route_asset_raw (&_relative_1, &_output, "Css", _route_base, _route_builder, _extensions_builder, "resource_sass", _source_0, None) ?;
		
		self.dependencies_exclude (&_output) ?;
		
		Ok (())
	}
	
	
	
	
	pub fn route_js (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _js_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_js_sources, _source_0) ?;
		
		let _route_base = self.configuration.js_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_raw (&_relative, &_source, "Js", _route_base, _route_builder, _extensions_builder, "resource_js", _source_0, None)
	}
	
	
	
	
	pub fn route_image (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_image", _source_0, None)
	}
	
	pub fn route_images (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_image", _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	pub fn route_icon (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.icons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_icon", _source_0, None)
	}
	
	pub fn route_icons (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.icons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_icon", _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	pub fn route_favicon (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_favicon", _source_0, None)
	}
	
	pub fn route_favicons (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_favicon", _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	fn route_image_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _macro : &str, _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _content_type = detect_content_type_from_extension (&_source) ?;
		match _content_type {
			"Png" | "Jpeg" | "Icon" | "Svg" =>
				(),
			_ =>
				return Err (error_with_format (0x0fd2d804, format_args! ("{}", _source.display ()))),
		};
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _extensions_builder, _macro, _source_0, _source_relative)
	}
	
	
	
	
	pub fn route_font (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_font_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	pub fn route_fonts (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_font_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	fn route_font_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _content_type = detect_content_type_from_extension (&_source) ?;
		match _content_type {
			"FontTtf" | "FontOtf" | "FontWoff" | "FontWoff2" =>
				(),
			_ =>
				return Err (error_with_format (0x1a4ccbf4, format_args! ("{}", _source.display ()))),
		};
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _extensions_builder, "resource_font", _source_0, _source_relative)
	}
	
	
	
	
	pub fn route_asset (&mut self, _source_0 : &str, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_0 (&_relative, &_source, _content_type, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	pub fn route_assets (&mut self, _sources_0 : &str, _glob : Option<&str>, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_asset_0 (&_relative, &_source, _content_type, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	fn route_asset_0 (&mut self, _relative : &Path, _source : &Path, _content_type : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _content_type = _content_type.map_or_else (|| detect_content_type_from_extension (&_source), |_content_type| Ok (_content_type)) ?;
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _extensions_builder, "resource_asset", _source_0, _source_relative)
	}
	
	
	
	
	pub fn route_sitemap (&mut self, _prefix : &str, _format : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _route = _route_builder.build (Path::new (""), Path::new (""), None, None) ?;
		let _extensions = _extensions_builder.build () ?;
		let _format = token_tree_parse (_format) ?;
		
		let _id = self.generate_id ();
		
		self.route_names.push (format! ("Route_{}", _id));
		
		writeln! (self.generated, "::hyper_static_server::route_sitemap! (Route_{}, {:?}, {:?}, {}, {});", _id, _route, _prefix, _format, _extensions) .infallible (0x5f529a53);
		
		Ok (())
	}
	
	
	
	
	pub fn watch_asset (&mut self, _source : &str) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) ?;
		
		self.dependencies_include (&_source) ?;
		
		Ok (())
	}
	
	pub fn watch_assets (&mut self, _sources : &str, _glob : Option<&str>) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources, _glob) ?;
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		self.dependencies_include_all (_files.iter () .map (|_pair| _pair.1.as_path ())) ?;
		
		Ok (())
	}
	
	
	
	
	pub fn generate (mut self) -> BuilderResult {
		
		self.dependencies_extend () ?;
		
		writeln! (self.generated, "::hyper_static_server::routes! (Routes, [") .infallible (0x4bf5618f);
		for _route_name in self.route_names.into_iter () {
			writeln! (self.generated, "\t{},", _route_name) .infallible (0x894377dd);
		}
		writeln! (self.generated, "]);") .infallible (0x28d1ed4d);
		
		writeln! (self.generated, "::hyper_static_server::dependencies! (Dependencies, [") .infallible (0x1a6c02cd);
		for _dependency in self.dependencies.iter () {
			writeln! (self.generated, "\t{:?},", _dependency) .infallible (0x9df69eb7);
		}
		writeln! (self.generated, "]);") .infallible (0x57d05438);
		
		create_file_from_str (&self.configuration.generated, &self.generated) ?;
		
		if false {
			eprintln! ("--------------------------------------------------------------------------------");
			eprintln! ("{}", self.generated);
			eprintln! ("--------------------------------------------------------------------------------");
		}
		
		for _dependency in self.dependencies {
			println! ("cargo:rerun-if-changed={}", _dependency.display ());
		}
		
		Ok (())
	}
}




impl Builder {
	
	
	fn resolve_file (&self, _root : Option<&Path>, _source : &str) -> BuilderResult<(PathBuf, PathBuf)> {
		
		let (_path, _relative_root) = self.resolve_source (_root, _source, true) ?;
		
		if ! _path.is_file () {
			return Err (error_with_format (0x039d945b, format_args! ("{}", _path.display ())));
		}
		
		self.resolve_relative_and_path (&_path, &_relative_root)
	}
	
	
	fn resolve_files (&self, _root : Option<&Path>, _sources : &str, _glob : Option<&str>) -> BuilderResult<(Vec<(PathBuf, PathBuf)>, Vec<PathBuf>)> {
		
		let (_root, _relative_root) = self.resolve_source (_root, _sources, false) ?;
		
		if ! _root.is_dir () {
			return Err (error_with_code (0x621693a6));
		}
		
		let _glob = _glob.map (|_pattern| globset::Glob::new (_pattern) .or_wrap (0xf68023ce)) .transpose () ?;
		let _glob = _glob.map (|_pattern| _pattern.compile_matcher ());
		
		let mut _files = Vec::new ();
		let mut _folders = Vec::new ();
		
		let _walker = walkdir::WalkDir::new (&_root)
				.follow_links (true)
				.sort_by (|_left, _right| ffi::OsStr::cmp (_left.file_name (), _right.file_name ()));
		
		for _entry in _walker {
			let _entry = _entry ?;
			let _path = _entry.path ();
			
			if _path.is_file () {
				
				if let Some (_glob) = _glob.as_ref () {
					if ! _glob.is_match (_path) {
						continue;
					}
				}
				
				let _relative_and_path = self.resolve_relative_and_path (_path, &_relative_root) ?;
				
				_files.push (_relative_and_path);
			}
			
			if _path.is_dir () {
				
				let _path = normalize_path (_path) ?;
				
				_folders.push (_path);
			}
		}
		
		Ok ((_files, _folders))
	}
	
	
	fn resolve_source (&self, _root : Option<&Path>, _source : &str, _name_only : bool) -> BuilderResult<(PathBuf, PathBuf)> {
		
		let _path = if _source.starts_with ("_/") || (_source == "_") {
			let _root = _root.or_wrap (0x6e3319c9) ?;
			if _source != "_" {
				_root.join (&_source[2..])
			} else {
				_root.to_owned ()
			}
			
		} else if _source.starts_with ("./") || _source.starts_with ("..") || (_source == ".") || (_source == "..") {
			let _root = self.configuration.sources.as_ref () .or_wrap (0x0791a9b4) ?;
			_root.join (&_source)
			
		} else if _source.starts_with (">") {
			PathBuf::from (&_source[1..])
			
		} else {
			return Err (error_with_code (0x41071330));
		};
		
		if ! _path.exists () {
			return Err (error_with_format (0x1086bd9d, format_args! ("{}", _path.display ())));
		}
		
		if _name_only {
			let _relative_root = _path.parent () .or_wrap (0x067a2cad) ? .to_path_buf ();
			Ok ((_path, _relative_root))
		} else {
			Ok ((_path.clone (), _path))
		}
	}
	
	
	fn resolve_relative_and_path (&self, _path : &Path, _relative_root : &Path) -> BuilderResult<(PathBuf, PathBuf)> {
		
		let _relative = _path.strip_prefix (_relative_root) .or_wrap (0x546e7cd9) ? .to_str () .or_wrap (0xa48f283c) ?;
		let _relative = ["/", _relative].concat () .into ();
		
		let _path = normalize_path (&_path) ?;
		
		Ok ((_relative, _path))
	}
}




impl Builder {
	
	
	fn dependencies_include (&mut self, _path : &Path) -> BuilderResult {
		
		self.dependencies.insert (_path.into ());
		
		Ok (())
	}
	
	fn dependencies_include_all <'a> (&mut self, _paths : impl Iterator<Item = &'a Path>) -> BuilderResult {
		
		for _path in _paths {
			self.dependencies_include (_path) ?;
		}
		
		Ok (())
	}
	
	#[ allow (dead_code) ]
	fn dependencies_exclude (&mut self, _path : &Path) -> BuilderResult {
		
		self.dependencies.remove (_path.into ());
		
		Ok (())
	}
	
	fn dependencies_extend (&mut self) -> BuilderResult {
		
		let mut _extra = Vec::new ();
		
		for _dependency in self.dependencies.iter () {
			
			let _metadata = fs::symlink_metadata (_dependency) .or_wrap (0x06a4fbd5) ?;
			
			if _metadata.file_type () .is_symlink () {
				let _target = _dependency.canonicalize () .or_wrap (0x8df4310e) ?;
				_extra.push (_target);
			}
		}
		
		for _dependency in _extra.into_iter () {
			self.dependencies.insert (_dependency);
		}
		
		Ok (())
	}
}




impl Builder {
	
	
	fn generate_id (&mut self) -> String {
		let _id = self.counter;
		self.counter += 1;
		format! ("{:04}", _id)
	}
}




impl Builder {
	
	
	#[ cfg (feature = "sass-rs") ]
	fn compile_sass (&mut self, _source : &Path) -> BuilderResult<String> {
		
		let _extension = _source.extension () .or_wrap (0x836ff108) ? .to_str () .or_wrap (0x4068e13f) ?;
		let _indented_syntax = match _extension {
			"sass" =>
				true,
			"scss" =>
				false,
			_ =>
				return Err (error_with_code (0x720c0c23)),
		};
		
		let _options = sass::Options {
				output_style : sass::OutputStyle::Expanded,
				precision : 4,
				indented_syntax : _indented_syntax,
				include_paths : vec! [],
			};
		
		let mut _context = sass::Context::new_file (&_source) .map_err (|_message| error_with_message (0xfde48681, &_message)) ?;
		_context.set_options (_options);
		let _data = _context.compile () .map_err (|_message| error_with_message (0x00c4c0dd, &_message)) ?;
		
		Ok (_data)
	}
	
	
	#[ cfg (feature = "sass-alt") ]
	fn compile_sass (&mut self, _source : &Path) -> BuilderResult<String> {
		
		let _parent = _source.parent () .or_wrap (0xf6ce0d79) ?;
		
		let _extension = _source.extension () .or_wrap (0xf2cd37bc) ? .to_str () .or_wrap (0xdb216e38) ?;
		let _input_syntax = match _extension {
			"sass" =>
				sass::InputSyntax::SASS,
			"scss" =>
				sass::InputSyntax::SCSS,
			_ =>
				return Err (error_with_code (0x90668feb)),
		};
		
		pub struct Importer { parent : PathBuf, resolved : rc::Rc<cell::RefCell<Vec<Box<Path>>>> }
		impl sass::SassImporter for Importer {
			fn callback (&mut self, _path_0 : &ffi::CStr, _compiler : sass::SassCompiler) -> Result<Option<Vec<sass::SassImportEntry>>, sass::SassImporterError> {
				
				let _path_0 = Path::new (_path_0.to_str () .or_panic (0xfd5fc0a2));
				let _path = self.parent.join (_path_0);
				
				for _extension in &["sass", "scss", "css"] {
					let _path = _path.with_extension (_extension);
					if ! _path.exists () {
						continue;
					}
					let _path = normalize_path (&_path);
					// eprintln! ("[dd] [d84fd5f5] {} -> {}", _path_0.display (), _path.display ());
					
					// NOTE:  The code bellow will keep a reference to `_path` that should live until after the compilation completes.
					let mut _resolved = self.resolved.borrow_mut ();
					_resolved.push (_path.into_boxed_path ());
					let _path = _resolved.last () .or_panic (0x5d9cba96) .as_ref ();
					let _entry = sass::SassImport::AbsolutePath (_path);
					
					return Ok (Some (vec! (_entry.into_sass_import_entry ())));
				}
				
				panic_with_format (0xe9c920d0, format_args! ("{}", _path.display ()));
			}
		}
		impl fmt::Debug for Importer {
			fn fmt (&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
				_formatter.write_str ("SassImporter()")
			}
		}
		
		let _resolved = rc::Rc::new (cell::RefCell::new (Vec::new ()));
		let _importer = Box::new (Importer { parent : _parent.into (), resolved : _resolved.clone () });
		
		let _options = sass::SassOptions {
				output_style : sass::OutputStyle::Expanded,
				source_comments : true,
				source_map_embed : false,
				source_map_contents : false,
				source_map_file_urls : false,
				omit_source_map_url : true,
				indent : ffi::CString::new ("\t") .infallible (0x77771198),
				linefeed : ffi::CString::new ("\n") .infallible (0xef2eea09),
				precision : 4,
				input_syntax : _input_syntax,
				include_paths : &[],
				function_list : rc::Rc::new (sass::SassFunctionList::new (Vec::new ())),
				importer_list : rc::Rc::new (sass::SassImporterList::new (vec! (_importer))),
				header_list : rc::Rc::new (sass::SassImporterList::new (Vec::new ())),
			};
		
		let _data = _options.compile_file (_source) .or_wrap (0xbbaffa6f) ?;
		
		for _dependency in _resolved.borrow () .iter () {
			self.dependencies_include (_dependency.as_ref ());
		}
		
		Ok (_data)
	}
}




#[ cfg (feature = "pulldown-cmark") ]
impl Builder {
	
	
	fn compile_markdown (&self, _source : &Path, _html_wrapper : bool, _title_detect : bool) -> BuilderResult<(Option<String>, String)> {
		
		let _input = fs::read_to_string (_source) ?;
		
		let mut _options = cmark::Options::empty ();
		_options.insert (cmark::Options::ENABLE_TABLES);
		_options.insert (cmark::Options::ENABLE_FOOTNOTES);
		_options.insert (cmark::Options::ENABLE_STRIKETHROUGH);
		_options.insert (cmark::Options::ENABLE_TASKLISTS);
		
		let mut _contents = String::with_capacity (_input.len () * 2);
		
		let mut _title = None;
		let mut _title_capture = None;
		if ! _title_detect {
			_title_capture = Some (false);
		}
		
		let _parser = cmark::Parser::new_ext (&_input, _options);
		
		let _parser = _parser.into_iter () .inspect (
				|_event| {
					match _event {
						cmark::Event::Start (cmark::Tag::Heading (1)) =>
							if _title_capture == None {
								_title_capture = Some (true);
							},
						cmark::Event::Text (_text) =>
							if _title_capture == Some (true) {
								if ! _text.is_empty () {
									_title = Some (_text.as_ref () .to_owned ());
								}
								_title_capture = Some (false);
							},
						_ =>
							(),
					}
				});
		
		cmark::html::push_html (&mut _contents, _parser);
		
		if ! _html_wrapper {
			return Ok ((_title, _contents));
		}
		
		let mut _output = String::with_capacity (_contents.len () + 1024);
		
		{
			_output.push_str ("<!DOCTYPE html>\n");
			_output.push_str ("<html>\n");
			_output.push_str ("<head>\n");
			let _title = _title.as_ref () .map (String::as_str) .unwrap_or ("");
			if ! _title.is_empty () {
				_output.push_str ("<title>");
				cmark::escape::escape_html (&mut _output, &_title) .infallible (0xdc5ea905);
				_output.push_str ("</title>\n");
			}
			_output.push_str (r#"<meta name="viewport" content="width=device-width, height=device-height" />"#);
			_output.push_str ("\n");
			_output.push_str ("</head>\n");
			_output.push_str ("<body>\n");
		}
		
		_output.push_str (&_contents);
		
		{
			_output.push_str ("</body>\n");
			_output.push_str ("</html>\n");
		}
		
		Ok ((_title, _output))
	}
}




fn create_file_from_str (_path : &Path, _data : &str) -> BuilderResult {
	
	fs::create_dir_all (_path.parent () .or_wrap (0x370af23d) ?) ?;
	
	let mut _file = fs::File::create (&_path) ?;
	_file.write_all (_data.as_bytes ()) ?;
	
	Ok (())
}




pub trait RoutePathBuilder {
	
	fn build (&self, _source_relative : &Path, _source_path : &Path, _route_prefix_hint : Option<&Path>, _route_infix_hint : Option<&Path>) -> BuilderResult<PathBuf>;
}


impl RoutePathBuilder for () {
	
	fn build (&self, _source_relative : &Path, _source_path : &Path, _route_prefix_hint : Option<&Path>, _route_infix_hint : Option<&Path>) -> BuilderResult<PathBuf> {
		generate_route (_source_relative, _route_prefix_hint, _route_infix_hint)
	}
}


impl RoutePathBuilder for (bool, &str) {
	
	fn build (&self, _source_relative : &Path, _source_path : &Path, _route_prefix_hint : Option<&Path>, _route_infix_hint : Option<&Path>) -> BuilderResult<PathBuf> {
		if self.0 {
			generate_route (_source_relative, Some (Path::new (self.1)), None)
		} else {
			normalize_route (Path::new (self.1), true, false)
		}
	}
}




pub trait RouteExtensionsBuilder {
	
	fn build (&self) -> BuilderResult<proc_macro2::TokenTree>;
}


impl RouteExtensionsBuilder for () {
	
	fn build (&self) -> BuilderResult<proc_macro2::TokenTree> {
		Ok (proc_macro2::Group::new (proc_macro2::Delimiter::Parenthesis, proc_macro2::TokenStream::new ()) .into ())
	}
}


impl RouteExtensionsBuilder for str {
	
	fn build (&self) -> BuilderResult<proc_macro2::TokenTree> {
		token_tree_parse (self)
	}
}




fn generate_route (_source_relative : &Path, _route_prefix : Option<&Path>, _route_infix : Option<&Path>) -> BuilderResult<PathBuf> {
	
	let _route_prefix = _route_prefix.or_wrap (0x1ba00780) ?;
	
	if ! _route_prefix.starts_with ("/") || (_route_prefix.ends_with ("/") && _route_prefix != Path::new ("/")) {
		return Err (error_with_code (0x6fc9256c));
	}
	if ! _source_relative.starts_with ("/") || _source_relative.ends_with ("/") {
		return Err (error_with_code (0xace09af4));
	}
	if let Some (_route_infix) = _route_infix {
		if _route_infix.starts_with ("/") || _route_infix.ends_with ("/") {
			return Err (error_with_code (0xd224b592));
		}
	}
	
	let _source_relative = _source_relative.strip_prefix ("/") .or_wrap (0xbd4b80bd) ?;
	
	let _route = if let Some (_route_infix) = _route_infix {
		let _route_infix = _route_infix.strip_prefix ("/") .or_wrap (0x1a7e3353) ?;
		_route_prefix.join (_route_infix) .join (_source_relative)
	} else {
		_route_prefix.join (_source_relative)
	};
	
	normalize_route (&_route, false, false)
}


fn normalize_route (_path_0 : &Path, _keep_trailing_slash : bool, _force_trailing_slash : bool) -> BuilderResult<PathBuf> {
	
	if ! _path_0.starts_with ("/") {
		return Err (error_with_code (0x1e7f7bc0));
	}
	
	let mut _path = PathBuf::new ();
	_path.push ("/");
	for _component in _path_0.components () {
		_path.push (_component);
	}
	
	if (_keep_trailing_slash || _force_trailing_slash) && (_path != Path::new ("/")) {
		if _path_0.to_string_lossy () .ends_with ("/") || _force_trailing_slash {
			let mut _path_1 = _path.clone () .into_os_string ();
			_path_1.push ("/");
			_path = _path_1.into ();
		}
	}
	
	Ok (_path)
}


fn normalize_path (_path_0 : &Path) -> BuilderResult<PathBuf> {
	
	let mut _path = PathBuf::new ();
	for _component in _path_0.components () {
		_path.push (_component);
	}
	
	Ok (_path)
}




fn detect_content_type_from_extension (_source : &Path) -> BuilderResult<&'static str> {
	
	let _extension = _source.extension () .or_wrap (0x29957dc8) ? .to_str () .or_wrap (0x908aeea6) ?;
	
	let _content_type = match _extension {
		"text" | "txt" => "Text",
		"md" => "Text",
		"html" | "htm" => "Html",
		"css" => "Css",
		"js" => "Js",
		"json" => "Json",
		"xml" => "Xml",
		"png" => "Png",
		"jpeg" | "jpg" => "Jpeg",
		"ico" => "Icon",
		"svg" => "Svg",
		"ttf" => "FontTtf",
		"otf" => "FontOtf",
		"woff" => "FontWoff",
		"woff2" => "FontWoff2",
		_ =>
			return Err (error_with_format (0x2bd15bab, format_args! ("{}", _source.display ()))),
	};
	
	Ok (_content_type)
}




fn token_tree_parse (_string : &str) -> BuilderResult<proc_macro2::TokenTree> {
	let _stream = proc_macro2::TokenStream::from_str (_string) .or_wrap (0x72524db6) ?;
	let mut _stream = _stream.into_iter ();
	let _token = if let Some (_token) = _stream.next () {
		_token
	} else {
		return Err (error_with_code (0xe9c8879a))
	};
	if _stream.next () .is_some () {
		return Err (error_with_code (0xd96714a4))
	}
	Ok (_token)
}




#[ allow (dead_code) ]
fn fingerprint_data (_data : impl AsRef<[u8]>) -> String {
	use blake2::Digest as _;
	let mut _hasher = blake2::Blake2b::new ();
	_hasher.update (_data.as_ref ());
	let _hash = _hasher.finalize ();
	format! ("{:x}", _hash)
}






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


use crate::errors::*;

pub use crate::errors::{BuilderError, BuilderResult};




#[ derive (Clone, Debug) ]
pub struct BuilderConfiguration {
	
	pub sources : Option<PathBuf>,
	#[ cfg (feature = "builder-assets") ]
	pub assets_sources : Option<PathBuf>,
	#[ cfg (feature = "builder-askama") ]
	pub templates_sources : Option<PathBuf>,
	#[ cfg (feature = "builder-markdown") ]
	pub markdowns_sources : Option<PathBuf>,
	
	#[ cfg (feature = "builder-assets") ]
	pub assets_route_base : Option<PathBuf>,
	
	#[ cfg (feature = "builder-assets") ]
	pub css_route_base : Option<PathBuf>,
	#[ cfg (feature = "builder-assets") ]
	pub js_route_base : Option<PathBuf>,
	
	#[ cfg (feature = "builder-assets") ]
	pub images_route_base : Option<PathBuf>,
	#[ cfg (feature = "builder-assets") ]
	pub icons_route_base : Option<PathBuf>,
	#[ cfg (feature = "builder-assets") ]
	pub favicons_route_base : Option<PathBuf>,
	#[ cfg (feature = "builder-assets") ]
	pub fonts_route_base : Option<PathBuf>,
	
	pub outputs : PathBuf,
	pub generated : PathBuf,
}


impl Default for BuilderConfiguration {
	
	fn default () -> Self {
		
		let _sources = Self::resolve_sources () .else_panic (0xc6e7914d);
		let _outputs = Self::resolve_outputs () .else_panic (0x344e2c9e);
		
		#[ cfg (feature = "builder-assets") ]
		let _assets_sources = _sources.join ("./assets");
		#[ cfg (feature = "builder-assets") ]
		let _assets_sources = if _assets_sources.exists () { Some (_assets_sources) } else { None };
		
		#[ cfg (feature = "builder-askama") ]
		let _templates_sources = _sources.join ("./templates");
		#[ cfg (feature = "builder-askama") ]
		let _templates_sources = if _templates_sources.exists () { Some (_templates_sources) } else { None };
		
		#[ cfg (feature = "builder-markdown") ]
		let _markdowns_sources = _sources.join ("./markdown");
		#[ cfg (feature = "builder-markdown") ]
		let _markdowns_sources = if _markdowns_sources.exists () { Some (_markdowns_sources) } else { None };
		
		let _generated = _outputs.join ("./hss-builder-generated-default.in");
		
		#[ cfg (feature = "builder-assets") ]
		let _assets_route_base = Some (PathBuf::from ("/assets"));
		
		#[ cfg (feature = "builder-assets") ]
		let _css_route_base = Some (PathBuf::from ("/assets/css"));
		#[ cfg (feature = "builder-assets") ]
		let _js_route_base = Some (PathBuf::from ("/assets/js"));
		
		#[ cfg (feature = "builder-assets") ]
		let _images_route_base = Some (PathBuf::from ("/assets/images"));
		#[ cfg (feature = "builder-assets") ]
		let _icons_route_base = Some (PathBuf::from ("/assets/icons"));
		#[ cfg (feature = "builder-assets") ]
		let _favicons_route_base = Some (PathBuf::from ("/assets/favicons"));
		#[ cfg (feature = "builder-assets") ]
		let _fonts_route_base = Some (PathBuf::from ("/assets/fonts"));
		
		Self {
				
				sources : Some (_sources),
				#[ cfg (feature = "builder-assets") ]
				assets_sources : _assets_sources,
				#[ cfg (feature = "builder-askama") ]
				templates_sources : _templates_sources,
				#[ cfg (feature = "builder-markdown") ]
				markdowns_sources : _markdowns_sources,
				
				#[ cfg (feature = "builder-assets") ]
				assets_route_base : _assets_route_base,
				
				#[ cfg (feature = "builder-assets") ]
				css_route_base : _css_route_base,
				#[ cfg (feature = "builder-assets") ]
				js_route_base : _js_route_base,
				
				#[ cfg (feature = "builder-assets") ]
				images_route_base : _images_route_base,
				#[ cfg (feature = "builder-assets") ]
				icons_route_base : _icons_route_base,
				#[ cfg (feature = "builder-assets") ]
				favicons_route_base : _favicons_route_base,
				#[ cfg (feature = "builder-assets") ]
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
				
				#[ cfg (feature = "builder-assets") ]
				assets_sources : None,
				#[ cfg (feature = "builder-askama") ]
				templates_sources : None,
				#[ cfg (feature = "builder-markdown") ]
				markdowns_sources : None,
				
				#[ cfg (feature = "builder-assets") ]
				assets_route_base : None,
				
				#[ cfg (feature = "builder-assets") ]
				css_route_base : None,
				#[ cfg (feature = "builder-assets") ]
				js_route_base : None,
				
				#[ cfg (feature = "builder-assets") ]
				images_route_base : None,
				#[ cfg (feature = "builder-assets") ]
				icons_route_base : None,
				#[ cfg (feature = "builder-assets") ]
				favicons_route_base : None,
				#[ cfg (feature = "builder-assets") ]
				fonts_route_base : None,
				
				outputs : _outputs,
				generated : _generated,
			};
		
		Ok (_builder)
	}
	
	pub fn resolve_sources () -> BuilderResult<PathBuf> {
		let _sources = PathBuf::from (env::var ("CARGO_MANIFEST_DIR") .else_wrap (0x4c6c04d8) ?);
		if ! _sources.is_dir () {
			fail! (0x148bc689);
		}
		normalize_path (&_sources)
	}
	
	pub fn resolve_outputs () -> BuilderResult<PathBuf> {
		let _outputs = PathBuf::from (env::var ("OUT_DIR") .else_wrap (0xf039039f) ?);
		if ! _outputs.is_dir () {
			fail! (0xfcee6b4d);
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
				counter : 1,
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
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include (&_source) ?;
		
		let _mode = "auto";
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, {}, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _mode, _source, _description) .infallible (0x5fa962ac);
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0x46de4cc9);
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "builder-askama") ]
	pub fn route_askama (&mut self, _source_0 : &str, _context : Option<(&str, Option<&str>)>, _trait : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_templates_sources, _source_0) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		self.route_askama_0 (&_relative, &_source, _context, _trait, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	#[ cfg (feature = "builder-askama") ]
	pub fn route_askamas (&mut self, _sources_0 : &str, _glob : Option<&str>, _context : Option<(&str, Option<&str>)>, _trait : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_templates_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_askama_0 (&_relative, &_source, _context, _trait, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-askama") ]
	fn route_askama_0 (&mut self, _relative : &Path, _source : &Path, _context : Option<(&str, Option<&str>)>, _trait : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _relative_1 = _relative.with_extension ("");
		
		let _template = _relative.strip_prefix ("/") .infallible (0x7285dc26);
		
		let _route = _route_builder.build (&_relative_1, &_source, _route_base, None) ?;
		let _extensions = _extensions_builder.build () ?;
		
		let _id = self.generate_id ();
		
		let _content_type = "html";
		let _description = if let Some (_relative) = _source_relative {
			format! ("askama ({}, from = `{}`, file = `...{}`)", _content_type, _source_0, _relative.display ())
		} else {
			format! ("askama ({}, file = `{}`)", _content_type, _source_0)
		};
		
		self.route_names.push (format! ("Route_{}", _id));
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include (&_source) ?;
		
		let (_context_type, _context_path) = _context.unwrap_or (("!", None));
		let _trait_type = _trait.unwrap_or ("!");
		
		if let Some (_context_path) = _context_path {
			
			let _sources = self.configuration.sources.as_ref () .map (PathBuf::as_path);
			let (_, _context_path) = self.resolve_file (None, _context_path) ?;
			let _context_encoding = match _context_path.extension () .else_wrap (0x70d90b37) ? .to_str () .else_wrap (0xa22f0541) ? {
				"toml" => "toml",
				"yaml" => "yaml",
				"json" => "json",
				_ =>
					fail! (0xcbc42494, "{}", _context_path.display ()),
			};
			#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
			self.dependencies_include (&_context_path) ?;
			
			writeln! (self.generated, "::hyper_static_server::askama! (Resource_{}, Template_{}, {{ type : {}, deserialize : ({:?}, {:?}) }}, {{ trait : {} }}, {}, {:?}, {:?});", _id, _id, _context_type, _context_encoding, _context_path, _trait_type, _content_type, _template, _description) .infallible (0x3258a4c6);
		} else {
			writeln! (self.generated, "::hyper_static_server::askama! (Resource_{}, Template_{}, {{ type : {} }}, {{ trait : {} }}, {}, {:?}, {:?});", _id, _id, _context_type, _trait_type, _content_type, _template, _description) .infallible (0x35966385);
		}
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0x41a5ee4c);
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-askama") ]
	pub fn watch_askama (&mut self, _source : &str) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_templates_sources, _source) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include (&_source) ?;
		
		Ok (())
	}
	
	#[ cfg (feature = "builder-askama") ]
	pub fn watch_askamas (&mut self, _sources : &str, _glob : Option<&str>) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_templates_sources, _sources, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_files.iter () .map (|_pair| _pair.1.as_path ())) ?;
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "builder-askama") ]
	#[ cfg (feature = "builder-markdown") ]
	pub fn route_markdown_askama (&mut self, _source_markdown_0 : &str, _source_template_0 : &str, _context : Option<&str>, _trait : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative_template, _source_template) = self.resolve_file (_templates_sources, _source_template_0) ?;
		
		let _template = _relative_template.strip_prefix ("/") .infallible (0xda5e5ad4);
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include (&_source_template) ?;
		
		let _markdowns_sources = self.configuration.markdowns_sources.as_ref () .map (PathBuf::as_path);
		let (_relative_markdown, _source_markdown) = self.resolve_file (_markdowns_sources, _source_markdown_0) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		self.route_markdown_askama_0 (&_relative_markdown, &_source_markdown, &_template, _context, _trait, _route_base, _route_builder, _extensions_builder, _source_markdown_0, None)
	}
	
	#[ cfg (feature = "builder-askama") ]
	#[ cfg (feature = "builder-markdown") ]
	pub fn route_markdowns_askama (&mut self, _sources_markdown_0 : &str, _glob : Option<&str>, _source_template_0 : &str, _context : Option<&str>, _trait : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative_template, _source_template) = self.resolve_file (_templates_sources, _source_template_0) ?;
		
		let _template = _relative_template.strip_prefix ("/") .infallible (0xe0168bd3);
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include (&_source_template) ?;
		
		let _markdowns_sources = self.configuration.markdowns_sources.as_ref () .map (PathBuf::as_path);
		let (_files_markdown, _folders_markdown) = self.resolve_files (_markdowns_sources, _sources_markdown_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders_markdown.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		for (_relative_markdown, _source_markdown) in _files_markdown.into_iter () {
			
			self.route_markdown_askama_0 (&_relative_markdown, &_source_markdown, &_template, _context, _trait, _route_base, _route_builder, _extensions_builder, _sources_markdown_0, Some (_relative_markdown.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-askama") ]
	#[ cfg (feature = "builder-markdown") ]
	fn route_markdown_askama_0 (&mut self, _relative_markdown : &Path, _source_markdown : &Path, _template : &Path, _context : Option<&str>, _trait : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _relative_markdown_1 = _relative_markdown.with_extension ("");
		
		let (_output_body, _output_title, _output_metadata, _output_frontmatter, _refresher) = if cfg! (any (not (feature = "builder-relaxed-dependencies"), not (feature = "builder-markdown-dynamic"), not (feature = "builder-askama-dynamic"), feature = "production")) {
			
			self.dependencies_include (_source_markdown) ?;
			
			let _markdown = self.compile_markdown_body (_source_markdown, true) ?;
			let _markdown_body = _markdown.body;
			let _markdown_metadata = _markdown.metadata;
			let _markdown_frontmatter = _markdown.frontmatter;
			
			let _markdown_title = _markdown_metadata.title.as_ref () .map (String::as_str) .unwrap_or ("");
			
			let _output_title = self.configuration.outputs.join (fingerprint_data (&_markdown_title)) .with_extension ("txt");
			create_file_from_str (&_output_title, _markdown_title, true, true) ?;
			
			let _output_body = self.configuration.outputs.join (fingerprint_data (&_markdown_body)) .with_extension ("html");
			create_file_from_str (&_output_body, &_markdown_body, true, true) ?;
			
			let _output_metadata = self.configuration.outputs.join (fingerprint_data (&_markdown_body)) .with_extension ("meta");
			let _markdown_metadata = ::serde_json::to_string_pretty (&_markdown_metadata) .else_wrap (0x94fe158d) ?;
			create_file_from_str (&_output_metadata, &_markdown_metadata, true, true) ?;
			
			let _output_frontmatter = if let Some (_frontmatter) = _markdown_frontmatter {
				let _encoding = _frontmatter.encoding.as_ref ();
				let _data = _frontmatter.data.as_ref ();
				let _extension = match _encoding {
					"toml" => "toml",
					"yaml" => "yaml",
					"json" => "json",
					_ =>
						fail! (0x75f4427f, "{}", _encoding),
				};
				let _path = self.configuration.outputs.join (fingerprint_data (&_data)) .with_extension (_extension);
				create_file_from_str (&_path, &_data, true, true) ?;
				Some ((_extension, _path))
			} else {
				None
			};
			
			(_output_body, _output_title, _output_metadata, _output_frontmatter, false)
			
		} else {
			
			let _token = fingerprint_data (_source_markdown.to_string_lossy () .as_bytes ());
			
			let _output_body = self.configuration.outputs.join (&_token) .with_extension ("html");
			
			let _output_title = self.configuration.outputs.join (&_token) .with_extension ("txt");
			
			let _output_metadata = self.configuration.outputs.join (&_token) .with_extension ("meta");
			
			let _output_frontmatter = self.configuration.outputs.join (&_token) .with_extension ("data");
			
			(_output_body, _output_title, _output_metadata, Some (("auto", _output_frontmatter)), true)
		};
		
		let _route = _route_builder.build (&_relative_markdown_1, _source_markdown, _route_base, None) ?;
		let _extensions = _extensions_builder.build () ?;
		
		let _id = self.generate_id ();
		
		let _content_type = "html";
		let _description = if let Some (_relative) = _source_relative {
			format! ("markdown_askama ({}, from = `{}`, file = `...{}`)", _content_type, _source_0, _relative.display ())
		} else {
			format! ("markdown_askama ({}, file = `{}`)", _content_type, _source_0)
		};
		
		self.route_names.push (format! ("Route_{}", _id));
		
		let _refresher_code = if _refresher {
			let (_context_encoding, _context_path) = _output_frontmatter.as_ref () .else_panic (0xc27ad812);
			writeln! (self.generated, "::hyper_static_server::resource_markdown_refresher! (Refresher_{}, (relative_to_cwd, {:?}), (relative_to_cwd, {:?}), (relative_to_cwd, {:?}), (relative_to_cwd, {:?}), (relative_to_cwd, {:?}));", _id, _source_markdown, _output_body, _output_title, _output_metadata, _context_path) .infallible (0x2b54879e);
			format! ("Refresher_{},", _id)
		} else {
			String::new ()
		};
		
		let _trait_type = _trait.unwrap_or ("!");
		
		if let Some ((_context_encoding, _context_path)) = _output_frontmatter {
			if let Some (_context_type) = _context {
				writeln! (self.generated, "::hyper_static_server::askama_document! (Resource_{}, Template_{}, {{ type : {}, deserialize : ({:?}, {:?}) }}, {{ trait : {} }}, {}, {:?}, {:?}, {:?}, {:?}, {} {:?});", _id, _id, _context_type, _context_encoding, _context_path, _trait_type, _content_type, _template, _output_body, _output_title, _output_metadata, _refresher_code, _description) .infallible (0xed0b221b);
			} else {
				let _context_type = _context.unwrap_or ("!");
				writeln! (self.generated, "::hyper_static_server::askama_document! (Resource_{}, Template_{}, {{ type : {} }}, {{ trait : {} }}, {}, {:?}, {:?}, {:?}, {:?}, {} {:?});", _id, _id, _context_type, _trait_type, _content_type, _template, _output_body, _output_title, _output_metadata, _refresher_code, _description) .infallible (0xf02b2615);
			}
		} else {
			let _context_type = _context.unwrap_or ("!");
			writeln! (self.generated, "::hyper_static_server::askama_document! (Resource_{}, Template_{}, {{ type : {} }}, {{ trait : {} }}, {}, {:?}, {:?}, {:?}, {:?}, {} {:?});", _id, _id, _context_type, _trait_type, _content_type, _template, _output_body, _output_title, _output_metadata, _refresher_code, _description) .infallible (0xd64341cb);
		}
		
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0xafb30ea0);
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "builder-markdown") ]
	pub fn route_markdown (&mut self, _source_0 : &str, _header_source : Option<&str>, _footer_source : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _markdowns_sources = self.configuration.markdowns_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_markdowns_sources, _source_0) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		self.route_markdown_0 (&_relative, &_source, _header_source, _footer_source, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	#[ cfg (feature = "builder-markdown") ]
	pub fn route_markdowns (&mut self, _sources_0 : &str, _glob : Option<&str>, _header_source : Option<&str>, _footer_source : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _markdowns_sources = self.configuration.markdowns_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_markdowns_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = Some (Path::new ("/"));
		
		for (_relative, _source) in _files.into_iter () {
			
			if _source.extension () .else_wrap (0xc1ecda55) ? != "md" {
				fail! (0x393ea45d, "{}", _source.display ());
			}
			
			self.route_markdown_0 (&_relative, &_source, _header_source, _footer_source, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-markdown") ]
	fn route_markdown_0 (&mut self, _relative : &Path, _source : &Path, _header_source : Option<&str>, _footer_source : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _markdowns_sources = self.configuration.markdowns_sources.as_ref () .map (PathBuf::as_path);
		let _header_source = _header_source.map (|_source| BuilderResult::Ok (self.resolve_file (_markdowns_sources, _source) ? .1)) .transpose () ?;
		let _footer_source = _footer_source.map (|_source| BuilderResult::Ok (self.resolve_file (_markdowns_sources, _source) ? .1)) .transpose () ?;
		
		let _relative_1 = _relative.with_extension ("");
		
		if cfg! (any (not (feature = "builder-relaxed-dependencies"), not (feature = "builder-markdown-dynamic"), feature = "production")) {
			
			#[ cfg (any (not (feature = "builder-relaxed-dependencies"), not (feature = "builder-markdown-dynamic"), feature = "production")) ]
			{
				self.dependencies_include (&_source) ?;
				if let Some (_header_source) = &_header_source {
					self.dependencies_include (_header_source) ?;
				}
				if let Some (_footer_source) = &_footer_source {
					self.dependencies_include (_footer_source) ?;
				}
			}
			
			let _header_source = _header_source.as_ref () .map (PathBuf::as_path);
			let _footer_source = _footer_source.as_ref () .map (PathBuf::as_path);
			
			let _html_data = self.compile_markdown_html (_source, _header_source, _footer_source) ?;
			
			let _output = self.configuration.outputs.join (fingerprint_data (&_html_data)) .with_extension ("html");
			create_file_from_str (&_output, &_html_data, true, true) ?;
			
			// FIXME:  Here the second argument should be `_source`.
			self.route_asset_raw (&_relative_1, &_output, "html", _route_base, _route_builder, _extensions_builder, "markdown", _source_0, _source_relative) ?;
			
			self.dependencies_exclude (&_output) ?;
			
		} else {
			
			let _route = _route_builder.build (&_relative_1, &_source, _route_base, None) ?;
			let _extensions = _extensions_builder.build () ?;
			
			let _id = self.generate_id ();
			
			let _description = format! ("{} (file = `{}`)", "resource_markdown", _source_0);
			
			self.route_names.push (format! ("Route_{}", _id));
			
			writeln! (self.generated, "::hyper_static_server::resource_markdown_dynamic! (Resource_{}, {}, (relative_to_cwd, {:?}), (relative_to_cwd, {:?}), (relative_to_cwd, {:?}), {:?});", _id, "html", _source, _header_source, _footer_source, _description) .infallible (0x15f089dc);
			writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0xbf41dd16);
		}
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_css (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _css_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_css_sources, _source_0) ?;
		
		let _route_base = self.configuration.css_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_raw (&_relative, &_source, "css", _route_base, _route_builder, _extensions_builder, "resource_css", _source_0, None)
	}
	
	
	#[ cfg (feature = "builder-assets") ]
	#[ cfg (feature = "builder-assets-sass") ]
	pub fn route_sass (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _css_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_css_sources, _source_0) ?;
		
		let _relative_1 = _relative.with_extension ("css");
		
		let _route_base = self.configuration.css_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		if cfg! (any (not (feature = "builder-relaxed-dependencies"), not (feature = "builder-assets-sass-dynamic"), feature = "production")) {
			
			self.dependencies_include (&_source) ?;
			
			let _compiled = self.compile_sass (&_source) ?;
			
			let _output = self.configuration.outputs.join (fingerprint_data (&_compiled)) .with_extension ("css");
			create_file_from_str (&_output, &_compiled, true, true) ?;
			
			// FIXME:  Here the second argument should be `_source`.
			self.route_asset_raw (&_relative_1, &_output, "css", _route_base, _route_builder, _extensions_builder, "resource_sass", _source_0, None) ?;
			
			self.dependencies_exclude (&_output) ?;
			
		} else {
			
			let _route = _route_builder.build (&_relative_1, &_source, _route_base, None) ?;
			let _extensions = _extensions_builder.build () ?;
			
			let _id = self.generate_id ();
			
			let _description = format! ("{} (file = `{}`)", "resource_sass", _source_0);
			
			self.route_names.push (format! ("Route_{}", _id));
			
			writeln! (self.generated, "::hyper_static_server::resource_sass_dynamic! (Resource_{}, {}, (relative_to_cwd, {:?}), {:?});", _id, "css", _source, _description) .infallible (0xb7dd2208);
			writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?}, {});", _id, _id, _route, _extensions) .infallible (0x506e8636);
		}
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_js (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _js_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_js_sources, _source_0) ?;
		
		let _route_base = self.configuration.js_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_raw (&_relative, &_source, "js", _route_base, _route_builder, _extensions_builder, "resource_js", _source_0, None)
	}
	
	
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_image (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_image", _source_0, None)
	}
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_images (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_image", _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_icon (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.icons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_icon", _source_0, None)
	}
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_icons (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.icons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_icon", _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_favicon (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_favicon", _source_0, None)
	}
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_favicons (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, "resource_favicon", _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-assets") ]
	fn route_image_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _macro : &str, _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _content_type = detect_content_type_from_extension (&_source) ?;
		match _content_type {
			"png" | "jpeg" | "icon" | "svg" =>
				(),
			_ =>
				fail! (0x0fd2d804, "{}", _source.display ()),
		};
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _extensions_builder, _macro, _source_0, _source_relative)
	}
	
	
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_font (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_font_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_fonts (&mut self, _sources_0 : &str, _glob : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_font_0 (&_relative, &_source, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-assets") ]
	fn route_font_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _content_type = detect_content_type_from_extension (&_source) ?;
		match _content_type {
			"font_ttf" | "font_otf" | "font_woff" | "font_woff2" =>
				(),
			_ =>
				fail! (0x1a4ccbf4, "{}", _source.display ()),
		};
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _extensions_builder, "resource_font", _source_0, _source_relative)
	}
	
	
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_asset (&mut self, _source_0 : &str, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) ?;
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_0 (&_relative, &_source, _content_type, _route_base, _route_builder, _extensions_builder, _source_0, None)
	}
	
	#[ cfg (feature = "builder-assets") ]
	pub fn route_assets (&mut self, _sources_0 : &str, _glob : Option<&str>, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_asset_0 (&_relative, &_source, _content_type, _route_base, _route_builder, _extensions_builder, _sources_0, Some (_relative.as_path ())) ?;
		}
		
		Ok (())
	}
	
	
	#[ cfg (feature = "builder-assets") ]
	fn route_asset_0 (&mut self, _relative : &Path, _source : &Path, _content_type : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized), _source_0 : &str, _source_relative : Option<&Path>) -> BuilderResult {
		
		let _content_type = _content_type.map_or_else (|| detect_content_type_from_extension (&_source), |_content_type| Ok (_content_type)) ?;
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _extensions_builder, "resource_asset", _source_0, _source_relative)
	}
	
	
	
	
	#[ cfg (feature = "builder-sitemaps") ]
	pub fn route_sitemap (&mut self, _prefix : &str, _format : &str, _route_builder : &(impl RoutePathBuilder + ?Sized), _extensions_builder : &(impl RouteExtensionsBuilder + ?Sized)) -> BuilderResult {
		
		let _route = _route_builder.build (Path::new (""), Path::new (""), None, None) ?;
		let _extensions = _extensions_builder.build () ?;
		let _format = token_tree_parse (_format) ?;
		
		let _id = self.generate_id ();
		
		self.route_names.push (format! ("Route_{}", _id));
		
		writeln! (self.generated, "::hyper_static_server::route_sitemap! (Route_{}, {:?}, {:?}, {}, {});", _id, _route, _prefix, _format, _extensions) .infallible (0x5f529a53);
		
		Ok (())
	}
	
	
	
	
	#[ cfg (feature = "builder-assets") ]
	pub fn watch_asset (&mut self, _source : &str) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include (&_source) ?;
		
		Ok (())
	}
	
	#[ cfg (feature = "builder-assets") ]
	pub fn watch_assets (&mut self, _sources : &str, _glob : Option<&str>) -> BuilderResult {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources, _glob) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path)) ?;
		
		#[ cfg (any (not (feature = "builder-relaxed-dependencies"), feature = "production")) ]
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
		
		create_file_from_str (&self.configuration.generated, &self.generated, false, true) ?;
		
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
			fail! (0x039d945b, "{}", _path.display ());
		}
		
		self.resolve_relative_and_path (&_path, &_relative_root)
	}
	
	
	fn resolve_files (&self, _root : Option<&Path>, _sources : &str, _glob : Option<&str>) -> BuilderResult<(Vec<(PathBuf, PathBuf)>, Vec<PathBuf>)> {
		
		let (_root, _relative_root) = self.resolve_source (_root, _sources, false) ?;
		
		if ! _root.is_dir () {
			fail! (0x621693a6);
		}
		
		let _glob = _glob.map (|_pattern| globset::Glob::new (_pattern) .else_wrap (0xf68023ce)) .transpose () ?;
		let _glob = _glob.map (|_pattern| _pattern.compile_matcher ());
		
		let mut _files = Vec::new ();
		let mut _folders = Vec::new ();
		
		let _walker = walkdir::WalkDir::new (&_root)
				.follow_links (true)
				.sort_by (|_left, _right| ffi::OsStr::cmp (_left.file_name (), _right.file_name ()));
		
		for _entry in _walker {
			let _entry = _entry.else_wrap (0xdeed2955) ?;
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
			let _root = _root.else_wrap (0x6e3319c9) ?;
			if _source != "_" {
				_root.join (&_source[2..])
			} else {
				_root.to_owned ()
			}
			
		} else if _source.starts_with ("./") || _source.starts_with ("..") || (_source == ".") || (_source == "..") {
			let _root = self.configuration.sources.as_ref () .else_wrap (0x0791a9b4) ?;
			_root.join (&_source)
			
		} else if _source.starts_with (">") {
			PathBuf::from (&_source[1..])
			
		} else {
			fail! (0x41071330);
		};
		
		if ! _path.exists () {
			fail! (0x1086bd9d, "{}", _path.display ());
		}
		
		if _name_only {
			let _relative_root = _path.parent () .else_wrap (0x067a2cad) ? .to_path_buf ();
			Ok ((_path, _relative_root))
		} else {
			Ok ((_path.clone (), _path))
		}
	}
	
	
	fn resolve_relative_and_path (&self, _path : &Path, _relative_root : &Path) -> BuilderResult<(PathBuf, PathBuf)> {
		
		let _relative = _path.strip_prefix (_relative_root) .else_wrap (0x546e7cd9) ? .to_str () .else_wrap (0xa48f283c) ?;
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
	
	fn dependencies_exclude (&mut self, _path : &Path) -> BuilderResult {
		
		self.dependencies.remove (_path.into ());
		
		Ok (())
	}
	
	fn dependencies_extend (&mut self) -> BuilderResult {
		
		let mut _extra = Vec::new ();
		
		for _dependency in self.dependencies.iter () {
			
			let _metadata = fs::symlink_metadata (_dependency) .else_wrap (0x06a4fbd5) ?;
			
			if _metadata.file_type () .is_symlink () {
				let _target = _dependency.canonicalize () .else_wrap (0x8df4310e) ?;
				_extra.push (_target);
			}
		}
		
		for _dependency in _extra.into_iter () {
			self.dependencies.insert (_dependency);
		}
		
		self.dependencies.insert (PathBuf::from (file! ()));
		
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




#[ cfg (feature = "builder-assets-sass") ]
impl Builder {
	
	
	fn compile_sass (&mut self, _source : &Path) -> BuilderResult<String> {
		crate::support_sass::compile_sass (_source)
	}
}




#[ cfg (feature = "builder-markdown") ]
impl Builder {
	
	fn compile_markdown_body (&self, _source : &Path, _title_detect : bool) -> BuilderResult<crate::support_markdown::MarkdownOutput> {
		let mut _options = crate::support_markdown::MarkdownOptions::default ();
		_options.title_detect = _title_detect;
		crate::support_markdown::compile_markdown_from_path (_source, Some (&_options))
	}
	
	fn compile_markdown_html (&self, _source : &Path, _header : Option<&Path>, _footer : Option<&Path>) -> BuilderResult<String> {
		crate::support_markdown::compile_markdown_html_from_path (_source, _header, _footer, None)
	}
}




fn create_file_from_str (_path : &Path, _data : &str, _skip_if_exists : bool, _skip_if_same : bool) -> BuilderResult {
	
	fs::create_dir_all (_path.parent () .else_wrap (0x370af23d) ?) .else_wrap (0xaba3f86c) ?;
	
	let _metadata = match fs::symlink_metadata (_path) {
		Ok (_metadata) =>
			if _metadata.is_file () {
				Some (_metadata)
			} else {
				fail! (0x7b58f13c, "{}", _path.display ());
			}
		Err (_error) if _error.kind () == io::ErrorKind::NotFound =>
			None,
		Err (_error) =>
			fail! (0xb30cd904, cause => _error),
	};
	if _skip_if_exists && _metadata.is_some () {
		return Ok (());
	}
	if _skip_if_same && _metadata.is_some () {
		let _data_old = fs::read (_path) .else_wrap (0x6c311b95) ?;
		let _data_old_fingerprint = fingerprint_data (_data_old);
		let _data_new_fingerprint = fingerprint_data (_data);
		if _data_old_fingerprint == _data_new_fingerprint {
			return Ok (());
		}
	}
	
	// FIXME:  Use temporary file then rename!
	let mut _file = fs::File::create (_path) .else_wrap (0xbb2a285c) ?;
	_file.write_all (_data.as_bytes ()) .else_wrap (0x3f841686) ?;
	
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
	
	let _route_prefix = _route_prefix.else_wrap (0x1ba00780) ?;
	
	if ! _route_prefix.starts_with ("/") || (_route_prefix.ends_with ("/") && _route_prefix != Path::new ("/")) {
		fail! (0x6fc9256c);
	}
	if ! _source_relative.starts_with ("/") || _source_relative.ends_with ("/") {
		fail! (0xace09af4);
	}
	if let Some (_route_infix) = _route_infix {
		if _route_infix.starts_with ("/") || _route_infix.ends_with ("/") {
			fail! (0xd224b592);
		}
	}
	
	let _source_relative = _source_relative.strip_prefix ("/") .else_wrap (0xbd4b80bd) ?;
	
	let _route = if let Some (_route_infix) = _route_infix {
		let _route_infix = _route_infix.strip_prefix ("/") .else_wrap (0x1a7e3353) ?;
		_route_prefix.join (_route_infix) .join (_source_relative)
	} else {
		_route_prefix.join (_source_relative)
	};
	
	normalize_route (&_route, false, false)
}


fn normalize_route (_path_0 : &Path, _keep_trailing_slash : bool, _force_trailing_slash : bool) -> BuilderResult<PathBuf> {
	
	if ! _path_0.starts_with ("/") {
		fail! (0x1e7f7bc0);
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
	
	let _extension = _source.extension () .else_wrap (0x29957dc8) ? .to_str () .else_wrap (0x908aeea6) ?;
	
	let _content_type = match _extension {
		"text" | "txt" => "text",
		"md" => "text",
		"html" | "htm" => "html",
		"css" => "css",
		"js" => "js",
		"json" => "json",
		"xml" => "xml",
		"png" => "png",
		"jpeg" | "jpg" => "jpeg",
		"ico" => "icon",
		"svg" => "svg",
		"ttf" => "font_ttf",
		"otf" => "font_otf",
		"woff" => "font_woff",
		"woff2" => "font_woff2",
		_ => "unknown",
	};
	
	Ok (_content_type)
}




fn token_tree_parse (_string : &str) -> BuilderResult<proc_macro2::TokenTree> {
	let _stream = proc_macro2::TokenStream::from_str (_string) .else_replace (0x72524db6) ?;
	let mut _stream = _stream.into_iter ();
	let _token = if let Some (_token) = _stream.next () {
		_token
	} else {
		fail! (0xe9c8879a);
	};
	if _stream.next () .is_some () {
		fail! (0xd96714a4);
	}
	Ok (_token)
}




fn fingerprint_data (_data : impl AsRef<[u8]>) -> String {
	use blake2::Digest as _;
	let mut _hasher = blake2::Blake2b::new ();
	_hasher.update (_data.as_ref ());
	let _hash = _hasher.finalize ();
	format! ("{:x}", _hash)
}






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
		
		fmt::{Write as _},
		io::{Write as _},
		
	};


use ::walkdir;
use ::blake2;

#[ cfg (feature = "sass-rs") ]
use ::sass_rs as sass;

#[ cfg (feature = "sass-alt") ]
use ::sass_alt as sass;




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
		
		let _sources = Self::resolve_sources ();
		let _outputs = Self::resolve_outputs ();
		
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
	
	pub fn minimal () -> Self {
		
		let _sources = Self::resolve_sources ();
		let _outputs = Self::resolve_outputs ();
		let _generated = _outputs.join ("./hss-builder-generated-default.in");
		
		Self {
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
			}
		
	}
	
	pub fn resolve_sources () -> PathBuf {
		let _sources = PathBuf::from (env::var ("CARGO_MANIFEST_DIR") .expect ("[4c6c04d8]"));
		if ! _sources.is_dir () {
			panic! ("[148bc689]");
		}
		normalize_path (&_sources)
	}
	
	pub fn resolve_outputs () -> PathBuf {
		let _outputs = PathBuf::from (env::var ("OUT_DIR") .expect ("[f039039f]"));
		if ! _outputs.is_dir () {
			panic! ("[fcee6b4d]");
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
	
	
	
	
	fn route_asset_raw (&mut self, _relative : &Path, _source : &Path, _content_type : &str, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _macro : &str, _source_0 : &str, _source_relative : bool) {
		
		let _route = _route_builder.build (_relative, &_source, _route_base, None);
		
		let _id = self.generate_id ();
		
		let _description = if _source_relative {
			format! ("{} ({}, from = `{}`, file = `...{}`)", _macro, _content_type, _source_0, _relative.display ())
		} else {
			format! ("{} ({}, file = `{}`)", _macro, _content_type, _source_0)
		};
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies_include (&_source);
		
		let _mode = "auto";
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, {}, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _mode, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_askama (&mut self, _source_0 : &str, _route : &str) -> () {
		
		let _templates_sources = self.configuration.templates_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_templates_sources, _source_0) .expect ("[c1ef5a99]");
		
		let _content_type = "Html";
		
		let _route = normalize_route (_route.as_ref (), true, false);
		
		let _id = self.generate_id ();
		
		let _description = format! ("askama ({}, source = `{}`)", _content_type, _source_0);
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies_include (&_source);
		
		let _template = _relative.strip_prefix ("/") .expect ("[7285dc26]");
		
		writeln! (self.generated, "::hyper_static_server::askama! (Resource_{}, Template_{}, {}, {:?}, {:?});", _id, _id, _content_type, _template, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_css (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _css_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_css_sources, _source_0) .expect ("[c6442f7b]");
		
		let _route_base = self.configuration.css_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_raw (&_relative, &_source, "Css", _route_base, _route_builder, "resource_css", _source_0, false);
	}
	
	
	#[ cfg (any (feature = "sass-rs", feature = "sass-alt")) ]
	pub fn route_sass (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _css_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_css_sources, _source_0) .expect ("[4f6f6f41]");
		
		self.dependencies_include (&_source);
		
		let _compiled = self.compile_sass (&_source) .expect ("[cf9af211]");
		
		let _source = self.configuration.outputs.join (fingerprint_data (_source.to_string_lossy ().as_bytes ())) .with_extension ("css");
		create_file_from_str (&_source, &_compiled) .expect ("[bd7285f4]");
		
		let _route_base = self.configuration.css_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		let _relative = PathBuf::from (_relative) .with_extension ("css");
		
		self.route_asset_raw (_relative.as_ref (), &_source, "Css", _route_base, _route_builder, "resource_sass", _source_0, false);
		
		self.dependencies_exclude (&_source);
	}
	
	
	
	
	pub fn route_js (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _js_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_js_sources, _source_0) .expect ("[3acb623e]");
		
		let _route_base = self.configuration.js_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_raw (&_relative, &_source, "Js", _route_base, _route_builder, "resource_js", _source_0, false);
	}
	
	
	
	
	pub fn route_image (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) .expect ("[febbd06b]");
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, "resource_image", _source_0, false);
	}
	
	pub fn route_images (&mut self, _sources_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0) .expect ("[31f1c7d2]");
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path));
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, "resource_image", _sources_0, true);
		}
	}
	
	
	pub fn route_icon (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) .expect ("[ec14448c]");
		
		let _route_base = self.configuration.icons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, "resource_icon", _source_0, false);
	}
	
	pub fn route_icons (&mut self, _sources_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0) .expect ("[9aa78087]");
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path));
		
		let _route_base = self.configuration.icons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, "resource_icon", _sources_0, true);
		}
	}
	
	
	pub fn route_favicon (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) .expect ("[26c3b248]");
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_image_0 (&_relative, &_source, _route_base, _route_builder, "resource_favicon", _source_0, false);
	}
	
	pub fn route_favicons (&mut self, _sources_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0) .expect ("[a8b294f4]");
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path));
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_image_0 (&_relative, &_source, _route_base, _route_builder, "resource_favicon", _sources_0, true);
		}
	}
	
	
	fn route_image_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _macro : &str, _source_0 : &str, _source_relative : bool) -> () {
		
		let _content_type = detect_content_type_from_extension (&_source);
		match _content_type {
			"Png" | "Jpeg" | "Icon" | "Svg" =>
				(),
			_ =>
				panic! ("[0fd2d804] {}", _source.display ()),
		};
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, _macro, _source_0, _source_relative);
	}
	
	
	
	
	pub fn route_font (&mut self, _source_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) .expect ("[d84bbf42]");
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_font_0 (&_relative, &_source, _route_base, _route_builder, _source_0, false);
	}
	
	pub fn route_fonts (&mut self, _sources_0 : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0) .expect ("[61b17646]");
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path));
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_font_0 (&_relative, &_source, _route_base, _route_builder, _sources_0, true);
		}
	}
	
	
	fn route_font_0 (&mut self, _relative : &Path, _source : &Path, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _source_0 : &str, _source_relative : bool) -> () {
		
		let _content_type = detect_content_type_from_extension (&_source);
		match _content_type {
			"FontTtf" | "FontOtf" | "FontWoff" | "FontWoff2" =>
				(),
			_ =>
				panic! ("[1a4ccbf4] {}", _source.display ()),
		};
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, "resource_font", _source_0, _source_relative);
	}
	
	
	
	
	pub fn route_asset (&mut self, _source_0 : &str, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized)) {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source_0) .expect ("[8a973b98]");
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		self.route_asset_0 (&_relative, &_source, _content_type, _route_base, _route_builder, _source_0, false);
	}
	
	pub fn route_assets (&mut self, _sources_0 : &str, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources_0) .expect ("[cf4c2fb3]");
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path));
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _files.into_iter () {
			
			self.route_asset_0 (&_relative, &_source, _content_type, _route_base, _route_builder, _sources_0, true);
		}
	}
	
	
	fn route_asset_0 (&mut self, _relative : &Path, _source : &Path, _content_type : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _source_0 : &str, _source_relative : bool) {
		
		let _content_type = _content_type.unwrap_or_else (|| detect_content_type_from_extension (&_source));
		
		self.route_asset_raw (_relative, _source, _content_type, _route_base, _route_builder, "resource_asset", _source_0, _source_relative);
	}
	
	
	
	
	pub fn watch_asset (&mut self, _source : &str) {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) .expect ("[81a2a321]");
		
		self.dependencies_include (&_source);
	}
	
	pub fn watch_assets (&mut self, _sources : &str) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .map (PathBuf::as_path);
		let (_files, _folders) = self.resolve_files (_assets_sources, _sources) .expect ("[ae5a3a79]");
		
		self.dependencies_include_all (_folders.iter () .map (PathBuf::as_path));
		
		self.dependencies_include_all (_files.iter () .map (|_pair| _pair.1.as_path ()));
	}
	
	
	
	
	pub fn generate (mut self) -> () {
		
		self.dependencies_extend ();
		
		writeln! (self.generated, "::hyper_static_server::routes! (Routes, [") .unwrap ();
		for _route_name in self.route_names.into_iter () {
			writeln! (self.generated, "\t{},", _route_name) .unwrap ();
		}
		writeln! (self.generated, "]);") .unwrap ();
		
		writeln! (self.generated, "::hyper_static_server::dependencies! (Dependencies, [") .unwrap ();
		for _dependency in self.dependencies.iter () {
			writeln! (self.generated, "\t{:?},", _dependency) .unwrap ();
		}
		writeln! (self.generated, "]);") .unwrap ();
		
		create_file_from_str (&self.configuration.generated, &self.generated) .expect ("[9796aa67]");
		
		if false {
			eprintln! ("--------------------------------------------------------------------------------");
			eprintln! ("{}", self.generated);
			eprintln! ("--------------------------------------------------------------------------------");
		}
		
		for _dependency in self.dependencies {
			println! ("cargo:rerun-if-changed={}", _dependency.display ());
		}
	}
}




impl Builder {
	
	
	fn resolve_file (&self, _root : Option<&Path>, _source : &str) -> Result<(PathBuf, PathBuf), io::Error> {
		
		let (_path, _relative_root) = self.resolve_source (_root, _source, true) ?;
		
		if ! _path.is_file () {
			return Err (io::Error::new (io::ErrorKind::Other, format! ("[039d945b] {}", _path.display ())));
		}
		
		self.resolve_relative_and_path (&_path, &_relative_root)
	}
	
	
	fn resolve_files (&self, _root : Option<&Path>, _sources : &str) -> Result<(Vec<(PathBuf, PathBuf)>, Vec<PathBuf>), io::Error> {
		
		let (_root, _relative_root) = self.resolve_source (_root, _sources, false) ?;
		
		if ! _root.is_dir () {
			return Err (io::Error::new (io::ErrorKind::Other, "[621693a6]"));
		}
		
		let mut _files = Vec::new ();
		let mut _folders = Vec::new ();
		
		let _walker = walkdir::WalkDir::new (&_root)
				.follow_links (true)
				.sort_by (|_left, _right| ffi::OsStr::cmp (_left.file_name (), _right.file_name ()));
		
		for _entry in _walker {
			let _entry = _entry ?;
			let _path = _entry.path ();
			
			if _path.is_file () {
				
				let _relative_and_path = self.resolve_relative_and_path (_path, &_relative_root) ?;
				
				_files.push (_relative_and_path);
			}
			
			if _path.is_dir () {
				
				let _path = normalize_path (_path);
				
				_folders.push (_path);
			}
		}
		
		return Ok ((_files, _folders));
	}
	
	
	fn resolve_source (&self, _root : Option<&Path>, _source : &str, _name_only : bool) -> Result<(PathBuf, PathBuf), io::Error> {
		
		let _root = _root.expect ("[6e3319c9]");
		
		if ! _root.exists () {
			panic! ("[776c6647]");
		}
		
		if ! _source.starts_with ("_/") {
			return Err (io::Error::new (io::ErrorKind::Other, "[41071330]"));
		}
		
		let _path = _root.join (&_source[2..]);
		
		if ! _path.exists () {
			return Err (io::Error::new (io::ErrorKind::Other, format! ("[1086bd9d] {}", _path.display ())));
		}
		
		if _name_only {
			let _relative_root = _path.parent () .expect ("[067a2cad]") .to_path_buf ();
			Ok ((_path, _relative_root))
		} else {
			Ok ((_path.clone (), _path))
		}
	}
	
	
	fn resolve_relative_and_path (&self, _path : &Path, _relative_root : &Path) -> Result<(PathBuf, PathBuf), io::Error> {
		
		let _relative = _path.strip_prefix (_relative_root) .expect ("[546e7cd9]") .to_str () .expect ("[a48f283c]");
		let _relative = ["/", _relative].concat () .into ();
		
		let _path = normalize_path (&_path);
		
		Ok ((_relative, _path))
	}
}




impl Builder {
	
	
	fn dependencies_include (&mut self, _path : &Path) -> () {
		self.dependencies.insert (_path.into ());
	}
	
	fn dependencies_include_all <'a> (&mut self, _paths : impl Iterator<Item = &'a Path>) -> () {
		for _path in _paths {
			self.dependencies_include (_path);
		}
	}
	
	#[ allow (dead_code) ]
	fn dependencies_exclude (&mut self, _path : &Path) -> () {
		self.dependencies.remove (_path.into ());
	}
	
	fn dependencies_extend (&mut self) -> () {
		
		let mut _extra = Vec::new ();
		
		for _dependency in self.dependencies.iter () {
			
			let _metadata = fs::symlink_metadata (_dependency) .expect ("[e5c6e436]");
			
			if _metadata.file_type () .is_symlink () {
				let _target = _dependency.canonicalize () .expect ("[12da2ba1]");
				_extra.push (_target);
			}
		}
		
		for _dependency in _extra.into_iter () {
			self.dependencies.insert (_dependency);
		}
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
	fn compile_sass (&mut self, _source : &Path) -> Result<String, io::Error> {
		
		let _extension = _source.extension () .expect ("[836ff108]") .to_str () .expect ("[4068e13f]");
		let _indented_syntax = match _extension {
			"sass" =>
				true,
			"scss" =>
				false,
			_ =>
				panic! ("[720c0c23]"),
		};
		
		let _options = sass::Options {
				output_style : sass::OutputStyle::Expanded,
				precision : 4,
				indented_syntax : _indented_syntax,
				include_paths : vec! [],
			};
		
		let mut _context = sass::Context::new_file (&_source) .expect ("[5ebe6232]");
		_context.set_options (_options);
		let _data = _context.compile () .expect ("[31841210]");
		
		return Ok (_data);
	}
	
	
	#[ cfg (feature = "sass-alt") ]
	fn compile_sass (&mut self, _source : &Path) -> Result<String, io::Error> {
		
		let _parent = _source.parent () .expect ("[f6ce0d79]");
		
		let _extension = _source.extension () .expect ("[f2cd37bc]") .to_str () .expect ("[db216e38]");
		let _input_syntax = match _extension {
			"sass" =>
				sass::InputSyntax::SASS,
			"scss" =>
				sass::InputSyntax::SCSS,
			_ =>
				panic! ("[90668feb]"),
		};
		
		pub struct Importer { parent : PathBuf, resolved : rc::Rc<cell::RefCell<Vec<Box<Path>>>> }
		impl sass::SassImporter for Importer {
			fn callback (&mut self, _path_0 : &ffi::CStr, _compiler : sass::SassCompiler) -> Result<Option<Vec<sass::SassImportEntry>>, sass::SassImporterError> {
				
				let _path_0 = Path::new (_path_0.to_str () .expect ("[fd5fc0a2]"));
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
					let _path = _resolved.last () .expect ("[5d9cba96]") .as_ref ();
					let _entry = sass::SassImport::AbsolutePath (_path);
					
					return Ok (Some (vec! (_entry.into_sass_import_entry ())));
				}
				
				panic! ("[e9c920d0] {}", _path.display ());
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
				indent : ffi::CString::new ("\t") .expect ("[77771198]"),
				linefeed : ffi::CString::new ("\n") .expect ("[ef2eea09]"),
				precision : 4,
				input_syntax : _input_syntax,
				include_paths : &[],
				function_list : rc::Rc::new (sass::SassFunctionList::new (Vec::new ())),
				importer_list : rc::Rc::new (sass::SassImporterList::new (vec! (_importer))),
				header_list : rc::Rc::new (sass::SassImporterList::new (Vec::new ())),
			};
		
		let _data = _options.compile_file (_source) .expect ("[bbaffa6f]");
		
		for _dependency in _resolved.borrow () .iter () {
			self.dependencies_include (_dependency.as_ref ());
		}
		
		return Ok (_data);
	}
}




fn create_file_from_str (_path : &Path, _data : &str) -> Result<(), io::Error> {
	
	fs::create_dir_all (_path.parent () .expect ("[370af23d]")) ?;
	
	let mut _file = fs::File::create (&_path) ?;
	_file.write_all (_data.as_bytes ()) ?;
	
	return Ok (());
}




pub trait RoutePathBuilder {
	
	fn build (&self, _source_relative : &Path, _source_path : &Path, _route_prefix_hint : Option<&Path>, _route_infix_hint : Option<&Path>) -> PathBuf;
}


impl RoutePathBuilder for () {
	
	fn build (&self, _source_relative : &Path, _source_path : &Path, _route_prefix_hint : Option<&Path>, _route_infix_hint : Option<&Path>) -> PathBuf {
		generate_route (_source_relative, _route_prefix_hint, _route_infix_hint)
	}
}


impl RoutePathBuilder for (bool, &str) {
	
	fn build (&self, _source_relative : &Path, _source_path : &Path, _route_prefix_hint : Option<&Path>, _route_infix_hint : Option<&Path>) -> PathBuf {
		if self.0 {
			generate_route (_source_relative, Some (Path::new (self.1)), None)
		} else {
			normalize_route (Path::new (self.1), true, false)
		}
	}
}




fn generate_route (_source_relative : &Path, _route_prefix : Option<&Path>, _route_infix : Option<&Path>) -> PathBuf {
	
	let _route_prefix = _route_prefix.expect ("[1ba00780]");
	
	if ! _route_prefix.starts_with ("/") || _route_prefix.ends_with ("/") {
		panic! ("[6fc9256c]");
	}
	if ! _source_relative.starts_with ("/") || _source_relative.ends_with ("/") {
		panic! ("[ace09af4]");
	}
	if let Some (_route_infix) = _route_infix {
		if _route_infix.starts_with ("/") || _route_infix.ends_with ("/") {
			panic! ("[d224b592]");
		}
	}
	
	let _source_relative = _source_relative.strip_prefix ("/") .expect ("[bd4b80bd]");
	
	let _route = if let Some (_route_infix) = _route_infix {
		let _route_infix = _route_infix.strip_prefix ("/") .expect ("[1a7e3353]");
		_route_prefix.join (_route_infix) .join (_source_relative)
	} else {
		_route_prefix.join (_source_relative)
	};
	
	normalize_route (&_route, false, false)
}


fn normalize_route (_path_0 : &Path, _keep_trailing_slash : bool, _force_trailing_slash : bool) -> PathBuf {
	
	if ! _path_0.starts_with ("/") {
		panic! ("[1e7f7bc0]");
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
	
	_path
}


fn normalize_path (_path_0 : &Path) -> PathBuf {
	
	let mut _path = PathBuf::new ();
	for _component in _path_0.components () {
		_path.push (_component);
	}
	_path
}




fn detect_content_type_from_extension (_source : &Path) -> &'static str {
	
	let _extension = _source.extension () .expect ("[29957dc8]") .to_str () .expect ("[908aeea6]");
	
	match _extension {
		"text" | "txt" => "Text",
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
			panic! ("[2bd15bab] {}", _source.display ()),
	}
}




#[ allow (dead_code) ]
fn fingerprint_data (_data : impl AsRef<[u8]>) -> String {
	use blake2::Digest as _;
	let mut _hasher = blake2::Blake2b::new ();
	_hasher.update (_data.as_ref ());
	let _hash = _hasher.finalize ();
	format! ("{:x}", _hash)
}


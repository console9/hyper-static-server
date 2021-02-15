



use ::std::env;
use ::std::fmt::{Write as _};
use ::std::fs;
use ::std::io;
use ::std::io::{Write as _};
use ::std::path::{Path, PathBuf};

use ::sass_rs as sass;
use ::walkdir;
use ::blake2;




#[ derive (Clone, Debug) ]
pub struct BuilderConfiguration {
	
	pub sources : Option<PathBuf>,
	
	pub assets_sources : Option<PathBuf>,
	pub assets_route_base : Option<PathBuf>,
	
	pub css_sources : Option<PathBuf>,
	pub css_route_base : Option<PathBuf>,
	
	pub js_sources : Option<PathBuf>,
	pub js_route_base : Option<PathBuf>,
	
	pub images_route_base : Option<PathBuf>,
	pub favicons_route_base : Option<PathBuf>,
	pub fonts_route_base : Option<PathBuf>,
	
	pub askama_sources : Option<PathBuf>,
	
	pub outputs : PathBuf,
	pub generated : PathBuf,
}


impl Default for BuilderConfiguration {
	
	fn default () -> Self {
		
		let _sources = Self::resolve_sources ();
		let _outputs = Self::resolve_outputs ();
		let _generated = _outputs.join ("./hss-builder-generated-default.in");
		
		let _assets_sources = _sources.join ("./assets");
		let _css_sources = _assets_sources.join ("./css");
		let _js_sources = _assets_sources.join ("./js");
		
		let _assets_sources = if _assets_sources.exists () { Some (_assets_sources) } else { None };
		let _assets_route_base = Some (PathBuf::from ("/assets"));
		
		let _css_sources = if _css_sources.exists () { Some (_css_sources) } else { None };
		let _css_route_base = Some (PathBuf::from ("/assets/css"));
		
		let _js_sources = if _js_sources.exists () { Some (_js_sources) } else { None };
		let _js_route_base = Some (PathBuf::from ("/assets/js"));
		
		let _images_route_base = Some (PathBuf::from ("/assets/images"));
		let _favicons_route_base = Some (PathBuf::from ("/assets/favicons"));
		let _fonts_route_base = Some (PathBuf::from ("/assets/fonts"));
		
		let _askama_sources = _sources.join ("./templates");
		let _askama_sources = if _askama_sources.exists () { Some (_askama_sources) } else { None };
		
		Self {
				sources : Some (_sources),
				
				outputs : _outputs,
				generated : _generated,
				
				assets_sources : _assets_sources,
				assets_route_base : _assets_route_base,
				
				css_sources : _css_sources,
				css_route_base : _css_route_base,
				
				js_sources : _js_sources,
				js_route_base : _js_route_base,
				
				images_route_base : _images_route_base,
				favicons_route_base : _favicons_route_base,
				fonts_route_base : _fonts_route_base,
				
				askama_sources : _askama_sources,
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
				outputs : _outputs,
				generated : _generated,
				
				assets_sources : None,
				assets_route_base : None,
				
				css_sources : None,
				css_route_base : None,
				
				js_sources : None,
				js_route_base : None,
				
				images_route_base : None,
				favicons_route_base : None,
				fonts_route_base : None,
				
				askama_sources : None,
			}
		
	}
	
	pub fn resolve_sources () -> PathBuf {
		let _sources = PathBuf::from (env::var ("CARGO_MANIFEST_DIR") .expect ("[4c6c04d8]"));
		if ! _sources.is_dir () {
			panic! ("[148bc689]");
		}
		_sources
	}
	
	pub fn resolve_outputs () -> PathBuf {
		let _outputs = PathBuf::from (env::var ("OUT_DIR") .expect ("[f039039f]"));
		if ! _outputs.is_dir () {
			panic! ("[fcee6b4d]");
		}
		_outputs
	}
}




pub struct Builder {
	configuration : BuilderConfiguration,
	generated : String,
	counter : u32,
	route_names : Vec<String>,
	dependencies : Vec<PathBuf>,
}


impl Builder {
	
	pub fn new (_configuration : BuilderConfiguration) -> Self {
		Self {
				configuration : _configuration,
				generated : String::with_capacity (1024 * 1024),
				counter : 0,
				route_names : Vec::new (),
				dependencies : Vec::new (),
			}
	}
	
	pub fn new_with_defaults () -> Self {
		Self::new (BuilderConfiguration::default ())
	}
}




impl Builder {
	
	
	
	
	pub fn route_askama (&mut self, _source : &str, _route : &str) -> () {
		
		let _askama_sources = self.configuration.askama_sources.as_ref () .expect ("[b4034d6e]");
		let (_relative, _source) = self.resolve_file (_askama_sources, _source) .expect ("[c1ef5a99]");
		
		let _route = normalize_route (_route.as_ref (), true, false);
		
		let _id = self.generate_id ();
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _content_type = "Html";
		let _description = format! ("askama ({}, source = `...{}`)", _content_type, _relative);
		
		writeln! (self.generated, "::hyper_static_server::askama! (Resource_{}, Template_{}, {}, {:?}, {:?});", _id, _id, _content_type, &_relative[1..], _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_css (&mut self, _source : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _css_sources = self.configuration.css_sources.as_ref () .expect ("[7844407c]");
		let (_relative, _source) = self.resolve_file (_css_sources, _source) .expect ("[c6442f7b]");
		
		let _id = self.generate_id ();
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _route_base = self.configuration.css_route_base.as_ref () .map (PathBuf::as_path);
		let _route = _route_builder.build (_relative.as_ref (), &_source, _route_base, None);
		
		let _content_type = "Css";
		let _description = format! ("resource_css ({}, source = `...{}`)", _content_type, _relative);
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, embedded, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	pub fn route_sass (&mut self, _source : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _css_sources = self.configuration.css_sources.as_ref () .expect ("[0d19f056]") .to_owned ();
		let (_relative, _source) = self.resolve_file (&_css_sources, _source) .expect ("[4f6f6f41]");
		
		let _id = self.generate_id ();
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _relative = PathBuf::from (_relative) .with_extension ("css") .to_string_lossy () .into_owned ();
		
		let _route_base = self.configuration.css_route_base.as_ref () .map (PathBuf::as_path);
		let _route = _route_builder.build (_relative.as_ref (), &_source, _route_base, None);
		
		let _compiled = self.compile_sass (&_source, &_css_sources) .expect ("[cf9af211]");
		let _source = self.configuration.outputs.join (fingerprint_data (_source.to_string_lossy ().as_bytes ())) .with_extension ("css");
		create_file_from_str (&_source, &_compiled) .expect ("[bd7285f4]");
		
		let _content_type = "Css";
		let _description = format! ("resource_sass ({}, source = `...{}`)", _content_type, _relative);
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, embedded, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_js (&mut self, _source : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _js_sources = self.configuration.js_sources.as_ref () .expect ("[ce97440c]");
		let (_relative, _source) = self.resolve_file (_js_sources, _source) .expect ("[3acb623e]");
		
		let _id = self.generate_id ();
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _route_base = self.configuration.js_route_base.as_ref () .map (PathBuf::as_path);
		let _route = _route_builder.build (_relative.as_ref (), &_source, _route_base, None);
		
		let _content_type = "Js";
		let _description = format! ("resource_js ({}, source = `...{}`)", _content_type, _relative);
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, embedded, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_image (&mut self, _source : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[24f74a86]");
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) .expect ("[e606a001]");
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		let _id = self.generate_id ();
		
		self.route_image_0 (_id, _relative, _source, _route_base, _route_builder, None);
	}
	
	pub fn route_images (&mut self, _sources : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[24f74a86]");
		let (_paths, _folders) = self.resolve_files (_assets_sources, _sources) .expect ("[e606a001]");
		self.dependencies.extend (_folders);
		
		let _route_base = self.configuration.images_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _paths.into_iter () {
			
			let _id = self.generate_id ();
			
			self.route_image_0 (_id, _relative, _source, _route_base, _route_builder, Some (_sources.as_ref ()));
		}
	}
	
	pub fn route_favicon (&mut self, _source : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[24f74a86]");
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) .expect ("[e606a001]");
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		let _id = self.generate_id ();
		
		self.route_image_0 (_id, _relative, _source, _route_base, _route_builder, None);
	}
	
	pub fn route_favicons (&mut self, _sources : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[24f74a86]");
		let (_paths, _folders) = self.resolve_files (_assets_sources, _sources) .expect ("[e606a001]");
		self.dependencies.extend (_folders);
		
		let _route_base = self.configuration.favicons_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _paths.into_iter () {
			
			let _id = self.generate_id ();
			
			self.route_image_0 (_id, _relative, _source, _route_base, _route_builder, Some (_sources.as_ref ()));
		}
	}
	
	fn route_image_0 (&mut self, _id : u32, _relative : String, _source : PathBuf, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _from : Option<&Path>) -> () {
		
		let _route = _route_builder.build (_relative.as_ref (), &_source, _route_base, None);
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _extension = _source.extension () .expect ("[4b7affbe]") .to_str () .expect ("[a874a519]");
		let _content_type = match _extension {
			"png" => "Png",
			"jpeg" | "jpg" => "Jpeg",
			"ico" => "Icon",
			"svg" => "Svg",
			_ =>
				panic! ("[0fd2d804] {:?}", _source),
		};
		
		let _description = if let Some (_from) = _from {
			format! ("resource_image ({}, from = `...{}`, file = `...{}`)", _content_type, _from.display (), _relative)
		} else {
			format! ("resource_image ({}, file = `...{}`)", _content_type, _relative)
		};
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, embedded, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_font (&mut self, _source : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[24f74a86]");
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) .expect ("[e606a001]");
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		let _id = self.generate_id ();
		
		self.route_font_0 (_id, _relative, _source, _route_base, _route_builder, None);
	}
	
	pub fn route_fonts (&mut self, _sources : &str, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[24f74a86]");
		let (_paths, _folders) = self.resolve_files (_assets_sources, _sources) .expect ("[5ffa5360]");
		self.dependencies.extend (_folders);
		
		let _route_base = self.configuration.fonts_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _paths.into_iter () {
			
			let _id = self.generate_id ();
			
			self.route_font_0 (_id, _relative, _source, _route_base, _route_builder, Some (_sources.as_ref ()));
		}
	}
	
	
	fn route_font_0 (&mut self, _id : u32, _relative : String, _source : PathBuf, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _from : Option<&Path>) -> () {
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _route = _route_builder.build (_relative.as_ref (), &_source, _route_base, None);
		
		let _extension = _source.extension () .expect ("[d4f695e7]") .to_str () .expect ("[53aff957]");
		let _content_type = match _extension {
			"ttf" => "FontTtf",
			"otf" => "FontOtf",
			"woff" => "FontWoff",
			"woff2" => "FontWoff2",
			_ =>
				panic! ("[1a4ccbf4] {:?}", _source),
		};
		
		let _description = if let Some (_from) = _from {
			format! ("resource_font ({}, from = `...{}`, file = `...{}`)", _content_type, _from.display (), _relative)
		} else {
			format! ("resource_font ({}, file = `...{}`)", _content_type, _relative)
		};
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, embedded, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn route_asset (&mut self, _source : &str, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized)) {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[443c8ae5]");
		let (_relative, _source) = self.resolve_file (_assets_sources, _source) .expect ("[256c60bf]");
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		let _id = self.generate_id ();
		
		self.route_asset_0 (_id, _relative, _source, _content_type, _route_base, _route_builder, None);
	}
	
	
	pub fn route_assets (&mut self, _sources : &str, _content_type : Option<&str>, _route_builder : &(impl RoutePathBuilder + ?Sized)) -> () {
		
		let _assets_sources = self.configuration.assets_sources.as_ref () .expect ("[d807eb26]");
		let (_paths, _folders) = self.resolve_files (_assets_sources, _sources) .expect ("[5ffa5360]");
		self.dependencies.extend (_folders);
		
		let _route_base = self.configuration.assets_route_base.clone ();
		let _route_base = _route_base.as_ref () .map (PathBuf::as_path);
		
		for (_relative, _source) in _paths.into_iter () {
			
			let _id = self.generate_id ();
			
			self.route_asset_0 (_id, _relative, _source, _content_type, _route_base, _route_builder, Some (_sources.as_ref ()));
		}
	}
	
	
	fn route_asset_0 (&mut self, _id : u32, _relative : String, _source : PathBuf, _content_type : Option<&str>, _route_base : Option<&Path>, _route_builder : &(impl RoutePathBuilder + ?Sized), _from : Option<&Path>) {
		
		self.route_names.push (format! ("Route_{}", _id));
		self.dependencies.push (_source.clone ());
		
		let _route = _route_builder.build (_relative.as_ref (), &_source, _route_base, None);
		
		let _extension = _source.extension () .expect ("[29957dc8]") .to_str () .expect ("[908aeea6]");
		let _content_type = if let Some (_content_type) = _content_type {
			_content_type
		} else {
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
					panic! ("[2bd15bab] {:?}", _source),
			}
		};
		
		let _description = if let Some (_from) = _from {
			format! ("resource_asset ({}, from = `...{}`, file = `...{}`)", _content_type, _from.display (), _relative)
		} else {
			format! ("resource_asset ({}, file = `...{}`)", _content_type, _relative)
		};
		
		writeln! (self.generated, "::hyper_static_server::resource! (Resource_{}, {}, embedded, (relative_to_cwd, {:?}), {:?});", _id, _content_type, _source, _description) .unwrap ();
		writeln! (self.generated, "::hyper_static_server::route! (Route_{}, Resource_{}, {:?});", _id, _id, _route) .unwrap ();
	}
	
	
	
	
	pub fn generate (mut self) -> () {
		
		writeln! (self.generated, "::hyper_static_server::routes! (Routes, [") .unwrap ();
		for _route_name in self.route_names.into_iter () {
			writeln! (self.generated, "\t{},", _route_name) .unwrap ();
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
	
	
	fn resolve_file (&self, _root : &Path, _source : &str) -> Result<(String, PathBuf), io::Error> {
		
		if ! _root.exists () {
			panic! ("[776c6647]");
		}
		
		if ! _source.starts_with ("/") {
			return Err (io::Error::new (io::ErrorKind::Other, "[41071330]"));
		}
		
		let _path = _root.join (&_source[1..]);
		
		if ! _path.is_file () {
			return Err (io::Error::new (io::ErrorKind::Other, format! ("[039d945b] {:?}", _path)));
		}
		
		let _relative = _path.strip_prefix (_root) .expect ("[546e7cd9]") .to_str () .expect ("[a48f283c]");
		let _relative = ["/", _relative].concat ();
		
		return Ok ((_relative.into (), _path));
	}
	
	
	fn resolve_files (&self, _root : &Path, _sources : &str) -> Result<(Vec<(String, PathBuf)>, Vec<PathBuf>), io::Error> {
		
		if ! _root.exists () {
			panic! ("[e6a5c950]");
		}
		
		if ! _sources.starts_with ("/") {
			return Err (io::Error::new (io::ErrorKind::Other, "[8e912b21]"));
		}
		
		let _root = _root.join (&_sources[1..]);
		
		if ! _root.is_dir () {
			return Err (io::Error::new (io::ErrorKind::Other, "[621693a6]"));
		}
		
		let mut _paths = Vec::new ();
		let mut _folders = Vec::new ();
		
		for _entry in walkdir::WalkDir::new (&_root) {
			let _entry = _entry ?;
			let _path = _entry.path ();
			if _path.is_file () {
				let _relative = _path.strip_prefix (&_root) .expect ("[703112d8]") .to_str () .expect ("[072ffc03]");
				let _relative = ["/", _relative].concat ();
				_paths.push ((_relative.into (), _path.into ()));
			}
			if _path.is_dir () {
				_folders.push (_path.into ());
			}
		}
		
		return Ok ((_paths, _folders));
	}
	
	
	fn generate_id (&mut self) -> u32 {
		let _id = self.counter;
		self.counter += 1;
		_id
	}
}




impl Builder {
	
	
	fn compile_sass (&self, _source : &Path, _search : &Path) -> Result<String, io::Error> {
		
		let _options = sass::Options {
				output_style : sass::OutputStyle::Expanded,
				precision : 4,
				indented_syntax : true,
				include_paths : vec! [],
			};
		
		let mut _context = sass::Context::new_file (&_source) .expect ("[5ebe6232]");
		_context.set_options (_options);
		let _data = _context.compile () .expect ("[31841210]");
		
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




fn fingerprint_data (_data : impl AsRef<[u8]>) -> String {
	use blake2::Digest as _;
	let mut _hasher = blake2::Blake2b::new ();
	_hasher.update (_data.as_ref ());
	let _hash = _hasher.finalize ();
	format! ("{:x}", _hash)
}








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
		macro_rules! assets_image {
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
		macro_rules! assets_favicon {
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
		macro_rules! assets_font {
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


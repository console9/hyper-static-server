

#[ cfg (feature = "support-sass") ]
use ::sass_rs as sass;


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


use crate::builder_errors::*;




#[ cfg (feature = "support-sass") ]
pub fn compile_sass (_source : &Path) -> BuilderResult<String> {
	
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




#[ cfg (feature = "support-sass-alt") ]
pub fn compile_sass (_source : &Path) -> BuilderResult<String> {
	
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


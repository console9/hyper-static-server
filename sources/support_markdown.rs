
use ::pulldown_cmark as cmark;


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




pub fn compile_markdown_body_from_path (_source : &Path, _title_detect : bool) -> BuilderResult<(String, Option<String>, Option<(String, String)>)> {
	
	let _source = fs::read_to_string (_source) ?;
	
	compile_markdown_body_from_data (_source.as_str (), _title_detect)
}


pub fn compile_markdown_body_from_data (_source : &str, _title_detect : bool) -> BuilderResult<(String, Option<String>, Option<(String, String)>)> {
	
	let mut _input : Vec<&str> = _source.lines () .skip_while (|_line| _line.is_empty ()) .collect ();
	while let Some (_line) = _input.last () {
		if _line.is_empty () {
			_input.pop ();
		} else {
			break;
		}
	}
	
	if _input.is_empty () {
		return Err (error_with_code (0x1fc18809));
	}
	
	let (_input, _frontmatter) = {
		let _detected = if let Some (_line) = _input.first () {
			let _line_trimmed = _line.trim ();
			match _line_trimmed {
				"+++" =>
					Some (("toml", "+++")),
				"---" =>
					Some (("yaml", "---")),
				"{{{" =>
					Some (("json", "}}}")),
				_ =>
					None,
			}
		} else {
			None
		};
		if let Some ((_type, _marker)) = _detected {
			let mut _input = _input.into_iter ();
			let mut _frontmatter = Vec::new ();
			let mut _frontmatter_is_empty = true;
			_input.next ();
			while let Some (_line) = _input.next () {
				let _line_trimmed = _line.trim ();
				if _line_trimmed == _marker {
					break;
				} else {
					_frontmatter.push (_line);
					if ! _line_trimmed.is_empty () {
						_frontmatter_is_empty = false;
					}
				}
			}
			let _input : Vec<&str> = _input.collect ();
			let _frontmatter = if ! _frontmatter_is_empty {
				let _type = String::from (_type);
				let _frontmatter = _frontmatter.join ("\n");
				Some ((_type, _frontmatter))
			} else {
				None
			};
			(_input, _frontmatter)
		} else {
			(_input, None)
		}
	};
	
	let _input = _input.join ("\n");
	
	let mut _options = cmark::Options::empty ();
	_options.insert (cmark::Options::ENABLE_TABLES);
	_options.insert (cmark::Options::ENABLE_FOOTNOTES);
	_options.insert (cmark::Options::ENABLE_STRIKETHROUGH);
	_options.insert (cmark::Options::ENABLE_TASKLISTS);
	
	let mut _body = String::with_capacity (_input.len () * 2);
	
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
	
	cmark::html::push_html (&mut _body, _parser);
	
	Ok ((_body, _title, _frontmatter))
}




pub fn compile_markdown_body_to_paths (_source_path : &Path, _context_path : Option<&Path>, _title_path : Option<&Path>, _body_path : Option<&Path>) -> BuilderResult {
	
	let (_body, _title, _frontmatter) = compile_markdown_body_from_path (_source_path, _title_path.is_some ()) ?;
	
	if let Some (_path) = _context_path {
		let _data = if let Some ((_type, _data)) = _frontmatter {
			match _type.as_str () {
				"toml" => "## toml\n".to_owned () + &_data,
				"yaml" => "## yaml\n".to_owned () + &_data,
				"json" => _data,
				_ =>
					return Err (error_with_code (0xfc776131)),
			}
		} else {
			String::new ()
		};
		let mut _file = fs::File::create (_path) ?;
		_file.write_all (_data.as_bytes ()) ?;
	}
	
	if let Some (_path) = _title_path {
		let _data = if let Some (_title) = _title {
			_title
		} else {
			String::new ()
		};
		let mut _file = fs::File::create (_path) ?;
		_file.write_all (_data.as_bytes ()) ?;
	}
	
	if let Some (_path) = _body_path {
		let _data = _body;
		let mut _file = fs::File::create (_path) ?;
		_file.write_all (_data.as_bytes ()) ?;
	}
	
	Ok (())
}




pub fn compile_markdown_html_from_path (_source : &Path, _header : Option<&Path>, _footer : Option<&Path>) -> BuilderResult<String> {
	
	let _source = fs::read_to_string (_source) ?;
	let _header = if let Some (_header) = _header { Some (fs::read_to_string (_header) ?) } else { None };
	let _footer = if let Some (_footer) = _footer { Some (fs::read_to_string (_footer) ?) } else { None };
	
	let _source = _source.as_str ();
	let _header = _header.as_ref () .map (String::as_str);
	let _footer = _footer.as_ref () .map (String::as_str);
	
	compile_markdown_html_from_data (_source, _header, _footer)
}


pub fn compile_markdown_html_from_data (_source : &str, _header : Option<&str>, _footer : Option<&str>) -> BuilderResult<String> {
	
	let (_body, _title, _frontmatter) = compile_markdown_body_from_data (_source, true) ?;
	
	let _html = if _header.is_some () || _footer.is_some () {
		
		let _header = _header.unwrap_or ("");
		let _footer = _footer.unwrap_or ("");
		
		let _title = if let Some (_title) = _title {
			let mut _buffer = String::with_capacity (_title.len () * 3 / 2);
			cmark::escape::escape_html (&mut _buffer, &_title) .infallible (0xef399d64);
			_buffer
		} else {
			String::new ()
		};
		
		let mut _buffer = String::with_capacity (_header.len () + _body.len () + _footer.len ());
		_buffer.push_str (&_header.replace ("@@{{HSS::Markdown::Title}}", &_title));
		_buffer.push_str (&_body);
		_buffer.push_str (&_footer.replace ("@@{{HSS::Markdown::Title}}", &_title));
		
		_buffer
		
	} else {
		
		let mut _buffer = String::with_capacity (_body.len () + 1024);
		
		_buffer.push_str ("<!DOCTYPE html>\n");
		_buffer.push_str ("<html>\n");
		_buffer.push_str ("<head>\n");
		
		if let Some (_title) = _title {
			_buffer.push_str ("<title>");
			cmark::escape::escape_html (&mut _buffer, &_title) .infallible (0xdc5ea905);
			_buffer.push_str ("</title>\n");
		}
		
		_buffer.push_str (r#"<meta name="viewport" content="width=device-width, height=device-height" />"#);
		_buffer.push_str ("\n");
		_buffer.push_str ("</head>\n");
		_buffer.push_str ("<body>\n");
		
		_buffer.push_str (&_body);
		
		_buffer.push_str ("</body>\n");
		_buffer.push_str ("</html>\n");
		
		_buffer
	};
	
	Ok (_html)
}


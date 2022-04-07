
use ::pulldown_cmark as cmark;
use ::any_ascii as any_ascii;


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




pub struct MarkdownOptions {
	
	pub title_detect : bool,
	pub headings_detect : bool,
	pub headings_anchors : bool,
	
	pub enable_tables : bool,
	pub enable_footnotes : bool,
	pub enable_strikethrough : bool,
	pub enable_tasklists : bool,
	pub enable_headings_attributes : bool,
	
}


impl MarkdownOptions {
	
	pub fn new () -> Self {
		Self {
				
				title_detect : true,
				headings_detect : true,
				headings_anchors : true,
				
				enable_tables : true,
				enable_footnotes : true,
				enable_strikethrough : true,
				enable_tasklists : true,
				enable_headings_attributes : false,
				
			}
	}
}




pub struct MarkdownOutput {
	pub body : String,
	pub title : Option<String>,
	pub headers : Option<Vec<MarkdownHeader>>,
	pub frontmatter : Option<MarkdownFrontmatter>,
}


pub struct MarkdownHeader {
	pub level : u8,
	pub text : String,
	pub anchor : String,
}


pub struct MarkdownFrontmatter {
	pub encoding : String,
	pub data : String,
}




pub fn compile_markdown_body_from_path (_source : &Path, _options : MarkdownOptions) -> BuilderResult<MarkdownOutput> {
	
	let _source = fs::read_to_string (_source) ?;
	
	compile_markdown_body_from_data (_source.as_str (), _options)
}


pub fn compile_markdown_body_from_data (_source : &str, _options : MarkdownOptions) -> BuilderResult<MarkdownOutput> {
	
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
		if let Some ((_encoding, _marker)) = _detected {
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
				let _encoding = String::from (_encoding);
				let _frontmatter = _frontmatter.join ("\n");
				Some ((_encoding, _frontmatter))
			} else {
				None
			};
			(_input, _frontmatter)
		} else {
			(_input, None)
		}
	};
	
	let _input = _input.join ("\n");
	
	let mut _parser_options = cmark::Options::empty ();
	if _options.enable_tables {
		_parser_options.insert (cmark::Options::ENABLE_TABLES);
	}
	if _options.enable_footnotes {
		_parser_options.insert (cmark::Options::ENABLE_FOOTNOTES);
	}
	if _options.enable_strikethrough {
		_parser_options.insert (cmark::Options::ENABLE_STRIKETHROUGH);
	}
	if _options.enable_tasklists {
		_parser_options.insert (cmark::Options::ENABLE_TASKLISTS);
	}
	if _options.enable_headings_attributes {
		_parser_options.insert (cmark::Options::ENABLE_HEADING_ATTRIBUTES);
	}
	
	let mut _body = String::with_capacity (_input.len () * 2);
	
	
	let _parser = cmark::Parser::new_ext (&_input, _parser_options);
	
	let mut _events : Vec<_> = _parser.into_iter () .collect ();
	
	let mut _title = None;
	if _options.title_detect {
		let mut _capture_next = false;
		for _event in _events.iter () {
			match _event {
				cmark::Event::Start (cmark::Tag::Heading (cmark::HeadingLevel::H1, _, _)) =>
					_capture_next = true,
				cmark::Event::Text (_text) =>
					if _capture_next {
						if ! _text.is_empty () {
							_title = Some (_text.as_ref () .to_owned ());
						}
						break;
					}
				_ =>
					(),
			}
		}
	}
	
	let mut _headings_anchors = Vec::new ();
	if _options.headings_anchors {
		let mut _generate_next = false;
		for (_index, _event) in _events.iter () .enumerate () {
			match _event {
				cmark::Event::Start (cmark::Tag::Heading (_, _anchor, _)) =>
					if _anchor.is_none () {
						_generate_next = true;
					}
				cmark::Event::Text (_text) =>
					if _generate_next {
						if ! _text.is_empty () {
							let _anchor_id = markdown_anchor_from_text (_text.as_ref ());
							if ! _anchor_id.is_empty () {
								_headings_anchors.push ((_index - 1, _anchor_id));
							}
						}
						_generate_next = false;
					}
				_ =>
					(),
			}
		}
		for (_index, _anchor_id) in _headings_anchors.iter () {
			let _event = _events.get_mut (*_index) .infallible (0xf65facdb);
			match _event {
				cmark::Event::Start (cmark::Tag::Heading (_, ref mut _anchor, _)) =>
					*_anchor = Some (_anchor_id),
				_ =>
					unreachable! ("[eddfdaf1]"),
			}
		}
	}
	
	cmark::html::push_html (&mut _body, _events.into_iter ());
	
	let _frontmatter = if let Some ((_encoding, _data)) = _frontmatter {
		Some (MarkdownFrontmatter {
				encoding : _encoding,
				data : _data,
			})
	} else {
		None
	};
	
	let _headers = None;
	
	let _output = MarkdownOutput {
			body : _body,
			title : _title,
			headers : _headers,
			frontmatter : _frontmatter,
		};
	
	Ok (_output)
}




pub fn compile_markdown_body_to_paths (_source_path : &Path, _context_path : Option<&Path>, _title_path : Option<&Path>, _body_path : Option<&Path>) -> BuilderResult {
	
	let mut _options = MarkdownOptions::new ();
	_options.title_detect = _title_path.is_some ();
	
	let _output = compile_markdown_body_from_path (_source_path, _options) ?;
	
	let _body = _output.body;
	let _title = _output.title;
	let _frontmatter = _output.frontmatter;
	
	if let Some (_path) = _context_path {
		let _data = if let Some (_frontmatter) = _frontmatter {
			match _frontmatter.encoding.as_str () {
				"toml" => "## toml\n".to_owned () + &_frontmatter.data,
				"yaml" => "## yaml\n".to_owned () + &_frontmatter.data,
				"json" => _frontmatter.data,
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
	
	let mut _options = MarkdownOptions::new ();
	_options.title_detect = true;
	
	let _output = compile_markdown_body_from_data (_source, _options) ?;
	
	let _body = _output.body;
	let _title = _output.title;
	let _frontmatter = _output.frontmatter;
	
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




pub fn markdown_anchor_from_text (_text : &str) -> String {
	
	let mut _text = any_ascii::any_ascii (_text);
	_text.make_ascii_lowercase ();
	
	let _max_length = std::cmp::max (_text.len (), 128);
	let mut _id = String::with_capacity (_max_length);
	
	let mut _separator = false;
	for _character in _text.chars () {
		if _id.len () >= _max_length {
			break;
		}
		if _character.is_ascii_alphabetic () || _character.is_ascii_digit () {
			if _separator {
				_id.push ('_');
				_separator = false;
			}
			_id.push (_character);
		} else {
			_separator = true;
		}
	}
	
	return _id;
}


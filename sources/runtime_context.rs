

use ::std::{
		
		fmt::Debug,
		
		sync::Arc,
		
	};


use crate::{
		
		Resource,
		
	};


use crate::errors::*;




pub trait ContextResource
		where
			Self : Resource,
{
	type Context : Context;
	
	fn context_arc (&self) -> ResourceResult<Arc<Self::Context>>;
}




pub trait Context
		where
			Self : Sized + 'static,
{
	fn new_with_defaults () -> ContextResult<Self> {
		fail! (0xe41380ce, "context can't be created with defaults!");
	}
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8]) -> ContextResult<Self> {
		fail! (0x97024f74, "context can't be deserialized!");
	}
	
	#[ doc (hidden) ]
	fn hook_initialize (&mut self) -> ContextResult {
		Ok (())
	}
}




impl Context for () {
	
	fn new_with_defaults () -> ContextResult<Self> {
		Ok (())
	}
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8]) -> ContextResult<Self> {
		Ok (())
	}
}




#[ cfg (feature = "runtime-context-serde") ]
pub trait ContextSerde
		where
			Self : Sized + 'static,
			Self : ::serde::de::DeserializeOwned,
{
	fn new_with_serde <'a> (_encoding : &str, _data : &[u8]) -> ContextResult<Self> {
		match _encoding {
			
			#[ cfg (feature = "toml") ]
			"toml" =>
				::toml::from_slice (_data) .else_wrap (0xd02e891d),
			
			#[ cfg (feature = "serde_yaml") ]
			"yaml" =>
				::serde_yaml::from_slice (_data) .else_wrap (0xf6e7147f),
			
			#[ cfg (feature = "serde_json") ]
			"json" =>
				::serde_json::from_slice (_data) .else_wrap (0xa8d9dccf),
			
			"auto" => {
				let _encoding = if _data.starts_with (b"## toml\n") {
					"toml"
				} else if _data.starts_with (b"## yaml\n") {
					"yaml"
				} else if _data.starts_with (b"{") || _data.starts_with (b"[") {
					"json"
				} else {
					fail! (0x164f2b63, "encoding `{}` failed to match", _encoding);
				};
				Self::new_with_serde (_encoding, _data)
			}
			
			_ =>
				fail! (0x95017b94, "encoding `{}` not supported", _encoding),
		}
	}
	
	#[ doc (hidden) ]
	fn hook_initialize (&mut self) -> ContextResult {
		Ok (())
	}
}


#[ cfg (feature = "runtime-context-serde") ]
impl <S : ContextSerde> Context for S {
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8]) -> ContextResult<Self> {
		<Self as ContextSerde>::new_with_serde (_encoding, _data)
	}
	
	#[ doc (hidden) ]
	fn hook_initialize (&mut self) -> ContextResult {
		<Self as ContextSerde>::hook_initialize (self)
	}
}




pub trait ValueDebug : Debug {
	
	
	fn debug (&self) -> String {
		format! ("{:?}", self)
	}
	
	
	fn debug_pretty (&self) -> String {
		format! ("{:#?}", self)
	}
	
	
	fn debug_indent_askama (&self, _level : &usize, _indent : &str, _indent_prefix : &str, _indent_suffix : &str, _terminator : &str) -> String {
		let _indent = if _indent.is_empty () { None } else { Some (_indent) };
		let _indent_prefix = if _indent_prefix.is_empty () { None } else { Some (_indent_prefix) };
		let _indent_suffix = if _indent_suffix.is_empty () { None } else { Some (_indent_suffix) };
		let _terminator = Some (_terminator);
		self.debug_indent (*_level, _indent, _indent_prefix, _indent_suffix, _terminator)
	}
	
	fn debug_indent (&self, _level : usize, _indent : Option<&str>, _indent_prefix : Option<&str>, _indent_suffix : Option<&str>, _terminator : Option<&str>) -> String {
		
		let _pretty = self.debug_pretty ();
		
		let _indent_level = _level;
		let _indent_infix = _indent.unwrap_or ("    ");
		let _indent_prefix = _indent_prefix.unwrap_or ("");
		let _indent_suffix = _indent_suffix.unwrap_or ("");
		let _terminator = _terminator.unwrap_or ("\n");
		
		let mut _buffer = String::new ();
		for _line in _pretty.split ("\n") {
			_buffer.push_str (_indent_prefix);
			for _ in 0 .. _indent_level {
				_buffer.push_str (_indent_infix);
			}
			_buffer.push_str (_indent_suffix);
			_buffer.push_str (_line);
			_buffer.push_str (_terminator);
		}
		
		_buffer
	}
}


impl <V> ValueDebug for V where V : Debug {}


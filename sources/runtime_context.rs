

use ::std::{
		
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




use crate::hss;


use crate::{
		
		hss::ServerResult,
		hss::Extensions,
		
		hss::ResultExtWrap,
		hss::fail_with_code,
		hss::fail_with_message,
		hss::fail_with_format,
		
	};




pub trait StaticAskamaContext
		where Self : Sized
{
	fn new_with_defaults () -> ServerResult<Self> {
		fail_with_message! (0xe41380ce, "context can't be created with defaults!");
	}
	
	fn new_with_extensions (_extensions : &hss::Extensions) -> ServerResult<Self> {
		Self::new_with_defaults ()
	}
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8], _extensions : &hss::Extensions) -> ServerResult<Self> {
		fail_with_message! (0x97024f74, "context can't be deserialized!");
	}
}


impl StaticAskamaContext for () {
	
	fn new_with_defaults () -> ServerResult<()> {
		Ok (())
	}
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8], _extensions : &hss::Extensions) -> ServerResult<Self> {
		Ok (())
	}
}




#[ cfg (feature = "runtime-askama-serde") ]
pub trait StaticAskamaContextSerde
		where
				Self : Sized,
				Self : ::serde::de::DeserializeOwned,
{
	fn new_with_serde <'a> (_encoding : &str, _data : &[u8]) -> ServerResult<Self> {
		match _encoding {
			
			#[ cfg (feature = "toml") ]
			"toml" =>
				::toml::from_slice (_data) .or_wrap (0xd02e891d),
			
			#[ cfg (feature = "serde_yaml") ]
			"yaml" =>
				::serde_yaml::from_slice (_data) .or_wrap (0xf6e7147f),
			
			#[ cfg (feature = "serde_json") ]
			"json" =>
				::serde_json::from_slice (_data) .or_wrap (0xa8d9dccf),
			
			_ =>
				fail_with_format! (0x95017b94, "encoding `{}` not supported", _encoding),
		}
	}
}


#[ cfg (feature = "runtime-askama-serde") ]
impl <S : StaticAskamaContextSerde> StaticAskamaContext for S {
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8], _extensions : &hss::Extensions) -> ServerResult<Self> {
		Self::new_with_serde (_encoding, _data)
	}
}


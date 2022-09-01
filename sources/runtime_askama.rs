

use crate::hss;


use ::std::sync::Arc;


use crate::errors::*;








pub trait AskamaTemplate
		where Self : Sized + 'static
{
	
	type Context : AskamaContext;
	
	fn context (&self) -> &Self::Context;
}


pub trait AskamaTrait
		where Self : AskamaTemplate
{}


pub trait AskamaTraitDefault : AskamaTrait {}








pub trait AskamaContext
		where Self : Sized + 'static
{
	fn new_with_defaults () -> ResourceResult<Self> {
		fail! (0xe41380ce, "context can't be created with defaults!");
	}
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8]) -> ResourceResult<Self> {
		fail! (0x97024f74, "context can't be deserialized!");
	}
	
	#[ doc (hidden) ]
	fn hook_initialize (&mut self) -> ResourceResult {
		Ok (())
	}
}




impl AskamaContext for () {
	
	fn new_with_defaults () -> ResourceResult<Self> {
		Ok (())
	}
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8]) -> ResourceResult<Self> {
		Ok (())
	}
}




#[ cfg (feature = "runtime-askama-serde") ]
pub trait AskamaContextSerde
		where
				Self : Sized + 'static,
				Self : ::serde::de::DeserializeOwned,
{
	fn new_with_serde <'a> (_encoding : &str, _data : &[u8]) -> ResourceResult<Self> {
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
	fn hook_initialize (&mut self) -> ResourceResult {
		Ok (())
	}
}


#[ cfg (feature = "runtime-askama-serde") ]
impl <S : AskamaContextSerde> AskamaContext for S {
	
	fn new_with_deserialization (_encoding : &str, _data : &[u8]) -> ResourceResult<Self> {
		<Self as AskamaContextSerde>::new_with_serde (_encoding, _data)
	}
	
	#[ doc (hidden) ]
	fn hook_initialize (&mut self) -> ResourceResult {
		<Self as AskamaContextSerde>::hook_initialize (self)
	}
}








#[ derive (Debug, Clone) ]
pub struct AskamaDocument {
	pub title : String,
	pub body : String,
}


pub trait AskamaDocumentTemplate : AskamaTemplate {
	
	fn document (&self) -> &AskamaDocument;
	fn metadata (&self) -> &AskamaDocumentMetadata;
}


pub trait AskamaDocumentTrait
		where
			Self : AskamaDocumentTemplate,
			Self : AskamaTrait,
{}


pub trait AskamaDocumentTraitDefault : AskamaDocumentTrait {}








#[ cfg (feature = "runtime-askama-serde") ]
#[ derive (Debug, Clone) ]
#[ derive (serde::Serialize, serde::Deserialize) ]
pub struct AskamaDocumentMetadata {
	pub title : Option<String>,
	pub headings : Option<Vec<AskamaDocumentHeading>>,
}


#[ cfg (feature = "runtime-askama-serde") ]
#[ derive (serde::Serialize, serde::Deserialize) ]
#[ derive (Debug, Clone) ]
pub struct AskamaDocumentHeading {
	pub level : u8,
	pub text : Option<String>,
	pub anchor : Option<String>,
}


#[ cfg (feature = "runtime-askama-serde") ]
impl AskamaDocumentMetadata {
	
	pub fn load_from_json (_json : &str) -> ResourceResult<Self> {
		::serde_json::from_str (_json) .else_wrap (0x6410e85f)
	}
}




#[ cfg (not (feature = "runtime-askama-serde")) ]
#[ derive (Debug, Clone) ]
pub struct AskamaDocumentMetadata ();


#[ cfg (not (feature = "runtime-askama-serde")) ]
impl AskamaDocumentMetadata {
	
	pub fn load_from_json (_json : &str) -> ResourceResult<Self> {
		Ok (Self ())
	}
}








pub struct Singleton <T : Send + Sync + 'static> {
	cell : ::once_cell::sync::OnceCell<SingletonResult<T>>,
	builder : fn () -> SingletonResult<T>,
}


impl <T : Send + Sync + 'static> Singleton<T> {
	
	pub const fn new (_builder : fn () -> SingletonResult<T>) -> Self {
		Self {
				cell : ::once_cell::sync::OnceCell::new (),
				builder : _builder,
			}
	}
	
	pub fn get (&self) -> SingletonResult<&T> {
		match self.cell.get_or_init (|| (self.builder) ()) {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.clone ()),
		}
	}
}




pub type SingletonArc <T> = Singleton<Arc<T>>;


impl <T : Send + Sync + 'static> Singleton<Arc<T>> {
	
	pub fn get_arc (&self) -> SingletonResult<Arc<T>> {
		match self.get () {
			Ok (_arc) =>
				Ok (Arc::clone (_arc)),
			Err (_error) =>
				Err (_error.clone ()),
		}
	}
}


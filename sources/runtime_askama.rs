

use crate::hss;


use ::std::{
		
		sync::Arc,
		
	};


use crate::{
		
		Resource,
		Context,
		
	};


use crate::errors::*;








pub trait AskamaResource
		where
			Self : Resource,
{
	type Template : AskamaTemplate;
	
	fn template_arc (&self) -> ResourceResult<Arc<Self::Template>>;
	
	fn render (&self) -> ResourceResult<String>;
}




pub trait AskamaTemplate
		where
			Self : Sized + 'static,
{
	type Context : Context;
	
	fn context (&self) -> &Self::Context;
}




pub trait AskamaTrait
		where
			Self : AskamaTemplate,
{}


pub trait AskamaTraitDefault : AskamaTrait {}








pub trait AskamaDocumentResource
		where
			Self : AskamaResource,
			<Self as AskamaResource>::Template : AskamaDocumentTemplate,
{}




pub trait AskamaDocumentTemplate
		where
			Self : AskamaTemplate,
{
	fn document (&self) -> &AskamaDocument;
	fn metadata (&self) -> &AskamaDocumentMetadata;
}




pub trait AskamaDocumentTrait
		where
			Self : AskamaDocumentTemplate,
			Self : AskamaTrait,
{}


pub trait AskamaDocumentTraitDefault : AskamaDocumentTrait {}








#[ derive (Debug, Clone) ]
pub struct AskamaDocument {
	pub title : String,
	pub body : String,
}




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


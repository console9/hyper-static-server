
#![ no_implicit_prelude ]




// ################################################################################
// ################################################################################




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama_template {
	
	
	(
			$_resource_name : ident,
			$_template_name : ident,
			$_context_descriptor : tt,
			$_trait_descriptor : tt,
			$_template_path : literal
	) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub struct $_template_name {
			pub context : ::std::sync::Arc<$crate::context_type! ($_context_descriptor)>,
			pub __is_production : bool,
			pub __is_development : bool,
		}
		
		impl $crate::AskamaTemplate for $_template_name {
			
			type Context = $crate::context_type! ($_context_descriptor);
			
			fn context (&self) -> &Self::Context {
				use ::std::convert::AsRef as _;
				::std::sync::Arc::as_ref (&self.context)
			}
		}
		
		impl $crate::AskamaTrait for $_template_name {}
		
		$crate::askama_trait_impl! ($_template_name, $_trait_descriptor);
		
		$crate::cfg_builder_askama_dynamic_disabled! {
			#[ allow (non_camel_case_types) ]
			pub struct $_resource_name {
				template : ::std::sync::Arc<$_template_name>,
			}
		}
		
		$crate::cfg_builder_askama_dynamic_enabled! {
			#[ allow (non_camel_case_types) ]
			pub struct $_resource_name {}
		}
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				use $crate::errors::ResultExtWrap as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _template = Self::TEMPLATE_SINGLETON.get_arc () .else_wrap (0x1fa565bb) ?;
					let _self = Self {
							template : _template,
						};
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _self = Self {};
				}
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			fn render (&self) -> $crate::errors::ResourceResult<::std::string::String> {
				use ::std::convert::AsRef as _;
				use $crate::errors::ResultExtWrap as _;
				let _template = self.template_arc () ?;
				let _template = ::std::sync::Arc::as_ref (&_template);
				let _outcome : $crate::errors::AskamaResult<_> = ::askama::Template::render (_template) .else_wrap (0xe73feb57);
				_outcome.else_wrap (0x32bdca54)
			}
			
			pub fn template_arc (&self) -> $crate::errors::ResourceResult<::std::sync::Arc<$_template_name>> {
				use ::std::clone::Clone as _;
				use $crate::errors::ResultExtWrap as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _template = ::std::sync::Arc::clone (&self.template);
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _template = Self::template_build () ?;
					let _template = ::std::sync::Arc::new (_template);
				}
				$crate::errors::ResourceResult::Ok (_template)
			}
			
			fn template_build () -> $crate::errors::ResourceResult<$_template_name> {
				use $crate::AskamaContext as _;
				use $crate::errors::ResultExtWrap as _;
				let mut _context = $crate::context_new! ($_context_descriptor) .else_wrap (0x3fd73d1f) ?;
				_context.hook_initialize () ?;
				let _context = ::std::sync::Arc::new (_context);
				let _template = $_template_name {
						context : _context,
						__is_production : $crate::cfg_if_production! ({ true } | { false }),
						__is_development : $crate::cfg_if_production! ({ false } | { true }),
					};
				$crate::errors::ResourceResult::Ok (_template)
			}
			
			$crate::cfg_builder_askama_dynamic_disabled! {
				const TEMPLATE_SINGLETON : $crate::SingletonArc<$_template_name> = $crate::SingletonArc::new (|| {
						use $crate::errors::ResultExtWrap as _;
						let _template = Self::template_build () .else_wrap (0xe2bf20f2) ?;
						let _template = ::std::sync::Arc::new (_template);
						$crate::errors::SingletonResult::Ok (_template)
					});
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama {
	
	
	(
			$_resource_name : ident,
			$_template_name : ident,
			$_context_descriptor : tt,
			$_trait_descriptor : tt,
			$_content_type : tt,
			$_template_path : literal,
			$_description : literal
	) => {
		
		
		$crate::askama_template! ($_resource_name, $_template_name, $_context_descriptor, $_trait_descriptor, $_template_path);
		
		
		impl $_resource_name {
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::errors::HandlerResult {
				use $crate::errors::ResultExtWrap as _;
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () .else_wrap (0x0707a06d) ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::errors::HandlerResult::Ok (())
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! askama_document {
	
	
	(
			$_resource_name : ident,
			$_template_name : ident,
			$_context_descriptor : tt,
			$_trait_descriptor : tt,
			$_content_type : tt,
			$_template_path : literal,
			$_body_path : literal,
			$_title_path : literal,
			$_metadata_path : literal,
			$( $_refresher_name : ident, )?
			$_description : literal
	) => {
		
		#[ derive (::askama::Template) ]
		#[ template (path = $_template_path) ]
		#[ allow (non_camel_case_types) ]
		pub struct $_template_name {
			pub context : ::std::sync::Arc<$crate::context_type! ($_context_descriptor)>,
			pub document : ::std::sync::Arc<$crate::AskamaDocument>,
			pub metadata : ::std::sync::Arc<$crate::AskamaDocumentMetadata>,
			pub __is_production : bool,
			pub __is_development : bool,
		}
		
		impl $crate::AskamaTemplate for $_template_name {
			
			type Context = $crate::context_type! ($_context_descriptor);
			
			fn context (&self) -> &Self::Context {
				use ::std::convert::AsRef as _;
				::std::sync::Arc::as_ref (&self.context)
			}
		}
		
		impl $crate::AskamaDocumentTemplate for $_template_name {
			
			fn document (&self) -> &$crate::AskamaDocument {
				use ::std::convert::AsRef as _;
				::std::sync::Arc::as_ref (&self.document)
			}
			
			fn metadata (&self) -> &$crate::AskamaDocumentMetadata {
				use ::std::convert::AsRef as _;
				::std::sync::Arc::as_ref (&self.metadata)
			}
		}
		
		impl $crate::AskamaTrait for $_template_name {}
		
		impl $crate::AskamaDocumentTrait for $_template_name {}
		
		$crate::askama_trait_impl! ($_template_name, $_trait_descriptor);
		
		$crate::cfg_builder_askama_dynamic_disabled! {
			#[ allow (non_camel_case_types) ]
			pub struct $_resource_name {
				template : ::std::sync::Arc<$_template_name>,
			}
		}
		
		$crate::cfg_builder_askama_dynamic_enabled! {
			#[ allow (non_camel_case_types) ]
			pub struct $_resource_name {}
		}
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				use $crate::errors::ResultExtWrap as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _template = Self::TEMPLATE_SINGLETON.get_arc () .else_wrap (0xfb2b3835) ?;
					let _self = Self {
							template : _template,
						};
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _self = Self {};
				}
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			fn render (&self) -> $crate::errors::ResourceResult<::std::string::String> {
				use ::std::convert::AsRef as _;
				use $crate::errors::ResultExtWrap as _;
				let _template = self.template_arc () ?;
				let _template = ::std::sync::Arc::as_ref (&_template);
				let _outcome : $crate::errors::AskamaResult<_> = ::askama::Template::render (_template) .else_wrap (0x216e0521);
				_outcome.else_wrap (0xfbf47116)
			}
			
			pub fn template_arc (&self) -> $crate::errors::ResourceResult<::std::sync::Arc<$_template_name>> {
				use ::std::clone::Clone as _;
				use $crate::errors::ResultExtWrap as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _template = ::std::sync::Arc::clone (&self.template);
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _template = Self::template_build () ?;
					let _template = ::std::sync::Arc::new (_template);
				}
				$crate::errors::ResourceResult::Ok (_template)
			}
			
			fn template_build () -> $crate::errors::ResourceResult<$_template_name> {
				use $crate::AskamaContext as _;
				use $crate::errors::ResultExtWrap as _;
				use ::std::convert::From as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					$(
						::std::compile_error! ("`refresher` not supported without dynamic feature!");
						type _refresher_type = $_refresher_name;
					)?
					let _body = ::std::string::String::from (::std::include_str! ($_body_path));
					let _title = ::std::string::String::from (::std::include_str! ($_title_path));
					let _metadata = ::std::include_str! ($_metadata_path);
					let _metadata = $crate::AskamaDocumentMetadata::load_from_json (_metadata) .else_wrap (0x9fa06da0) ?;
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					$( $_refresher_name::refresh () ?; )?
					let _body = ::std::fs::read_to_string ($_body_path) .else_wrap (0x222c7659) ?;
					let _title = ::std::fs::read_to_string ($_title_path) .else_wrap (0x32c4e114) ?;
					let _metadata = ::std::fs::read_to_string ($_metadata_path) .else_wrap (0xc07d6b78) ?;
					let _metadata = $crate::AskamaDocumentMetadata::load_from_json (&_metadata) .else_wrap (0x93f8e36e) ?;
				}
				let mut _context = $crate::context_new! ($_context_descriptor) .else_wrap (0x01992727) ?;
				_context.hook_initialize () ?;
				let _context = ::std::sync::Arc::new (_context);
				let _document = $crate::AskamaDocument { title : _title, body : _body };
				let _document = ::std::sync::Arc::new (_document);
				let _metadata = ::std::sync::Arc::new (_metadata);
				let _template = $_template_name {
						context : _context,
						document : _document,
						metadata : _metadata,
						__is_production : $crate::cfg_if_production! ({ true } | { false }),
						__is_development : $crate::cfg_if_production! ({ false } | { true }),
					};
				$crate::errors::ResourceResult::Ok (_template)
			}
			
			$crate::cfg_builder_askama_dynamic_disabled! {
				const TEMPLATE_SINGLETON : $crate::SingletonArc<$_template_name> = $crate::SingletonArc::new (|| {
						use $crate::errors::ResultExtWrap as _;
						let _template = Self::template_build () .else_wrap (0xce62b3d7) ?;
						let _template = ::std::sync::Arc::new (_template);
						$crate::errors::SingletonResult::Ok (_template)
					});
			}
		}
		
		impl $_resource_name {
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::errors::HandlerResult {
				use $crate::errors::ResultExtWrap as _;
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () .else_wrap (0x53a2c22f) ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::errors::HandlerResult::Ok (())
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
#[ doc (hidden) ]
macro_rules! askama_trait_impl {
	
	( $_template_name : ident, ! ) => {
		$crate::askama_trait_impl! ($_template_name, { trait : ! });
	};
	
	( $_template_name : ident, { trait : ! } ) => {
		$crate::askama_trait_impl! ($_template_name, { trait : $crate::AskamaTraitDefault });
	};
	
	( $_template_name : ident, $_trait_type : ty ) => {
		$crate::askama_trait_impl! ($_template_name, { trait : $_trait_type });
	};
	
	( $_template_name : ident, { trait : $_trait_type : ty } ) => {
		
		impl $_trait_type for $_template_name {}
	};
}




// ################################################################################
// ################################################################################




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
macro_rules! context {
	
	( $_resource_name : ident, $_context_descriptor : tt ) => {
		
		$crate::cfg_builder_askama_dynamic_disabled! {
			pub struct $_resource_name {
				context : ::std::sync::Arc<$crate::context_type! ($_context_descriptor)>,
			}
		}
		
		$crate::cfg_builder_askama_dynamic_enabled! {
			pub struct $_resource_name {}
		}
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				use $crate::errors::ResultExtWrap as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _context = Self::CONTEXT_SINGLETON.get_arc () .else_wrap (0xa1d3b84e) ?;
					let _self = Self {
							context : _context,
						};
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _self = Self {};
				}
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			pub fn context_arc (&self) -> $crate::errors::ResourceResult<::std::sync::Arc<$crate::context_type! ($_context_descriptor)>> {
				use ::std::clone::Clone as _;
				use $crate::errors::ResultExtWrap as _;
				$crate::cfg_builder_askama_dynamic_disabled! {
					let _context = ::std::sync::Arc::clone (&self.context);
				}
				$crate::cfg_builder_askama_dynamic_enabled! {
					let _context = Self::context_build () ?;
					let _context = ::std::sync::Arc::new (_context);
				}
				$crate::errors::ResourceResult::Ok (_context)
			}
			
			fn context_build () -> $crate::errors::ResourceResult<$crate::context_type! ($_context_descriptor)> {
				use $crate::AskamaContext as _;
				use $crate::errors::ResultExtWrap as _;
				let mut _context = $crate::context_new! ($_context_descriptor) .else_wrap (0xfc4e0a63) ?;
				_context.hook_initialize () ?;
				$crate::errors::ResourceResult::Ok (_context)
			}
			
			$crate::cfg_builder_askama_dynamic_disabled! {
				const CONTEXT_SINGLETON : $crate::SingletonArc<$crate::context_type! ($_context_descriptor)> = $crate::SingletonArc::new (|| {
						use $crate::errors::ResultExtWrap as _;
						let _context = Self::context_build () .else_wrap (0x8565ef0c) ?;
						let _context = ::std::sync::Arc::new (_context);
						$crate::errors::SingletonResult::Ok (_context)
					});
			}
		}
	};
}




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
#[ doc (hidden) ]
macro_rules! context_type {
	
	( ! ) => {
		()
	};
	
	( () ) => {
		()
	};
	
	( $_context_type : ty ) => {
		$_context_type
	};
	
	( { type : $_context_type : tt $( , $( $_ : tt )+ )* } ) => {
		$crate::context_type! ( $_context_type )
	};
}




#[ cfg (feature = "runtime-askama") ]
#[ macro_export ]
#[ doc (hidden) ]
macro_rules! context_new {
	
	( ! ) => {
		$crate::context_new! ({ type : () })
	};
	
	( () ) => {
		$crate::context_new! ({ type : () })
	};
	
	( { type : ! $( , $( $_rest : tt )+ )* } ) => {
		$crate::context_new! ( { type : () $( , $( $_rest )+ )* } )
	};
	
	( $_context_type : ty ) => {
		$crate::context_new! ({ type : $_context_type })
	};
	
	( { type : $_context_type : ty } ) => {
		{
			<$_context_type as $crate::AskamaContext>::new_with_defaults ()
		}
	};
	
	( { type : $_context_type : ty, json : $_context_path : literal } ) => {
		$crate::context_new! ({ type : $_context_type, deserialize : ("json", $_context_path) })
	};
	( { type : $_context_type : ty, toml : $_context_path : literal } ) => {
		$crate::context_new! ({ type : $_context_type, deserialize : ("toml", $_context_path) })
	};
	( { type : $_context_type : ty, yaml : $_context_path : literal } ) => {
		$crate::context_new! ({ type : $_context_type, deserialize : ("yaml", $_context_path) })
	};
	
	( { type : $_context_type : ty, deserialize : ( $_context_encoding : literal, $_context_path : literal ) } ) => {
		{
			$crate::cfg_builder_askama_dynamic_disabled! {
				let _context = $crate::context_new! ({ type : $_context_type, (deserialize, embedded) : ($_context_encoding, $_context_path)});
			}
			$crate::cfg_builder_askama_dynamic_enabled! {
				let _context = $crate::context_new! ({ type : $_context_type, (deserialize, dynamic) : ($_context_encoding, $_context_path)});
			}
			_context
		}
	};
	
	( { type : $_context_type : ty, (deserialize, embedded) : ( $_context_encoding : literal, $_context_path : literal ) } ) => {
		{
			let _encoding : &str = $_context_encoding;
			let _data : &[u8] = ::std::include_bytes! ($_context_path);
			<$_context_type as $crate::AskamaContext>::new_with_deserialization (_encoding, _data)
		}
	};
	
	( { type : $_context_type : ty, (deserialize, dynamic) : ( $_context_encoding : literal, $_context_path : literal ) } ) => {
		{
			use $crate::errors::ResultExtWrap as _;
			let _encoding : &str = $_context_encoding;
			let _data = ::std::fs::read ($_context_path) .else_wrap (0x98ea260c) ?;
			<$_context_type as $crate::AskamaContext>::new_with_deserialization (_encoding, &_data)
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
macro_rules! resource {
	
	
	( $_resource_name : ident, $_content_type : tt, auto, $_resource_path : tt, $_description : literal ) => {
		$crate::cfg_if_production! {{
			$crate::resource! ($_resource_name, $_content_type, embedded, $_resource_path, $_description);
		} | {
			$crate::resource! ($_resource_name, $_content_type, dynamic, $_resource_path, $_description);
		}}
	};
	
	
	( $_resource_name : ident, $_content_type : tt, embedded, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_resource_name {};
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				let _self = Self {};
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			const RESOURCE : $crate::hss::EmbeddedResource =
					$crate::hss::EmbeddedResource::new_const (
							::std::include_bytes! ($crate::resource_path! ($_resource_path)),
							::std::option::Option::Some ($crate::resource_content_type! ($_content_type)),
						);
		}
		
		impl $_resource_name {
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				self
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::Handler for $_resource_name {
			
			type Future = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::Future;
			type ResponseBody = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::hss::EmbeddedResource as $crate::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::hss::Request<$crate::hss::Body>) -> Self::Future {
				Self::RESOURCE.handle (_request)
			}
		}
	};
	
	
	( $_resource_name : ident, $_content_type : tt, dynamic, $_resource_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_resource_name {
			resource : $crate::hss::FileResource,
		}
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				let _self = Self {
						resource : $crate::hss::FileResource::new (
								$crate::resource_path! ($_resource_path),
								::std::option::Option::Some ($crate::resource_content_type! ($_content_type)),
								false,
							)
					};
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				self
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::Handler for $_resource_name {
			
			type Future = <$crate::hss::FileResource as $crate::hss::Handler>::Future;
			type ResponseBody = <$crate::hss::FileResource as $crate::hss::Handler>::ResponseBody;
			type ResponseBodyError = <$crate::hss::FileResource as $crate::hss::Handler>::ResponseBodyError;
			
			fn handle (&self, _request : $crate::hss::Request<$crate::hss::Body>) -> Self::Future {
				self.resource.handle (_request)
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ cfg (all (feature = "builder-assets-sass-dynamic", not (feature = "production"))) ]
#[ doc (hidden) ]
macro_rules! resource_sass_dynamic {
	
	( $_resource_name : ident, $_content_type : tt, $_source_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_resource_name {
			source : &'static ::std::path::Path,
		}
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				let _self = Self {
						source : ::std::path::Path::new ($crate::resource_path! ($_source_path)),
					};
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			fn render (&self) -> $crate::errors::ResourceResult<::std::string::String> {
				use $crate::errors::ResultExtWrap as _;
				$crate::support_sass::compile_sass (self.source) .else_wrap (0xbe93dd79)
			}
		}
		
		impl $_resource_name {
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::errors::HandlerResult {
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () .else_wrap (0xdac7b964) ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::errors::HandlerResult::Ok (())
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ cfg (all (feature = "builder-markdown-dynamic", not (feature = "production"))) ]
#[ doc (hidden) ]
macro_rules! resource_markdown_dynamic {
	
	( $_resource_name : ident, $_content_type : tt, $_source_path : tt, $_header_path : tt, $_footer_path : tt, $_description : literal ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_resource_name {
			source : &'static ::std::path::Path,
			header : ::std::option::Option<&'static ::std::path::Path>,
			footer : ::std::option::Option<&'static ::std::path::Path>,
		}
		
		impl $crate::Resource for $_resource_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				let _self = Self {
						source : ::std::path::Path::new ($crate::resource_path! ($_source_path)),
						header : $crate::resource_path! ($_header_path) .map (::std::path::Path::new::<str>),
						footer : $crate::resource_path! ($_footer_path) .map (::std::path::Path::new::<str>),
					};
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_resource_name {
			
			fn render (&self) -> $crate::errors::ResourceResult<::std::string::String> {
				use $crate::errors::ResultExtWrap as _;
				$crate::support_markdown::compile_markdown_html_from_path (self.source, self.header, self.footer, ::std::option::Option::None) .else_wrap (0x0622a827)
			}
		}
		
		impl $_resource_name {
			
			pub fn into_handler (self) -> impl $crate::hss::Handler {
				$crate::hss::HandlerSimpleSyncWrapper::new (self)
			}
		}
		
		impl $crate::StaticResource for $_resource_name {
			
			fn content_type (&self) -> $crate::hss::ContentType {
				$crate::resource_content_type! ($_content_type)
			}
			
			fn description (&self) -> &'static str {
				$_description
			}
			
			fn into_handler_dyn (self) -> $crate::hss::HandlerDynArc {
				let _handler = self.into_handler ();
				let _handler = $crate::hss::HandlerDynArc::new (_handler);
				_handler
			}
		}
		
		impl $crate::hss::HandlerSimpleSync for $_resource_name {
			
			fn handle (&self, _request : &$crate::hss::Request<$crate::hss::Body>, _response : &mut $crate::hss::Response<$crate::hss::Body>) -> $crate::errors::HandlerResult {
				use $crate::hss::ResponseExt as _;
				use $crate::StaticResource as _;
				let _body = self.render () .else_wrap (0xd2a0cfbc) ?;
				_response.set_status_200 ();
				_response.set_content_type (self.content_type ());
				_response.set_body (_body);
				$crate::errors::HandlerResult::Ok (())
			}
		}
	};
}




#[ macro_export ]
#[ cfg (all (feature = "builder-markdown-dynamic", not (feature = "production"))) ]
#[ doc (hidden) ]
macro_rules! resource_markdown_refresher {
	
	( $_refresher_name : ident, $_source_path : tt, $_body_path : tt, $_title_path : tt, $_metadata_path : tt, $_frontmatter_path : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub(crate) struct $_refresher_name {}
		
		impl $_refresher_name {
			
			fn refresh () -> $crate::errors::ResourceResult {
				use $crate::errors::ResultExtWrap as _;
				$crate::support_markdown::compile_markdown_from_path_to_paths (
						::std::path::Path::new ($crate::resource_path! ($_source_path)),
						::std::option::Option::None,
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_body_path))),
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_title_path))),
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_metadata_path))),
						::std::option::Option::Some (::std::path::Path::new ($crate::resource_path! ($_frontmatter_path))),
					)
					.else_wrap (0xc4f1eb11)
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
macro_rules! route {
	
	
	( $_route_name : ident, $_resource_name : ty, $_route_path : literal, $_route_extensions : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_route_name ($crate::hss::Route);
		
		impl $crate::Resource for $_route_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				use ::std::convert::From as _;
				use $crate::StaticResource as _;
				let _resource = <$_resource_name as $crate::Resource>::new_with_defaults () ?;
				// let _ : &dyn $crate::StaticResource = &_resource;
				let _path = ::std::string::String::from ($_route_path);
				let _description = _resource.description ();
				let _handler = $crate::hss::RouteHandler::HandlerDynArc (_resource.into_handler_dyn () .into_arc ());
				let mut _route_extensions = $crate::route_extensions! ($_route_extensions);
				if _route_extensions.get::<$crate::StaticRouteDebug> () .is_none () {
					_route_extensions.insert ($crate::StaticRouteDebug::from_str_static (_description));
				}
				let mut _route = $crate::hss::Route {
						path : _path,
						handler : _handler,
						extensions : ::std::sync::Arc::new (_route_extensions),
					};
				let _self = Self (_route);
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $crate::StaticRoute for $_route_name {
			
			fn into_route (self) -> $crate::hss::Route {
				self.0
			}
		}
		
		impl ::std::convert::Into<$crate::hss::Route> for $_route_name {
			
			fn into (self) -> $crate::hss::Route {
				use $crate::StaticRoute as _;
				self.into_route ()
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ cfg (feature = "runtime-sitemaps") ]
#[ macro_export ]
macro_rules! route_sitemap {
	
	
	( $_route_name : ident, $_route_path : literal, $_prefix : literal, $_format : ident, $_route_extensions : tt ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_route_name ($crate::hss::Route);
		
		impl $crate::Resource for $_route_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				use $crate::errors::ResultExtWrap as _;
				use $crate::StaticResource as _;
				use ::std::convert::From as _;
				let _prefix = ::std::string::String::from ($_prefix);
				let _format = $crate::SitemapFormat::from_str_must (::std::stringify! ($_format));
				let _resource = $crate::RoutesSitemapResource::new (_prefix, _format) .else_wrap (0x255df805) ?;
				// let _ : &dyn $crate::StaticResource = &_resource;
				let _path = ::std::string::String::from ($_route_path);
				let _description = _resource.description ();
				let _handler = $crate::hss::RouteHandler::HandlerDynArc (_resource.into_handler_dyn () .into_arc ());
				let mut _route_extensions = $crate::route_extensions! ($_route_extensions);
				if _route_extensions.get::<$crate::StaticRouteDebug> () .is_none () {
					_route_extensions.insert ($crate::StaticRouteDebug::from_str_static (_description));
				}
				let mut _route = $crate::hss::Route {
						path : _path,
						handler : _handler,
						extensions : ::std::sync::Arc::new (_route_extensions),
					};
				let _self = Self (_route);
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $crate::StaticRoute for $_route_name {
			
			fn into_route (self) -> $crate::hss::Route {
				self.0
			}
		}
		
		impl ::std::convert::Into<$crate::hss::Route> for $_route_name {
			
			fn into (self) -> $crate::hss::Route {
				use $crate::StaticRoute as _;
				self.into_route ()
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ doc (hidden) ]
macro_rules! route_extensions {
	
	( $_route_extensions : tt ) => {
		{
			let mut _route_extensions = $crate::hss::Extensions::new ();
			$crate::route_extensions_insert! (_route_extensions, $_route_extensions);
			_route_extensions
		}
	};
}


#[ macro_export ]
#[ doc (hidden) ]
macro_rules! route_extensions_insert {
	
	( $_route_extensions : ident, () ) => {
	};
	
	( $_route_extensions : ident, {} ) => {
	};
	
	( $_route_extensions : ident, { $_key : ident $( , $( $_rest : tt )+ )* } ) => {
		$crate::route_extensions_insert_one! ($_route_extensions, $_key);
		$crate::route_extensions_insert! ($_route_extensions, { $( $( $_rest )* ),* });
	};
	
	( $_route_extensions : ident, { $_key : ident : $_value : tt $( , $( $_rest : tt )+ )* } ) => {
		$crate::route_extensions_insert_one! ($_route_extensions, $_key, $_value);
		$crate::route_extensions_insert! ($_route_extensions, { $( $( $_rest )* ),* });
	};
}


#[ macro_export ]
#[ doc (hidden) ]
macro_rules! route_extensions_insert_one {
	
	( $_route_extensions : ident, debug, $_debug : expr ) => {
		$_route_extensions.insert ($crate::StaticRouteDebug::new ($_debug));
	};
	
	( $_route_extensions : ident, sitemap ) => {
		$_route_extensions.insert ($crate::RouteSitemapEntry::new ());
	};
	( $_route_extensions : ident, sitemap, { frequency : $_frequency : ident } ) => {
		{
			let mut _entry = $crate::RouteSitemapEntry::new ();
			_entry.frequency = $crate::SitemapFrequency::from_str_must (::std::stringify! ($_frequency));
			$_route_extensions.insert (_entry);
		}
	};
	( $_route_extensions : ident, sitemap, { frequency : $_frequency : ident, priority : $_priority : ident } ) => {
		{
			let mut _entry = $crate::RouteSitemapEntry::new ();
			_entry.frequency = $crate::SitemapFrequency::from_str_must (::std::stringify! ($_frequency));
			_entry.priority = $crate::SitemapPriority::from_str_must (::std::stringify! ($_priority));
			$_route_extensions.insert (_entry);
		}
	};
	( $_route_extensions : ident, sitemap, { frequency : $_frequency : ident, priority : $_priority : literal } ) => {
		{
			let mut _entry = $crate::RouteSitemapEntry::new ();
			_entry.frequency = $crate::SitemapFrequency::from_str_must (::std::stringify! ($_frequency));
			_entry.priority = $crate::SitemapPriority::from_str_must (::std::stringify! ($_priority));
			$_route_extensions.insert (_entry);
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
macro_rules! routes {
	
	
	( $_name : ident, [ $( $_route : ty, )* ] ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_name ($crate::hss::Routes);
		
		impl $crate::Resource for $_name {
			
			fn new_with_defaults () -> $crate::errors::ResourceResult<Self> {
				use $crate::errors::ResultExtWrap as _;
				use ::std::iter::IntoIterator as _;
				let _routes = Self::routes_with_defaults () ?;
				let mut _builder = $crate::hss::RoutesBuilder::new ();
				for _route in _routes.into_iter () {
					_builder = _builder.with_route_object (_route);
				}
				let _routes = _builder.build () .else_wrap (0x4b34a624) ?;
				let _self = Self (_routes);
				$crate::errors::ResourceResult::Ok (_self)
			}
		}
		
		impl $_name {
			
			pub fn routes_with_defaults () -> $crate::errors::ResourceResult<::std::vec::Vec<$crate::hss::Route>> {
				use $crate::StaticRoute as _;
				let _routes = ::std::vec! (
						$(
							{
								let _route : $_route = <$_route as $crate::Resource>::new_with_defaults () ?;
								// let _ : &dyn $crate::StaticRoute = &_route;
								let _route = _route.into_route ();
								_route
							},
						)*
					);
				$crate::errors::ResourceResult::Ok (_routes)
			}
		}
		
		impl $crate::StaticRoutes for $_name {
			
			fn into_routes (self) -> $crate::hss::Routes {
				self.0
			}
		}
		
		impl ::std::convert::Into<$crate::hss::Routes> for $_name {
			
			fn into (self) -> $crate::hss::Routes {
				use $crate::StaticRoutes as _;
				self.into_routes ()
			}
		}
		
		impl $_name {
			
			pub fn eprintln () -> $crate::errors::HandlerResult {
				use $crate::errors::ResultExtWrap as _;
				use ::std::iter::IntoIterator as _;
				let _routes = Self::routes_with_defaults () .else_wrap (0x1a085f16) ?;
				for _route in _routes.into_iter () {
					if let ::std::option::Option::Some (_debug) = _route.extensions.get::<$crate::StaticRouteDebug> () {
						::std::eprintln! ("[dd] [825798f8]  **  {} -> {:?}", _route.path, _debug);
					} else {
						::std::eprintln! ("[dd] [df531ca1]  **  {}", _route.path);
					}
				}
				$crate::errors::HandlerResult::Ok (())
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ doc (hidden) ]
macro_rules! dependencies {
	
	
	( $_name : ident, [ $( $_dependency : literal, )* ] ) => {
		
		#[ allow (non_camel_case_types) ]
		pub struct $_name ();
		
		impl $_name {
			
			pub fn dependencies () -> ::std::vec::Vec<&'static ::std::path::Path> {
				::std::vec! (
					$( ::std::path::Path::new ($_dependency), )*
				)
			}
		}
		
		impl $_name {
			
			pub fn eprintln () -> () {
				use ::std::iter::IntoIterator as _;
				for _dependency in Self::dependencies () .into_iter () {
					::std::eprintln! ("[dd] [402419e4]  !!  {}", _dependency.display ());
				}
			}
		}
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ doc (hidden) ]
macro_rules! resource_content_type {
	
	( text ) => { $crate::hss::ContentType::Text };
	( html ) => { $crate::hss::ContentType::Html };
	( css ) => { $crate::hss::ContentType::Css };
	( js ) => { $crate::hss::ContentType::Js };
	
	( json ) => { $crate::hss::ContentType::Json };
	( xml ) => { $crate::hss::ContentType::Xml };
	
	( png ) => { $crate::hss::ContentType::Png };
	( jpeg ) => { $crate::hss::ContentType::Jpeg };
	( svg ) => { $crate::hss::ContentType::Svg };
	( icon ) => { $crate::hss::ContentType::Icon };
	
	( font_ttf ) => { $crate::hss::ContentType::FontTtf };
	( font_otf ) => { $crate::hss::ContentType::FontOtf };
	( font_woff ) => { $crate::hss::ContentType::FontWoff };
	( font_woff2 ) => { $crate::hss::ContentType::FontWoff2 };
	
	( unknown ) => { $crate::hss::ContentType::Unknown };
	
}




// ################################################################################
// ################################################################################




#[ macro_export ]
macro_rules! builder_generated {
	
	
	() => {
		::std::include! (::std::concat! (::std::env! ("OUT_DIR"), "/hss-builder-generated-default.in"));
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ doc (hidden) ]
macro_rules! resource_path {
	
	
	( ( relative_to_crate, None ) ) => {
		::std::option::Option::None
	};
	( ( relative_to_cwd, None ) ) => {
		::std::option::Option::None
	};
	( None ) => {
		::std::option::Option::None
	};
	
	
	( ( relative_to_crate, Some ( $_path : literal ) ) ) => {
		::std::option::Option::Some ( $crate::resource_path! ( ( relative_to_crate, $_path ) ) )
	};
	( ( relative_to_cwd, Some ( $_path : literal ) ) ) => {
		::std::option::Option::Some ( $crate::resource_path! ( ( relative_to_cwd, $_path ) ) )
	};
	( Some ( $_path : literal ) ) => {
		::std::option::Option::Some ( $crate::resource_path! ( $_path ) )
	};
	
	( ( relative_to_crate, $_path : literal ) ) => {
		::std::concat! (::std::env! ("CARGO_MANIFEST_DIR"), "/", $_path)
	};
	( ( relative_to_cwd, $_path : literal ) ) => {
		$_path
	};
	( $_path : literal ) => {
		$crate::resource_path! ( ( relative_to_crate, $_path ) )
	};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ cfg (feature = "production") ]
#[ doc (hidden) ]
macro_rules! cfg_if_production {
	( { $( $_then_token : tt )* } | { $( $_else_token : tt )* } ) => { $( $_then_token )* };
	( { $( $_then_token : tt )* } ) => { $( $_then_token )* };
}

#[ macro_export ]
#[ cfg (not (feature = "production")) ]
#[ doc (hidden) ]
macro_rules! cfg_if_production {
	( { $( $_then_token : tt )* } | { $( $_else_token : tt )* } ) => { $( $_else_token )* };
	( { $( $_then_token : tt )* } ) => {};
}




// ################################################################################
// ################################################################################




#[ macro_export ]
#[ cfg (any (not (feature = "builder-askama-dynamic"), feature = "production")) ]
#[ doc (hidden) ]
macro_rules! cfg_builder_askama_dynamic_disabled {
	( $( $_token : tt )* ) => { $( $_token )* };
}

#[ macro_export ]
#[ cfg (all (feature = "builder-askama-dynamic", not (feature = "production"))) ]
#[ doc (hidden) ]
macro_rules! cfg_builder_askama_dynamic_disabled {
	( $( $_token : tt )* ) => {};
}


#[ macro_export ]
#[ cfg (all (feature = "builder-askama-dynamic", not (feature = "production"))) ]
#[ doc (hidden) ]
macro_rules! cfg_builder_askama_dynamic_enabled {
	( $( $_token : tt )* ) => {
		$( $_token )*
	};
}

#[ macro_export ]
#[ cfg (any (not (feature = "builder-askama-dynamic"), feature = "production")) ]
#[ doc (hidden) ]
macro_rules! cfg_builder_askama_dynamic_enabled {
	( $( $_token : tt )* ) => {};
}




// ################################################################################
// ################################################################################


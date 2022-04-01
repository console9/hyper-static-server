

#[ allow (unused_imports) ]
pub(crate) use crate::hss::{
		
		error_with_code,
		error_with_format,
		error_with_message,
		
		ResultExtWrap as _,
		ResultExtPanic as _,
		ErrorExtWrap as _,
		
	};




pub type BuilderResult<V = ()> = ::std::result::Result<V, BuilderError>;
pub type BuilderError = ::std::io::Error;



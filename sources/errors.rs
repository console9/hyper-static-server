

pub use ::hyper_simple_server::errors::*;


pub use ::vrl_errors::*;


#[ cfg (feature = "support-builder") ]
::vrl_errors::define_error! (pub BuilderError, result : BuilderResult);

#[ cfg (feature = "exporter") ]
::vrl_errors::define_error! (pub ExportError, result : ExportResult);

#[ cfg (feature = "runtime") ]
::vrl_errors::define_error! (pub ResourceError, result : ResourceResult);

#[ cfg (feature = "runtime") ]
::vrl_errors::define_error! (pub SingletonError, result : SingletonResult);

#[ cfg (feature = "runtime-context") ]
::vrl_errors::define_error! (pub ContextError, result : ContextResult);

#[ cfg (feature = "runtime-askama") ]
::vrl_errors::define_error! (pub AskamaError, result : AskamaResult);

#[ cfg (feature = "runtime-sitemaps") ]
::vrl_errors::define_error! (pub SitemapError, result : SitemapResult);



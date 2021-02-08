

#![ allow (unused_imports) ]
#![ allow (dead_code) ]




pub(crate) use crate::server::*;
pub(crate) use crate::resources::*;
pub(crate) use crate::main::*;




pub(crate) use ::hyper_simple_server as hss;

pub(crate) type Request = hss::Request<hss::Body>;
pub(crate) type Response = hss::Response<hss::Body>;
pub(crate) type ServerResponseFuture = hss::HandlerFutureDynBox;
pub(crate) type ServerResult = hss::ServerResult;
pub(crate) type ServerError = hss::ServerError;

pub(crate) use hss::RequestExt as _;
pub(crate) use hss::ResponseExt as _;
pub(crate) use hss::ResponseExtBuild as _;




pub(crate) use ::std::*;
pub(crate) use ::std::prelude::v1::*;




pub(crate) use ::rand as rand;




pub(crate) fn random_token () -> String {
	use rand::Rng as _;
	let mut _rand = rand::thread_rng ();
	let _token = _rand.gen::<u128> ();
	format! ("{:0x}", _token)
}


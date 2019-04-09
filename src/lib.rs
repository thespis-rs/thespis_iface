#![ allow( unused_imports, dead_code ) ]
#![ feature( await_macro, async_await, futures_api, arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias, box_syntax, box_patterns, associated_type_defaults ) ]


mod actor     ;
mod address   ;
mod envelope  ;
mod executor  ;
mod handler   ;
mod mailbox   ;
mod message   ;
mod recipient ;
mod thespis   ;

pub use
{
	actor     :: * ,
	address   :: * ,
	envelope  :: * ,
	executor  :: * ,
	handler   :: * ,
	mailbox   :: * ,
	message   :: * ,
	recipient :: * ,
	thespis   :: * ,
};


#[ cfg( feature = "derive" ) ] pub use thespis_derive::{ Actor };
#[ cfg( feature = "remote" ) ] mod remote;
#[ cfg( feature = "remote" ) ] pub use remote::*;


use std::{ pin::Pin, future::Future };
//
pub type           TupleResponse<'a   > = Pin<Box< dyn Future< Output = () > + 'a        >>;
pub type ThreadSafeTupleResponse<'a   > = Pin<Box< dyn Future< Output = () > + 'a + Send >>;
pub type                Response<'a, R> = Pin<Box< dyn Future< Output = R  > + 'a        >>;
pub type      ThreadSafeResponse<'a, R> = Pin<Box< dyn Future< Output = R  > + 'a + Send >>;

pub type ThesRes<T> = Result<T, failure::Error>;


mod import
{
	pub use
	{
		std     :: { sync::Arc, pin::Pin, future::Future, convert::TryFrom, hash::Hash, fmt::{ Debug, Display } } ,
		futures :: { prelude::{ Stream, Sink }, channel::{ oneshot, mpsc }, task::Spawn } ,
		failure :: { Error } ,
	};

	#[ cfg( feature = "remote" ) ]
	//
	pub use
	{
		bytes :: { Bytes, BytesMut } ,
	};


}




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
};


#[ cfg( feature = "derive" ) ] pub use thespis_derive::{ Actor };
#[ cfg( feature = "remote" ) ] mod remote;
#[ cfg( feature = "remote" ) ] pub use remote::*;


use std::{ pin::Pin, future::Future };
//
pub type Return      <'a, R> = Pin<Box< dyn Future<Output = R> + 'a >> ;
pub type BoxEnvelope <    A> = Box< dyn Envelope<A> >                  ;
pub type BoxRecipient<    M> = Box< dyn Recipient<M> >                 ;


pub mod thread_safe
{
	use std::{ pin::Pin, future::Future };
	use crate::*;

	pub type Return      <'a, R> = Pin<Box< dyn Future<Output = R> + 'a + Send >> ;
	pub type BoxEnvelope <    A> = Box< dyn Envelope<A>  + Send >                 ;
	pub type BoxRecipient<    M> = Box< dyn Recipient<M> + Send >                 ;
}


pub type ThesRes<T> = Result<T, failure::Error>;


mod import
{
	pub use
	{
		std     ::
		{
			sync    :: Arc                ,
			pin     :: Pin                ,
			future  :: Future             ,
			convert :: TryFrom            ,
			hash    :: Hash               ,
			fmt     :: { Debug, Display } ,
			any     :: Any                ,
		},

		futures :: { prelude::{ FutureExt, Stream, Sink }, channel::{ oneshot, mpsc }, task::Spawn } ,
		failure :: { Error } ,
	};

	#[ cfg( feature = "remote" ) ]
	//
	pub use
	{
		bytes :: { Bytes, BytesMut } ,
		serde :: { Serialize, de::DeserializeOwned } ,
	};


}




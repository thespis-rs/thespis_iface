#![ feature( await_macro, async_await, arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias, box_syntax, box_patterns, associated_type_defaults ) ]


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


// recipient.send now requires futures::sink::SinkExt.
// let's publicly re-export that.
//
pub use futures::sink::{ Sink, SinkExt };


#[ cfg( feature = "derive" ) ] pub use thespis_derive::{ Actor };
#[ cfg( feature = "remote" ) ] mod remote;
#[ cfg( feature = "remote" ) ] pub use remote::*;


use std::{ pin::Pin, future::Future, any::Any };
use failure::Error;
//
pub type Return      <'a, R> = Pin<Box< dyn Future<Output = R> + 'a + Send >> ;
pub type ReturnNoSend<'a, R> = Pin<Box< dyn Future<Output = R> + 'a        >> ;


pub type BoxEnvelope <A> = Box< dyn Envelope<A>  + Send                > ;
pub type BoxAny      < > = Box< dyn Any          + Send + Sync         > ;
pub type BoxRecipient<M> = Box< dyn Recipient<M> + Send + Sync + Unpin > ;

#[ cfg( feature = "remote" ) ]
//
pub type BoxServiceMap<MS> = Box< dyn ServiceMap<MS> + Send + Sync > ;



pub type ThesRes<T> = Result<T, Error>;


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
		bytes :: { Bytes, BytesMut                 } ,
		serde :: { Serialize, de::DeserializeOwned } ,
	};
}




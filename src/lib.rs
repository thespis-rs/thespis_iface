#![ feature( await_macro, async_await, arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias, box_syntax, box_patterns, associated_type_defaults ) ]


mod actor     ;
mod address   ;
mod envelope  ;
mod error     ;
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
	error     :: * ,
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


use std::{ pin::Pin, future::Future, any::Any };
//
pub type Return      <'a, R> = Pin<Box< dyn Future<Output = R> + 'a + Send >> ;
pub type ReturnNoSend<'a, R> = Pin<Box< dyn Future<Output = R> + 'a        >> ;


pub type BoxEnvelope <A>    = Box< dyn Envelope<A>               + Send                > ;
pub type BoxAny      < >    = Box< dyn Any                       + Send + Sync         > ;
pub type BoxRecipient<M, E> = Box< dyn Recipient<M, SinkError=E> + Send + Sync + Unpin > ;
pub type ThesRes<T>         = Result< T, ThesErr >;


mod import
{
	pub use
	{
		std::
		{
			fmt                           ,
			sync    :: Arc                ,
			pin     :: Pin                ,
			future  :: Future             ,
			convert :: TryFrom            ,
			hash    :: Hash               ,
			any     :: Any                ,
		},

		futures :: { prelude::{ FutureExt, Stream, Sink }, channel::{ oneshot, mpsc }, task::Spawn } ,
		failure   :: { Fail, bail, err_msg, AsFail, Context as FailContext, Backtrace, ResultExt } ,
	};
}




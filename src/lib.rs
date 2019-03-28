#![ allow( unused_imports, dead_code ) ]
#![ feature( await_macro, async_await, futures_api, arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias ) ]


mod actor     ;
mod address   ;
mod envelope  ;
mod handler   ;
mod mailbox   ;
mod message   ;

pub use
{
	actor     :: * ,
	address   :: * ,
	envelope  :: * ,
	handler   :: * ,
	mailbox   :: * ,
	message   :: * ,
};


#[ cfg( feature = "derive" ) ]
//
pub use thespis_derive::{ Actor };


use std::{ pin::Pin, future::Future };
//
pub type Response<'a, M> = Pin<Box< dyn Future< Output = <M as Message>::Result > + Send + 'a> >;


mod import
{
	pub use
	{
		std     :: { sync::Arc, pin::Pin, future::Future } ,
		futures :: { prelude::{ Stream, Sink }, channel::{ oneshot, mpsc } } ,
	};
}




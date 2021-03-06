#![ cfg_attr( nightly, cfg_attr( nightly, doc = include_str!("../README.md") )) ]
#![ doc = "" ] // empty doc line to handle missing doc warning when the feature is missing.
//
#![ doc    ( html_root_url = "https://docs.rs/thespis" ) ]
#![ deny   ( missing_docs                              ) ]
#![ forbid ( unsafe_code                               ) ]
#![ allow  ( clippy::suspicious_else_formatting        ) ]

#![ warn
(
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


mod actor     ;
mod address   ;
mod handler   ;
mod identify  ;
mod message   ;

pub use
{
	actor     :: * ,
	address   :: * ,
	handler   :: * ,
	identify  :: * ,
	message   :: * ,
};


// address.send requires futures::sink::SinkExt.
// let's publicly re-export that.
//
pub use futures_sink::{ Sink };


#[ cfg( feature = "derive" ) ] pub use thespis_derive::{ Actor, async_fn, async_fn_local };


use std::{ pin::Pin, future::Future, any::Any };

/// A boxed future that is `Send`, shorthand for async trait method return types.
//
pub type Return<'a, R> = Pin<Box< dyn Future<Output = R> + 'a + Send >>;

/// A boxed future that is not `Send`, shorthand for async trait method return types.
//
pub type ReturnNoSend<'a, R> = Pin<Box< dyn Future<Output = R> + 'a >>;


/// Shorthand for a boxed [`Address`] that is Send.
//
pub type BoxAddress<M, E> = Box< dyn Address<M, Error=E> + Send + Unpin >;


mod import
{
	pub(crate) use
	{
		std :: { sync::Arc, rc::Rc, fmt } ,
	};
}




// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//!
#![ cfg_attr( feature = "external_doc", feature(external_doc)         ) ]
#![ cfg_attr( feature = "external_doc", doc(include = "../README.md") ) ]
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


// address.send now requires futures::sink::SinkExt.
// let's publicly re-export that.
//
pub use futures::sink::{ Sink, SinkExt };


#[ cfg( feature = "derive" ) ] pub use thespis_derive::{ Actor, async_fn, async_fn_nosend };


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
		std       :: { sync::Arc, fmt    } ,
		futures   :: { future::FutureExt } ,
	};
}




#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../README.md") ]

#![ doc    ( html_root_url = "https://docs.rs/thespis" ) ]
#![ deny   ( missing_docs                              ) ]
#![ forbid ( unsafe_code                               ) ]
#![ allow  ( clippy::suspicious_else_formatting        ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
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
/// Re-export from the futures library.
///
//
pub use futures_sink::{ Sink };


#[ cfg( feature = "derive" ) ] pub use thespis_derive::{ Actor, async_fn, async_fn_local };


use std::{ pin::Pin, future::Future };

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




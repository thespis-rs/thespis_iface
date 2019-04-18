use crate :: { import::*, * };


pub trait Recipient< M: Message >
{
	fn send( &mut self, msg: M ) -> Response< ThesRes<()> >

		where M: Message<Result = ()>,

	;

	fn call( &mut self, msg: M ) -> Response< ThesRes<<M as Message>::Result> >;

	fn clone_box( &self ) -> Box< dyn Recipient<M> >;
}


pub trait ThreadSafeRecipient<M>

	where  M                    : Message + Send,
	      <M as Message>::Result: Send          ,

{
	fn send( &mut self, msg: M ) -> ThreadSafeResponse< ThesRes<()> >

		where M: Message<Result = ()>

	;

	fn call( &mut self, msg: M ) -> ThreadSafeResponse< ThesRes<<M as Message>::Result> >;

	fn clone_box( &self ) -> Box< dyn ThreadSafeRecipient<M> >;
}



/// An Recipient that allows you to send messages to actors in a different process/host.
///
#[ cfg( feature = "remote" ) ]
//
use serde::{ Serialize, de::DeserializeOwned };

#[ cfg( feature = "remote" ) ]
//
pub trait RemoteRecipient<M>

	where  M                    : Message + Serialize + DeserializeOwned,
	      <M as Message>::Result: Serialize + DeserializeOwned          ,

{
	fn send( &mut self, msg: M ) -> Response< ThesRes<()> >

		where M: Message<Result = ()>

	;

	fn call( &mut self, msg: M ) -> Response< ThesRes<<M as Message>::Result> >;

	fn clone_box( &self ) -> Box< dyn RemoteRecipient<M> >;
}


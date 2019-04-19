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



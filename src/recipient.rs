use crate :: { import::*, * };


pub trait Recipient< M: Message >
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn send( &mut self, msg: M ) -> Response< ThesRes<()> >;



	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Response< ThesRes<<M as Message>::Result> >;

	fn clone_box( &self ) -> Box< dyn Recipient<M> >;
}


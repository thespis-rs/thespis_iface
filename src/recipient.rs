use crate :: { import::*, *, thread_safe::{ Return, BoxRecipient } };


pub trait Recipient< M: Message >
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn send( &mut self, msg: M ) -> Return< ThesRes<()> >;



	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return< ThesRes<<M as Message>::Return> >;

	fn clone_box( &self ) -> BoxRecipient<M>;
}


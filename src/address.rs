use crate :: { import::*, * };


/// An address that allows you to send messages to an actor
///
pub trait Address< A: Actor > : Clone
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn send<M>( &mut self, msg: M ) -> Response< ThesRes<()> >

		where A: Handler< M >           ,
		      M: Message< Result = () > ,
	;


	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call<M: Message>( &mut self, msg: M ) -> Response< ThesRes<<M as Message>::Result> >

		where A: Handler< M >,
	;


	fn recipient<M>( &self ) -> Box< dyn Recipient<M> >

		where M: Message    ,
		      A: Handler<M> ,
	;
}


/// An Address that allows you to send messages to actors in different threads
/// This incurs more overhead than the normal Address.
///
pub trait ThreadSafeAddress< A: Actor > : Clone
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn send<M>( &mut self, msg: M ) -> ThreadSafeResponse< ThesRes<()> >

	where  A                    : Handler<M>                    ,
	       M                    : Message< Result = () > + Send ,
	      <M as Message>::Result: Send                          ,

	;


	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call<M: Message>( &mut self, msg: M ) -> ThreadSafeResponse< ThesRes<<M as Message>::Result> >

	where  A                    : Handler<M>     ,
	       M                    : Message + Send ,
	      <M as Message>::Result: Send           ,

	;

	fn recipient<M>( &self ) -> Box< dyn ThreadSafeRecipient<M> >

		where  A                    : Handler<M>     ,
		       M                    : Message + Send ,
		      <M as Message>::Result: Send           ,
	;
}


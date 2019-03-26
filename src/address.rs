use crate :: { import::*, * };


pub trait Address< A: Actor + Send >
{
	fn new( mb: mpsc::UnboundedSender<Box< dyn Envelope<A> >> ) -> Self;

	fn send<M>( &mut self, msg: M )

		where A: Handler< M >,
		      M: Message<Result = ()> + Send + 'static,

	;

	fn call<M: Message + 'static>( &mut self, msg: M ) -> Pin<Box< dyn Future< Output = M::Result > + Send + '_ > >

		where A: Handler< M >,
		      M: Send        ,
		      M::Result: Send,

	;
}


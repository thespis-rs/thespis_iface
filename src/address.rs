use crate :: { import::*, * };


pub trait Address< A: Actor >
{
	fn send<M>( &mut self, msg: M ) -> Pin<Box< Future<Output=()> + '_>>

		where A: Handler< M >,
		      M: Message<Result = ()> + 'static,

	;

	fn call<M: Message + 'static>( &mut self, msg: M ) -> Pin<Box< dyn Future< Output = M::Result > + '_ > >

		where A: Handler< M >,

	;
}


pub trait ThreadSafeAddress< A: Actor > : Address<A>
{
	fn send<M>( &mut self, msg: M ) -> Pin<Box< Future<Output=()> + '_>>

		where A: Handler< M >,
		      M: Message<Result = ()> + Send + 'static,

	;

	fn call<M: Message + 'static>( &mut self, msg: M ) -> Pin<Box< dyn Future< Output = M::Result > + Send + '_ > >

		where A: Handler< M >,
		      M: Send        ,
		      M::Result: Send,

	;
}


use crate :: { import::*, * };


pub trait Address< A: Actor >
{
	fn send<M>( &mut self, msg: M ) -> TupleResponse

		where A: Handler< M >,
		      M: Message<Result = ()> + 'static,

	;

	fn call<M: Message + 'static>( &mut self, msg: M ) -> Response<M>

		where A: Handler< M >,

	;
}


pub trait ThreadSafeAddress< A: Actor > : Address<A>
{
	fn send<M>( &mut self, msg: M ) -> ThreadSafeTupleResponse

		where A: Handler< M >,
		      M: Message<Result = ()> + Send + 'static,

	;

	fn call<M: Message + 'static>( &mut self, msg: M ) -> ThreadSafeResponse<M>

		where A: Handler< M >,
		      M: Send        ,
		      M::Result: Send,

	;
}


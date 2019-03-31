use crate :: { import::*, * };


pub trait Recipient< M: Message + 'static >
{
	fn send( &mut self, msg: M ) -> TupleResponse

		where M: Message<Result = ()>,

	;

	fn call( &mut self, msg: M ) -> Response<M>;
}


pub trait ThreadSafeRecipient<M>

	where  M                    : Message + Send + 'static,
	      <M as Message>::Result: Send                    ,

{
	fn send( &mut self, msg: M ) -> ThreadSafeTupleResponse

		where M: Message<Result = ()>

	;

	fn call( &mut self, msg: M ) -> ThreadSafeResponse<M>;
}


use crate :: { import::*, * };


pub trait Recipient< M: Message + 'static >
{
	fn send( &mut self, msg: M ) -> TupleResponse

		where M: Message<Result = ()> + 'static,

	;

	fn call( &mut self, msg: M ) -> Response<M>;
}


pub trait ThreadSafeRecipient< M: ThreadSafeMessage + 'static >
{
	fn send( &mut self, msg: M ) -> ThreadSafeTupleResponse

		where M: ThreadSafeMessage<Result = ()> + 'static,
		      <M as Message>::Result: Send,

	;

	fn call( &mut self, msg: M ) -> ThreadSafeResponse<M>

		where M        : ThreadSafeMessage ,
		      <M as Message>::Result: Send,

	;
}


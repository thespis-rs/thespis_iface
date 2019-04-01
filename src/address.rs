use crate :: { import::*, * };


pub trait Address< A: Actor > : Clone
{
	fn send<M>( &mut self, msg: M ) -> TupleResponse

		where A: Handler< M >           ,
		      M: Message< Result = () > ,
	;


	fn call<M: Message>( &mut self, msg: M ) -> Response<M>

		where A: Handler< M >,
	;


	fn recipient<M>( &self ) -> Box< dyn Recipient<M> >

		where M: Message   ,
		      A: Handler<M>          ,
	;
}


pub trait ThreadSafeAddress< A: Actor > : Clone
{
	fn send<M>( &mut self, msg: M ) -> ThreadSafeTupleResponse

	where  A                    : Handler<M>                              ,
	       M                    : Message< Result = () > + Send ,
	      <M as Message>::Result: Send                                    ,

	;

	fn call<M: Message>( &mut self, msg: M ) -> ThreadSafeResponse<M>

	where  A                    : Handler<M>     ,
	       M                    : Message + Send ,
	      <M as Message>::Result: Send           ,

	;

	fn recipient<M>( &self ) -> Box< dyn ThreadSafeRecipient<M> >

		where  A                    : Handler<M>     ,
		       M                    : Message + Send ,
		      <M as Message>::Result: Send                     ,
	;
}


use crate :: { import::*, * };

pub trait Handler< M: Message > where Self: Actor
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> Response< <M as Message>::Result >;
}

pub trait ThreadSafeHandler<M>

	where

		Self     : Actor          ,
		M        : Send + Message ,
		M::Result: Send           ,

{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> ThreadSafeResponse< <M as Message>::Result >;
}

use crate :: { import::*, * };

pub trait Handler< M: Message > where Self: Actor
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> Return< <M as Message>::Result >;
}

pub trait ThreadSafeHandler<M>

	where

		Self     : Actor          ,
		M        : Send + Message ,
		M::Result: Send           ,

{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> ThreadSafeReturn< <M as Message>::Result >;
}

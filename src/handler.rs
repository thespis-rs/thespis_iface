use crate :: { import::*, * };

pub trait Handler< M: Message > where Self: Actor
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> Return< <M as Message>::Return >;
}

// pub trait ThreadSafeHandler<M>

// 	where

// 		Self     : Actor          ,
// 		M        : Send + Message ,
// 		M::Return: Send           ,

// {
// 	#[ must_use = "Futures do nothing unless polled" ]
// 	//
// 	fn handle( &mut self, msg: M ) -> thread_safe::Return< <M as Message>::Return >;
// }

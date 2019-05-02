use crate :: { * };

pub trait Handler< M: Message > where Self: Actor
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> ReturnNoSend< <M as Message>::Return >;
}

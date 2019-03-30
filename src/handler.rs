use crate :: { import::*, * };

pub trait Handler< M: Message > where Self: Actor
{
	fn handle( &mut self, msg: M ) -> Response<M>;
}

pub trait ThreadSafeHandler< M: Message > where Self: Actor
{
	fn handle( &mut self, msg: M ) -> ThreadSafeResponse<M>;
}

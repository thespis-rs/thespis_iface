use crate :: { import::*, * };


pub trait Mailbox< A: Actor >
{
	fn start( &mut self, actor: A ) -> TupleResponse;
}

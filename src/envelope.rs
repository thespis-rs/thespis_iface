use crate :: { import::*, * };


pub trait Envelope< A > where A: Actor
{
	fn handle( self: Box<Self>, actor: &mut A ) -> TupleResponse;
}


pub trait ThreadSafeEnvelope< A > : Send where A: Actor + Send
{
	fn handle( self: Box<Self>, actor: &mut A ) -> ThreadSafeTupleResponse;
}

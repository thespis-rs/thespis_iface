use crate :: { import::*, * };


pub trait Envelope< A > where A: Actor
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( self: Box<Self>, actor: &mut A ) -> Return<()>;
}

use crate :: { import::*, * };


pub trait Envelope< A > : Send where A: Actor + Send
{
	fn handle( self: Box<Self>, actor: &mut A ) -> Pin<Box< dyn Future< Output = () > + Send + '_> >;
}

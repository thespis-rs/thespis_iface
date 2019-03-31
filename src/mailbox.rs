use crate :: { import::*, * };


pub trait Mailbox< A: Actor >
{
	fn start( self, actor: A ) -> Pin<Box< dyn Future<Output = ()> >>;
}

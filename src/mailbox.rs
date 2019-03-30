use crate :: { import::*, * };


pub trait Mailbox< A: Actor >
{
	fn start( &mut self, actor: A ) -> Pin<Box< Future<Output=()> + '_>>;
}

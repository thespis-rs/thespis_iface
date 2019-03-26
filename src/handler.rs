use crate :: { import::*, * };

pub trait Handler< M: Message > where Self: Actor
{
	fn handle( &mut self, msg: M ) -> Pin<Box< dyn Future< Output = M::Result > + Send + '_> >;
}

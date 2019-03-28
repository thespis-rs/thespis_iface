use crate :: { import::*, * };


pub trait Mailbox< A: Actor + Send >
{
	fn new( actor: A, exec: &mut impl Spawn ) -> Self;


	fn addr< Adr >( &mut self ) -> Adr

		where Adr: Address<A>

	;

}

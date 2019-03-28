use crate :: { *, import::* };

pub trait Actor

	where Self: Sized + Send

{
	fn start<Mb>( self, exec: &mut impl Spawn ) -> Mb where Mb: Mailbox< Self >
	{
		Mb::new( self, exec )
	}
}


use crate :: { * };

pub trait Actor

	where Self: Sized + Send

{
	fn start<Mb>( self ) -> Mb where Mb: Mailbox< Self >
	{
		Mb::new( self )
	}
}


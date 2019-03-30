use crate :: { *, import::* };

pub trait Actor

	where Self: Sized + Send

{
	// Currently doesn't work because it drops the future returned from mb.start.
	//
	fn start( self, mb: &mut impl Mailbox<Self> )
	{
		mb.start( self );
	}


	fn started (){}
	fn stopping(){}
	fn stopped (){}
}


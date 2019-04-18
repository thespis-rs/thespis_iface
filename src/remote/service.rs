use crate::{ import::*, * } ;


pub trait Service: Message
{
	type UniqueID: UniqueID;

	fn sid() -> Self::UniqueID;


	fn uid( seed: &[u8] ) -> Self::UniqueID;

	//fn recipient( peer: impl Address<impl Router> ) -> Recipient<Self> ;
}


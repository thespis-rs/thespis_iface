use crate::{ import::*, * } ;


pub trait Service

	where  Self                    : Message + Serialize + DeserializeOwned,
         <Self as Message>::Result:           Serialize + DeserializeOwned,
{
	type UniqueID: UniqueID;

	fn uid( seed: &[u8] ) -> Self::UniqueID;

	//fn recipient( peer: impl Address<impl Router> ) -> Recipient<Self> ;
}


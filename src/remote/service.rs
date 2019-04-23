use crate::{ import::*, * } ;


/// A [Message] that can be received from remote code. Mainly defines that this [Message] type can has
/// a unique id which allows distinguishing it from other services. It is namespaced, so that different
/// components/processes can expose services to the network which will accept the same [Message] type,
/// yet give them a unique identifier.
///
pub trait Service<Namespace>

	// TODO: make peace with serde and figure out once and for all when it makes sense
	//       to create a bound to Deserialize<'de> and when to use DeserializeOwned.
	//
	where  Self                    : Message + Serialize + DeserializeOwned,
         <Self as Message>::Return:           Serialize + DeserializeOwned,
{
	type UniqueID: UniqueID;

	/// The unique service id. It needs to be static. You can create runtime static data with
	/// lazy_static or OnceCell. That way it will only have to be generated once per service per
	/// process run. Even better is to be able to generate it from const code so it has no runtime
	/// overhead.
	///
	/// For a given Service and Namespace, the output should always be the same, even accross processes
	/// compiled with different versions of rustc. Ideally the algorithm is also clearly described so
	/// programs written in other languages can also communicate with your services.
	//
	fn sid() -> &'static Self::UniqueID where Self: Sized;
}


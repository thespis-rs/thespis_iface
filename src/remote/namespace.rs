/// Trait requiring the presence of a &'static string constant with the name of the namespace (service map)
/// a service lives in. [Service] is generic over this trait.
//
pub trait Namespace
{
	const NAMESPACE: &'static str;
}

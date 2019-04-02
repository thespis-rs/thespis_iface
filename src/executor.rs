use crate :: { import::*, * };

/// Methods can't take generic parameters, because we need to be able to turn this into a trait
/// object. That's why we have to take pinned boxed futures.
///
/// Methods cannot take mutable references to self, because we store dyn Executor in a threadlocal
/// static for convenience of the API. We could put it in a RefCell, but this would make the borrows
/// really coarse which might be troublesome for implementations, eg. spawn needs to work within run.
///
// FIXME: Currently the runtime is not defined by thespis_iface. It only exists in thespis_impl.
//        That makes it strange that this trait is here instead of in thespis_impl, but it allows libraries
//        to provide executor impls without having to depend on thespis_impl.
//
//        Try to make this so it works for both Send and !Send, for features generic over Output.
//
pub trait Executor
{
	/// Spawn a future on a threadpool
	///
	fn spawn_pool( &self, fut: Pin<Box< dyn Future< Output = () > + 'static >> ) -> ThesRes<()>;

	/// Spawn a future without creating new threads, and without paying for thread synchronization
	///
	fn spawn( &self, fut: Pin<Box< dyn Future< Output = () > + 'static >> ) -> ThesRes<()>;

	/// Block the current thread and run the given future and all the tasks it spawned. Return when the
	/// original future and all tasks spawned by it have resolved.
	///
	fn run( &self );
}

# thespis_iface
The interface of the thespis actor model (contains only traits).

## References:

- [Actor Model of Computation: Scalable Robust Information Systems](https://arxiv.org/abs/1008.1459) by Carl Hewitt.
- video: [Hewitt, Meijer and Szyperski: The Actor Model (everything you wanted to know...)](https://youtu.be/7erJ1DV_Tlo)

## TODO:

- fix clone on recipient like so: https://stackoverflow.com/revisions/30353928/3
- oblige all implementors of all traits to implement Debug?
- go over actix features and see what would be useful to have (backpressure?)
- defaults for associated types, like () for Message::Return, and possibility to derive with defaults
- how can an actor stop itself, should mb be fed to handle, should there be a method self.mb, should there be a stop method on mailbox
- do some proper research on tokio reactor. Just figured out we don't need a tokio runtime to use stuff that uses epoll. A futures 0.3 executor will do just fine, just using compat on the futures and streams from tokio.
- on generic impls, tag methods as default, so that users can override them for specific types.
- check mut requirements. we require mut in alot of places, like when sending on an address the address has to be mut. Should we relieve certain of those. It means for example that a struct which holds an addr must also be mut or put it in Refcell just to send messages.
- think about and write up the difference in Send + reply_to and Call. Performance, possiblity to pass on reply_to, link the outgoing send to the incoming response? Possibility to send a reply_to address that is not your own?
- enable CI testing
- enable dependabot
- code coverage?
- impl Sink for references? &'a Addr<A>
- currently spawned tasks swallow panics... We should do something about that. Verify, I think it's only when remote handle gets dropped
- try to relax pinning requirements where we can and impl unpin for things like addr to avoid
  forwarding pinning requirements to Actor.
- check TODO's and FIXME's in code


## Design issues:

- The wire format is a hand baked solution just to get it working. Now we should find out what the final formats might look like. Cap'n proto? or SBE? : https://polysync.io/blog/session-types-for-hearty-codecs

- notion of "one message at a time". This is only necessary if the actor has mutable state. If the actor works without mutable state, it can be run in parallel. In thespis there is currently no way to create this optimization. In actix this is represented by sync actors, where you spawn several of them on different threads and then have one address for the lot. It would be nice if we could have like a mailbox type which shall give you only an immutable reference to self, and now it will process messages in parallel. This will however require a different Handler trait, and that's not very elegant.

- Address is capability. Currently we work on "in process" stuff (the programmer compiling is deemed responsible for the outcome), so not much security considerations are in place. And we work on cross process communication where we consider an all public interface. A process provides services, and if it accepts a connection, it specifies which services will respond on that connection, but it's not more fine grained than that ATM. Eg. We have not implemented any authentication/authorization/encryption/signing.
Consider how you make something available to some entity that is authorized only.

- Two big axises are difficult to implement DRY:
  - Send vs !Send messages, see: https://users.rust-lang.org/t/how-do-you-all-make-your-dynamic-code-send-agnostic/27567/5
  - mut vs not mut actor: would allow processing messages in parallel, but needs a different Handler trait, and Handler trait is used throughout as trait bound A: Handler<M>

- our generic story does not work for remote services right now. Both service map and Peer cannot really
  handle other types. We should work on that and test it. The service_map macro has hardoded references to ServiceID, Peer and needs to create Multiservice to send errors back to the remote.




## TODO:

- issues to improve:
  - make running concurrent handlers more ergonomic.

- Ideas from act-zero:
  - does not require boxing if the response is immediately ready in a handler. For that it uses an enum.
  - automates concurrncy of handlers that don't need mut state based on receiver, &self, &mut self, Addr<Local<Self>>.


- thespis_remote RemoteAddr is a different type then thespis::Addr. How transparent is this for combining local and remote actors. It is a different type and boxing it won't help here since the error type on the Sink is different. We should play around with this and see if we can smoothen that out. Update guide when we know what the final score is.


- !Send Messages.

- performance:
  - channel impls
  - thread sync
  - heap allocations

- go over actix features and see what would be useful to have, and at least list the things we don't have.
- polish async_chanx
- write docs and guide

- Actor should also require AssertUnwindSafe, but currently types from other libraries that use things like `UnsafeCell<Option<AtomicUsize>>`, like tokio channels will not implement that even if they probably should, which makes it to much of a hassle right now. Opened one issue at futures: https://github.com/rust-lang/futures-rs/issues/2211.



## Blocked

- defaults for associated types, like () for Message::Return, and possibility to derive with defaults
  blocked on stabilization of defaults for associated types.

- on generic impls, tag methods as default, so that users can override them for specific types.
  blocked on stabilization of specialization.

- async in traits

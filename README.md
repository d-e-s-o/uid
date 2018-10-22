uid
===

- [Documentation][docs-rs]

**uid** (short for **u**nique **id**entifiers) is a crate for creating
unique IDs for usage in a Rust program. IDs can have many purposes in
programs, but this crate's IDs are mostly useful for identification
tasks. For example, consider having some form of simple in-memory
database (in the form of a `HashMap` or something more sophisticated).
IDs as created by this crate make for perfect primary keys.

Said IDs have a couple properties that, depending on the context, are
useful to have:
1) IDs are lightweight and ultimately just a number with some compile
   time magic (and a small creation cost) attached to them.
2) Created IDs are guaranteed to be unique. That is, when created at
   runtime (deserialization is not supported for obvious reasons), two
   newly crated IDs are guaranteed to never be the same.
3) They are fully thread safe. That is, uniqueness is guaranteed even
   among different threads.
4) IDs are immutable. Once created, an ID can be copied, hashed,
   compared, and more, but it cannot be changed.
5) IDs can form name (or rather type) spaces. That is, an ID can be
   parametrized by an arbitrary (potentially private) type and only IDs
   that are parametrized by the same type can interact (i.e., be
   compared etc.).


[docs-rs]: https://docs.rs/crate/uid

Entity Rust
==========

This project is not functional yet, just some design ideas.

### Note to self
We use a bunch of mutexes internally to ensure proper initialization.
During runtime though these are basically read-only so could definitely
be optimized away. We should attempt to make these mutexes read/write
aware or actually optimize them away somehow.

(Note that every lazy_static! call has mutexes (also implicitly))

# Library-First

When building binary crates, the crate should be designed from the ground up to
be a library exposing the necessary functionality, and a thin wrapper around it
that is a binary which excises the functionality.

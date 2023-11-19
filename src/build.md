# Build system

This chapter discusses some build systems that play nice with Rust. Having a good
build system is important, because you do not want to end up with a complicated mess
of Bash scripts that are required to be run in the right order to build your project
successfully.

Note that build systems are not necessarily mutually-exclusive: most of the time,
even when using a build system that is not Cargo, you will still have the necessary
Cargo manifests in the project that allows standard Cargo tooling to work.

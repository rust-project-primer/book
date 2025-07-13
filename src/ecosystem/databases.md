# Databases

Many applications need to store and query structured data in some way. Whether
it is some backend application that needs to handle thousands of requests per
seconds, or a local application that needs to store some configuration in a
local database, there is a lot of choice and different scales for databases.

[Are We Web Yet: Database](https://www.arewewebyet.org/topics/database/) tracks
the progress and the ecosystem around databases on Rust. It makes sense to check
back there to follow new developments. The
[databases tag on crates.io](https://crates.io/keywords/database) is also a good
place to discover new, popular database-related crates.

## Embedded Databases

Embedded databases are commonly deployed to store structured data locally, such
as settings, on resource-constrainted devices, or on edge deployed applications.
They typically do not operate over the network, but rather interact directly
with a local database file.

- sqlite
- sled

## Database Connectors

- mysql
- postgres

## Database Frameworks

- SQLx
- Diesel

See also: https://www.arewewebyet.org/topics/database/

# Serialization

Serialization is the process of turning structured data into a flat format,
usually textual (JSON, YAML, TOML) or binary (MessagePack, Bincode, Postcard).
Typically this is done to save data (on disk, in a database) or exchange it
(between processes, between services over a network). Deserialization is the
inverse: turning a flat representation back into structured data.

For example:

- When you read a config file from disk and parse it, you are deserializing it.
- When you make an API request and send JSON-encoded data, you are serializing
  it.

One important distinction between serialization formats is whether they are
_self-describing_ or not. A self-describing format like JSON includes the field
names in the serialized output, so a reader can understand the structure without
knowing the schema ahead of time. A non-self-describing format like Bincode or
Postcard omits this information and relies on both sides agreeing on the schema,
which makes the output smaller and faster to parse but less flexible.

Rust has several popular crates for serialization. The dominant one is `serde`,
which most of the ecosystem is built around. There are also alternatives that
make different tradeoffs.

| Crate     | Description                                                       |
| --------- | ----------------------------------------------------------------- |
| serde     | General-purpose serialization framework with broad format support |
| facet     | Reflection-based approach that avoids monomorphization            |
| miniserde | Lightweight serde alternative with smaller code size              |
| bincode   | Binary serialization for inter-process communication              |

## Serde

[Serde][serde] (short for **ser**ialize/**de**serialize) is the standard
serialization framework in Rust. It works through two traits, `Serialize` and
`Deserialize`, which you derive on your types. Format-specific crates then
provide serializers and deserializers that work with any type implementing these
traits.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    timeout: u64,
    verbose: bool,
}
```

This single derive gives you access to every format that serde supports. You can
serialize this struct to JSON, YAML, TOML, MessagePack, or any other format by
choosing the appropriate crate:

| Crate      | Format      | Self-describing |
| ---------- | ----------- | --------------- |
| serde_json | JSON        | Yes             |
| serde_yaml | YAML        | Yes             |
| toml       | TOML        | Yes             |
| postcard   | Postcard    | No              |
| bincode    | Bincode     | No              |
| csv        | CSV         | Partially       |
| rmp-serde  | MessagePack | Yes             |
| ciborium   | CBOR        | Yes             |

You can find a more complete list of supported formats on the
[serde website](https://serde.rs/#data-formats).

### Default Values

When deserializing, you can provide default values for fields that may be
missing from the input. This is useful for configuration files where you want
sensible defaults:

```rust
#[derive(Deserialize)]
struct Config {
    name: String,
    #[serde(default = "default_timeout")]
    timeout: u64,
    #[serde(default)]
    verbose: bool,
}

fn default_timeout() -> u64 {
    30
}
```

The `#[serde(default)]` attribute uses the type's `Default` implementation,
while `#[serde(default = "...")]` calls a specific function.

### Renaming Fields

Rust conventions use `snake_case` for field names, but many formats use
`camelCase` or `PascalCase`. Serde lets you rename fields in the serialized
output without changing your Rust code:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    user_name: String,      // serialized as "userName"
    created_at: String,     // serialized as "createdAt"
}
```

You can also rename individual fields:

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "type")]
    kind: String,  // "type" is a reserved keyword in Rust
}
```

### Versioned Structs

As your application evolves, the shape of your serialized data may change. Serde
provides several tools for handling this gracefully:

- `#[serde(default)]` on fields lets you add new fields without breaking
  existing data, since missing fields get their default value.
- `#[serde(deny_unknown_fields)]` on the struct rejects input that contains
  fields your struct doesn't know about, which is useful for catching typos in
  configuration files.
- `#[serde(alias = "old_name")]` lets you accept both old and new field names
  during a migration period.

For more complex schema migrations, you may need to deserialize into an
intermediate representation (such as `serde_json::Value`) and transform it
before deserializing into the final struct.

One pattern is to version your structs by using an internally tagged enum. Each
variant is a newtype containing a version-specific struct:

```rust
#[derive(Serialize, Deserialize)]
struct ConfigV1 {
    name: String,
    age: usize,
}

#[derive(Serialize, Deserialize)]
struct ConfigV2 {
    full_name: String,
    age: usize,
    address: String,
    zip_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
enum Config {
    #[serde(rename = "1")]
    V1(ConfigV1),
    #[serde(rename = "2")]
    V2(ConfigV2),
}
```

The `#[serde(tag = "version")]` attribute tells serde to use the `version` field
to determine which variant to deserialize into. This works with newtype variants
containing structs, so the inner struct's fields are merged into the JSON
object. With this setup, both of these JSON inputs would be accepted:

```json
{
  "version": "1",
  "name": "John Doe",
  "age": 42
}
```

```json
{
  "version": "2",
  "full_name": "John Doe",
  "age": 62,
  "address": "1042 Sweeny Drive",
  "zip_code": "18831"
}
```

But what if you previously did not use versioned structs, and you want to start
using them? You can combine `#[serde(untagged)]` with a tagged inner enum to
also accept legacy data that has no version field:

```rust
#[derive(Serialize, Deserialize)]
struct ConfigLegacy {
    name: String,
    id: u64,
}

#[derive(Serialize, Deserialize)]
struct ConfigV1 {
    name: String,
    age: usize,
}

#[derive(Serialize, Deserialize)]
struct ConfigV2 {
    full_name: String,
    age: usize,
    address: String,
    zip_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
enum ConfigVersioned {
    #[serde(rename = "1")]
    V1(ConfigV1),
    #[serde(rename = "2")]
    V2(ConfigV2),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Config {
    Versioned(ConfigVersioned),
    Legacy(ConfigLegacy),
}
```

The `#[serde(untagged)]` attribute means serde will try decoding the variants
one by one. Since legacy values don't have a `version` field, they will fail to
decode as `ConfigVersioned` and fall back to being decoded as `ConfigLegacy`.

### Preserving Unknown Fields

The `#[serde(flatten)]` attribute merges the fields of a nested struct into the
parent. One useful application of this is preserving unknown fields during
round-tripping. If you are parsing data from an API response that may gain new
fields in the future, and you want to make sure that deserializing and
re-serializing does not lose anything, you can capture the unknown fields into a
map:

```rust
use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    foo: String,
    bar: String,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}
```

Any JSON keys that don't match `foo` or `bar` are collected into `other`, and
when you serialize the struct back, those keys are included in the output.

### Custom Implementations

For most types, the derived `Serialize` and `Deserialize` implementations are
sufficient. When they are not, you can implement the traits manually. Common
reasons include:

- Serializing a type in a format that doesn't match its Rust structure (for
  example, serializing a `Duration` as a human-readable string like `"30s"`).
- Enforcing validation during deserialization (rejecting values that are
  syntactically valid but semantically wrong).
- Working with external types that don't implement serde traits.

Manual implementations use serde's `Serializer` and `Deserializer` visitor
pattern. This is more involved than deriving, but the
[serde documentation](https://serde.rs/impl-serialize.html) covers it well.

### Companion Crates

Several crates extend serde's functionality:

[`serde_with`][serde_with] provides custom field-level serialization helpers
through attributes. For example, serializing a `Duration` as seconds, a `Vec` as
a comma-separated string, or skipping serialization of `Option::None` values. It
saves you from writing manual trait implementations for common patterns.

[`serde_transcode`][serde_transcode] allows converting between serde formats
without an intermediate Rust type. For example, you can transcode JSON to YAML
directly, which is useful for format conversion tools.

If you manually implement Serialize and Deserialize, the `serde_test` crate can
be very helpful in testing that these work correctly.

## Protocol Buffers

[Protocol Buffers][protobuf] (protobuf) is Google's language-neutral
serialization format, widely used for RPC and inter-service communication.
Unlike serde, protobuf uses a separate schema definition (`.proto` files) that
is compiled into Rust code.

The two main Rust crates for protobuf are:

- [`prost`][prost]: Generates idiomatic Rust structs from `.proto` files. This
  is the most popular choice and integrates well with the `tonic` gRPC
  framework.
- [`protobuf`][rust-protobuf]: The official Google-maintained Rust
  implementation.

Protobuf is a good choice when you need cross-language interoperability with a
well-defined schema, especially if you are already using gRPC.

## Facet

[Facet][facet] takes a fundamentally different approach to serialization than
serde. Where serde generates specialized serialization code for each type
through monomorphization, facet uses _compile-time reflection_: the derive macro
generates metadata (a `Shape`) describing each type's structure, and generic
serialization code operates on this metadata at runtime.

```rust
use facet::Facet;

#[derive(Facet)]
struct Config {
    name: String,
    timeout: u64,
    verbose: bool,
}
```

The key tradeoff is explicit: facet trades some runtime performance (roughly
3-6x slower than serde for serialization) for significantly reduced compile
times. Because the serialization logic is not monomorphized per type, adding new
types or changing existing ones does not cause cascading recompilation of
serialization code. For large projects where compile times are a bottleneck,
this can be a meaningful improvement.

Facet is more than a serialization library. Because it provides runtime type
information, it can also be used for pretty-printing, diffing, CLI argument
parsing, and code generation for other languages. The facet ecosystem includes
crates like `facet-json`, `facet-toml`, `facet-yaml`, `facet-pretty`, and
`facet-diff`.

Facet is backed by AWS and the Zed editor team, and is under active development.
It is newer than serde and its ecosystem is smaller, but it represents an
interesting architectural alternative for projects where compile time matters
more than serialization throughput.

## Miniserde

[Miniserde][miniserde] is a minimal serialization library by the same author as
serde (David Tolnay). It deliberately supports only a subset of what serde can
do: JSON serialization and deserialization with no support for other formats, no
custom field attributes, and no generic type parameters.

In exchange, miniserde produces significantly less code through
monomorphization, resulting in smaller binaries and faster compile times. It is
a good choice for small tools or WebAssembly targets where binary size is a
primary concern and you only need JSON support.

## Conclusion

For most projects, serde is the right choice. It has the broadest format
support, the largest ecosystem, and most Rust crates that expose serializable
types already use it. The other crates are worth considering when you have
specific constraints:

- **Bincode** or **Postcard**: When you need compact binary serialization
  between Rust processes.
- **Protocol Buffers**: When you need cross-language interoperability with a
  schema-first approach, especially with gRPC.
- **Facet**: When compile times are a significant concern and you can accept
  slower runtime serialization.
- **Miniserde**: When binary size matters and you only need JSON.

## Reading

```reading
style: book
title: Serde
url: https://serde.rs/
author: David Tolnay
---
The serde book is a reference guide for how to use serde, lists the various
formats that serde can serialize and deserialize, and gives advice on using
advanced features.
```

```reading
style: article
title: "Rust serialization: What's ready for production today?"
url: https://blog.logrocket.com/rust-serialization-whats-ready-for-production-today
author: Andre Bogus
---
In this article, Andre goes through several serialization frameworks in Rust
and explains which ones are stable and reliable and fit for use in production
Rust applications.
```

```reading
style: article
title: "Introducing facet: Reflection for Rust"
url: https://fasterthanli.me/articles/introducing-facet-reflection-for-rust
author: Amos Wenger
---
Amos explains the motivation behind facet: serde's monomorphization causes
significant compile-time costs in large projects. Facet takes a different
approach by generating metadata rather than specialized code, trading runtime
performance for faster builds and additional capabilities like reflection.
```

[serde]: https://serde.rs/
[serde_with]: https://docs.rs/serde_with
[serde_transcode]: https://docs.rs/serde-transcode
[bincode]: https://docs.rs/bincode
[facet]: https://github.com/facet-rs/facet
[miniserde]: https://github.com/dtolnay/miniserde
[protobuf]: https://protobuf.dev/
[prost]: https://github.com/tokio-rs/prost
[rust-protobuf]: https://github.com/stepancheg/rust-protobuf

# Serialization

Serialization is the process of turning structured data into a flat format,
usually a textual or binary format. Typically this is done to save data (on
disk, in a database), exchange it (between processes, between services).
Deserialization is the process of doing the inverse: turning a flat
representation into a structured representation.

For example:

- When you read a config file from disk and parse it, you are deserializing it.
- When you make an API request and send JSON-encoded data, you are serializing
  it.

Performing serialization and deserialization is important for any program that
must communicate with the outside world.

Rust has some popular crates that are used for doing this. The most popular
crate is `serde` (which stands for *ser*ialize, *de*serialize). Many Rust crates
have optional feature flags that make their types compatible with it.

| Name      | Description                                                                                                                                                                          |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| serde     | General purpose serialization and deserialization library.                                                                                                                           |
| miniserde | Serialization and deserialization library that is designed to be similar to serde, while doing less monomorphization and thereby producing smaller code.                             |
| bincode   | Binary serialization library, designed for inter-process communication. It is compatible with serde, but also has the option of using its own traits for more control of the layout. |
| facet     | Reflection library that is able to use the information about types to serialize and deserialize them.                                                                                |

## Serde

Serde supports a variety of formats. You can find a list of supported formats
[here](https://serde.rs/#data-formats).

| Crate      | Format   |
| ---------- | -------- |
| serde_json | JSON     |
| postcard   | Postcard |
| bincode    | Bincode  |
| csv        | CSV      |
| serde_yaml | YAML     |

### Versioned Structs

### Default Values

### Renaming Fields

### Custom Implementations

### Plugins

- serde_with
- serde_transcode

## Bincode

Bincode is a binary serialization library, designed for inter-process
communication. It is compatible with serde, but also has the option of using its
own traits for more control of the layout.

## Facet

## Miniserde

## Conclusion

If you don't have any specific requirements, sticking to `serde` is a good
choice. The other crates are more specialized and are useful if you have
specific requirements. Most of the time, the deciding factor is compatibility
with the ecosystem. Many existing crates already use `serde` for their
serialization needs, or have a feature flag that you can enable which makes
their types compatibhttps://serde.rs/#data-formatsle with `serde`.

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

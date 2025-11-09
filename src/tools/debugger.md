# Debugging

## Binary Debugging

https://rustc-dev-guide.rust-lang.org/debugging-support-in-rustc.html

## Time-travel Debugging

The [rr](https://rr-project.org/) debugger allows you to record and replay the execution of a
program. This is useful for debugging asynchronous applications, as it allows you to step through
the execution of the program at a slower pace, and inspect the state of the program at any point in
time.

https://gist.github.com/spacejam/15f27007c0b1bcc1d6b4c9169b18868c

## Tokio Console

https://github.com/tokio-rs/console/

## Reading

```reading
style: article
title: "Road to TurboWish Part 3: Design"
url: https://blog.pnkfx.org/blog/2021/05/03/road-to-turbowish-part-3-design/
author: Felix S. Klock II
---
In this article, Felix explains the design of TurboWish, a tool to debug
asynchronous applications. This design would eventually become the Tokio
console.
```

```reading
style: article
title: using rust with rr
url: https://gist.github.com/spacejam/15f27007c0b1bcc1d6b4c9169b18868c
author: Tyler Neely
---
TODO
```

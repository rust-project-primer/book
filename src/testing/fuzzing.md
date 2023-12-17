# Fuzzing

Fuzzing is an approach to testing code that generates random inputs for your
code and uses instrumentation to monitor which branches are being triggered,
with the goal of triggering all branches inside the code. In doing so, it can
test your code very thoroughly and often times discover edge cases.

Fuzzing is a very good strategy when your code parses untrusted data. It allows
you to have confidence that for any possible input, your program does not
misbehave.  The downside of fuzzing is that usually, it can only detect
crashes. When possible, it is better to test individual pieces of code
using property testing.

## Examples

*TODO*

## Reading

- [Rust Fuzz Book](https://rust-fuzz.github.io/book/introduction.html)

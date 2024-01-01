# Conclusion

In this chapter, we have discussed many of the options that Rust has for testing
code. We have discussed some strategies of how to split tests into different
tiers, so that local development can be snappy while testing runs in CI can be
extensive. We have looked at some randomized testing methods like property
testing, fuzzing and mutation testing and showed how they can be used to gain
confidence that code behaves correctly in the face of untrusted inputs. We have
explored how to handle external services when writing well-tested code. We have
also covered how to measure how much of the code is covered during testing.

In my opinion, testing is very valuable for writing robust code. Especially
approaches that take little effort but produce great test coverage (such as
randomized testing) can help make sure that code really does behave well.

## Matrix




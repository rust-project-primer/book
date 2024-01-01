# Tiers

Ideally, what we would like to achieve is having different tiers of tests so that local
development can happen with little friction, but the project is still robus.

| Tier | Description |
| --- | --- |
| Fast | Tests that are run locally. Only runs tests for the crate that is currently being worked on. |
| Medium | Runs all tests with all features enabled. |
| Slow | Runs all tests from Tier 2, plus long-running tests such as fuzzing, extended property testing, async permutation tests. |


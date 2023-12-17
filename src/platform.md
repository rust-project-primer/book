# Platform

Code is not written in isolation, instead it is written collaboratively with
others; often in public. Code platforms exist to allow this kind of collaboration
to happen, by offering version control systems, interfaces for proposing code changes
and doing code reviews. They also typically offer projects management capabilities
such as milestone and issue tracking.

What is perhaps more important to building robust code is that most programming
platforms today also include some kind of *Continuous Integration* system. Simply
put, this is a system which allows one to run various kinds of automated checks
on code before it is allowed to be approved.

One common use-case is to run unit tests in the CI system for every proposed code
change, and only allow the code change to be applied if all of the unit tests pass.
This rule ensures that no matter what, the code always has working unit tests.
However, there are many more kinds of checks that can and should be performed for
code changes, some of which this guide will explain and recommend.

Due to the large number of CI systems which are in use today, it would be impossible
to show working examples for all of them. Instead, this guide recommends and will
focus on two rather popular platforms: [GitHub][github] or [GitLab][gitlab], which
both use the `git` version control system and have similar feature sets.

| Platform | Description |
| --- | --- |
| <img src="gitlab.png" width="200px" /> | **GitLab**, with GitLab CI |
| <img src="github.png" width="200px" /> | **GitHub**, with GitHub Actions |

If you happen to use a different platform or CI system, you may have to adapt
the examples in this guide to the system that you are using.

[github]: https://github.com
[gitlab]: https://gitlab.com


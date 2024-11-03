# Continuous Integration

Modern software development tries to be very automated. The days where
developers push code to a server using FTP are gone, modern practises use
automated testing (often called *Continuous Integration*) and automated
deployment of code (often called *Continuous Deployment*).

<center>

![CI/CD Loop](ci-cd-loop.svg)

</center>

The idea behind these systems is twofold:

- Having automated tests (CI) and enforcing them to succeed dramatically reduces
  incidents in production. Code projects should not rely on correctness because
  of knowledge hidden inside senior developer brains, but rather their proprties
  should be encoded, measured and tested automatically.
- Having frequent and automated deployments (CD) allows teams to react faster,
  making them automated forces teams to write good tests to prevent production
  incidents.

There are other resources that go much further into depth of why these systems
are useful. This book doesn't focus too much on the deployment aspects of Rust
projects. But this book does focus on the various bits of tooling that Rust has
which you can use to build useful CI pipelines that ensure that your Rust
project stays in good shape over time.

The CI/CD systems that we have today are all built around a simple idea: the
ability to run code in reaction to various events. For example, when a
developer creates a merge request, you might have some code that runs unit
tests, determines test coverage, and runs other checks against the codebase.
Doing this means that the developer can get quick feedback if he or she has
made an error, and can rectify it easy. When a code change request is accepted,
another job might run which triggers the deployment. Some CI/CD systems also
have the ability to run jobs on a schedule, for example to run mode extensive
tests on a daily basis, rather than for every single change request.

- Diagram

All of these functionalities are enabled by having a good CI/CD solution.
Generally, your code hosting solution should have some functionality built-in,
for example GitLab and GitHub both have good CI/CD situations. It is also
possible to use or deploy an external CI/CD solution, there is a whole [list of
options](https://ligurio.github.io/awesome-ci/). But generally, unless you have
specific requirements, just use whatever your development platform uses or
whatever is the easiest to operate.

## Reading

[Continuous
Integration](https://martinfowler.com/articles/continuousIntegration.html) by
Martin Fowler

*In this article, Martin summarizes continuous integration practises. In his
own words: "Continuous Integration is a software development practice where each
member of a team merges their changes into a codebase together with their
colleagues changes at least daily. Each of these integrations is verified by an
automated build (including test) to detect integration errors as quickly as
possible. Teams find that this approach reduces the risk of delivery delays,
reduces the effort of integration, and enables practices that foster a healthy
codebase for rapid enhancement with new features."*

[Continuous Integration](https://abseil.io/resources/swe-book/html/ch23.html)
in Software Engineering at Google

*TODO*

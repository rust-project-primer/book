# GitLab CI

GitLab is an [open-source][source] software development platform. It is similar
to GitHub, but offers some more advanced features.

[source]: https://gitlab.com/gitlab-org/gitlab

GitLab CI is configured using a `.gitlab-ci.yml` file in the repository root.
It is built around Docker containers, every job runs in a Docker container and
executes some commands that are configurable. Background services (such as
databases) can also be launched in the background by providing Docker images.
The GitLab CI runner is configurable and can also uses other, non-Docker
backends, such as running jobs in virtual machines using QEMU (this is useful
for running tests on platforms such as FreeBSD or Windows).

## Example

```yaml
test:cargo:
  image: "rust:latest"
  script:
    - rustc --version && cargo --version
    - cargo test --workspace --verbose
```

## Reading

[Get started with GitLab CI/CD](https://docs.gitlab.com/ee/ci/)

*Shows you how to get started with GitLab CI.*

[Deploying Rust with Docker and Kubernetes](https://www.fpcomplete.com/blog/deploying-rust-with-docker-and-kubernetes/)

*In this article, FP complete shows you how to deploy a Rust application with
Docker and Kubernetes using GitLab CI.*

[(New) Adventures in CI](https://www.bassi.io/articles/2019/04/13/adventures-in-ci/)

*In this blog post, Emmanuele Bassi shows you how the GNOME project uses GitLab
CI to generate coverage reports for every commit.*

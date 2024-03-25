# GitHub Actions

GitHub was the first user-interface for the Git version-control system. It was
launched in 2008, and famously acquired by Microsoft in 2018. GitHub has GitHub
Actions, which allows you to build Continuous Integration and Continuous
Deployment pipelines using TypeScript.

## Example

GitHub actions work with YAML configuration files placed in the
`.github/workflows` folder in your Git repository.

```yaml
name: Hello World
on: [push]
jobs:
  hello-world:
    runs-on: ubuntu-latest
    steps:
      - run: echo "Hello world"
```

You can define multiple jobs, define when they should run (on pushing to a
branch, on merging a branch), define dependencies (jobs configured in other
repositories), what machine the jobs should run on (usually Ubuntu Linux), and
what the jobs should do (usually some bash commands).

## Reading

[GitHub Actions QuickStart](https://docs.github.com/en/actions/quickstart)

*Shows you how to get started with GitHub Actions.*

[GitHub Actions Feels Bad](https://www.youtube.com/watch?v=9qljpi5jiMQ) by Amos Wenger

*The history and design of GitHub actions, and why they are perhaps not
designed in an ideal way.*

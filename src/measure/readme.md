# Measure

Everybody talks about being _data-driven_, but few software projects actually are. There might be a
set of properties of your software that you care about. For example, some of these properties might
be correctness (measured by the ability of the test suite to test all edge cases your software may
run into), performance (measured by the execution time of a set of operations that is representative
for what your software might do in the real world). Any properties that are critical to the project
should be continuously measured, and these (aggregated) measurements made available to engineers to
help them shape the direction and implementation of the project.

<center>

![Measurement feedback loop](measurment-loop.svg)

</center>

If you have a Rust software project, you should ask yourself the question: are there any important
properties that this software must uphold? Based on your responses to this question, you should
think about how you can measure these properties, and ensure that they are continuously monitored.
Some of the properties might be implicit and difficult to identify. For example, if you are running
a web application, you want users to have a good experience. Part of that could be that your
application should be _snappy_, but it is difficult to quantify that. If there are less users on
your side, is it because it is slower? Or is the design worse than it was? One part of being
data-driven is identifying what data is critical.

Software constantly changes, and just because you have come up with a data structure that performed
well on the workload today, does not mean that it will still be the best data structure for the job
tomorrow. Coming up with metrics and continuously monitoring them allows you to notice regressions
_before_ they hit production.

Here are some examples for properties that you might want to measure over time, and why they might
be critical to a project. Every project is different, and not all proprties might be of equal
importance. Setting up and maintaining measurement pipelines takes time, so you should choose the
properties you optimize for wisely.

|    Properties    | Use-case                                                                                                                                                                                                     |
| :--------------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Binary size**  | You are deploying a WebAssembly-powered website, which needs to be fetched by the browser on every first request. You want to ensure that the website loads quickly, so you want to measure the binary size. |
| **Memory usage** | You are writing a firmware for a microcontroller which has limited amount of memory. You want to measure the dynamic memory usage to ensure that it stays within the allowed limit.                          |
| **Correctness**  | Your project includes a bespoke lock-free data structure to handle data for many concurrent requests, and you want to make sure that it is correct for all possible use-cases.                               |
| **Performance**  | Your application includes some custom data processing code that is mission-critical. You want to measure the performance of them over time to ensure that there are no regressions as it is being developed. |

But measuring them is only one half of the equation. The other half is: how do you collect,
aggregate this information and make it available to your engineers to shape the decision process?
There are some tools that can help with this, for example:

| Tool                            | Purpose                                                                                                                                                                        |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Bencher](https://bencher.dev/) | Aggregates benchmark results, allowing you to see how performance changes over time.                                                                                           |
| [GitLab](https://gitlab.com/)   | GitLab has the ability to visualize code coverage and test results measured in CI jobs in merge requests, allowing developers to assess how well new code is covered by tests. |

This chapter focusses on showing you how you can measure properties of your codebase continuously,
and what options you have for aggregating this information and use it in decision-making processes.
Naturally, this chapter can't cover every single metric you might care about, but it can give you an
appreciation for how you can approach this.

## Reading

```reading
style: article
title: Performance Culture
url: https://joeduffyblog.com/2016/04/10/performance-culture/
author: Joe Duffy
archived: joeduffy-performance-culture.pdf
---
Joe argues that performant software is not an accident, but rather a product of
a performance culture. He explains what this culture looks like: that the
properties that the project wants to uphold (eg performance) have management
buy-in and are not afterthoughts, they are constantly measured so that
engineers can make data-driven decisions when implementing new features in code
reviews.
```

```reading
style: book
title: "Systems Performance: Enterprise and the Cloud, 2nd Edition"
url: https://brendangregg.com/systems-performance-2nd-edition-book.html
author: Brendan Gregg
---
In this book, Brendan goes into depths of how to analyze the performance of
systems, specifically in the context of cloud-deployed software. Linux has
powerful capabilities of hooking into application execution at runtime,
instrumenting it with eBPF code to measure not only how the application is
performing, but also giving the ability to understand why it is performing the
way it is. This book is a must-read for anyone who deeply cares about
performance, wants to measure and debug it.
```

```reading
style: article
title: Be good-argument-driven, not data-driven
url: https://twitchard.github.io/posts/2022-08-26-metrics-schmetrics.html
author: Richard Marmorstein
---
Richard talks about using data to influence development of software. He
explains that while data is useful, at the end it should be used to back up
arguments, not an end in itself. There are cases where data could be
interpreted wrong, and you should be sceptical of poor arguments made by
incorrectly-interpreted data.
```

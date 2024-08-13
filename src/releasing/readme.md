# Releasing

Releasing is the process of publishing new versions of software. Typically,
there is some process around it, which includes throrough testing to ensure that
it is bug-free, and the publishing of artefacts (binaries).

The release process for every project is different. Some projects combine
releases with deployments of the updated software, which is called *Continuous Deployment*.
Some projects create releases when there are enough new features to warrant a new
version, others use a fixed calendar-based release cycle where a new version
is released at fixed schedules.

Some of the challenges when it comes to releases are:

- How do you communicate breaking changes? Often times, breaking
  changes are encoded in the release version using *Semantic versioning*,
  and a changelog is published that documents all changes for downstream users.
- How do you make the software usable? This includes publishing binaries
  (for applications), publishing packages (Debian packages, Flatpak files, RPM packages),
  publishing it to Crate registries (for libraries), publishing Docker containers.
- How do you automate the release process so that it runs smoothly with little
  human intervention?

This section addresses some of these questions.

# A Weekend Playing with Rust

This project forms part of some [documentation](https://one.pythian.com/pages/viewpage.action?pageId=240747156) around using Docker Desktop and VS Code's Remote Containers ("Dev Containers") feature. In particular, this is one of the demonstrations for that page

Please don't expect this repository to form part of any completed work.

If you're interested, it forms the start of a tool which will query the 'release' artefacts on GitHub and list/download the artefacts that match a given version constraint (think version constraints like in Python, Ruby, Rust)... an experimentation in using Rust to improve security outcomes in managing downloads from GitHub releases.

## Opportunities

It's very early work... but if you're looking to get your hands dirty, you might:

* Experiment by adding a configuration file that can list different releases, version constraints and asset name regexps. You might consider YAML for this... or TOML, both being well supported in Rust.
* There will be various tidy-ups needed against the version tag to make it usable against the version comparison library (Rust's Cargo)... such as removing a leading 'v' from v1.2.3 to get 1.2.3; there are likely to be other such things and its probably easiest to have a set of regex replacements that could be specified in a config file.
* Tidy up the output so it just writes the output, maybe in different formats (eg. JSON, text lines) so some other tooling can then download the files.
* Experiment adding to that tooling with some DevSecOps... such as by seeing what Anchore's Syft and Grype tools could do with both our code and the releases we download.
* Implement a 'greylisting' feature, where we only consider an asset if its been released for a period of time (eg. a week) as a guard against 'sabotage' or other supply-chain corruption.
* Do something with the release notes... maybe detect if a CVE is mentioned?
* Similar to Cargo.toml and Cargo.lock (or other similar 'lock' files which specify the actual version currently used), you might consider adding similar functionality.
KAS WGPU
======

Toolkit rendering over the [wgpu](https://crates.io/crates/wgpu) lib.


Optional features
-------

This crate has the following feature flags:

-   `clipboard` (enabled by default): clipboard integration
-   `stack_dst` (enabled by default): enables `kas-theme::MultiTheme`
-   `gat`: enables usage of the Generic Associated Types feature (nightly only
    and currently unstable), allowing some usages of `unsafe` to be avoided.
    (The plan is to enable this by default once the feature is mature.)
-   `unsize`: forwards this feature flag to `kas-theme`

Copyright and Licence
-------

The [COPYRIGHT](COPYRIGHT) file includes a list of contributors who claim
copyright on this project. This list may be incomplete; new contributors may
optionally add themselves to this list.

The KAS library is published under the terms of the Apache License, Version 2.0.
You may obtain a copy of this licence from the [LICENSE](LICENSE) file or on
the following webpage: <https://www.apache.org/licenses/LICENSE-2.0>

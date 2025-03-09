# Release Notes for v0.1.4

- we adjusted the dependency list of only activate the **native-tls-vendored**
  feature from reqwest for the musl based target.

motivation is resolve the building issues (on github actions) where openssl-sys
was not able to work with the openssl installation from Ubuntu.

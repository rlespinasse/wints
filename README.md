# WINTS

[![Continuous Integration Status][1]][3]
[![Security Audit Status][2]][3]
[![Licence][4]][5]

You may have a lot of URLs for a lot of context.
If the question is **What I Need To See in this context?**, then the enswer is `wints`.

## Installation

Build Manually

```sh
git clone --depth 1 git@github.com:rlespinasse/wints.git ~/.wints
cd ~/.wints
cargo install --path .
cargo build --release
# put the resulting `target/release/wints` executable on your PATH.
```

## Usage

To run, `wints` need a `.wints.yaml` file like this default one (generated when missing)

```yaml
---
elements:
  - context: repository code
    urls:
      - https://github.com/rlespinasse/wints
  - context: repository issues
    urls:
      - https://github.com/rlespinasse/wints/issues
```

So when you run

- `wints repository` will open all urls
- `wints issues` will only open the urls on **repository issues** context

Fuzzy-matching is also possible,

- `wints repo code` will only open the urls on **repository code** context

[1]: https://github.com/rlespinasse/wints/workflows/Continuous%20integration/badge.svg
[2]: https://github.com/rlespinasse/wints/workflows/Security%20audit/badge.svg
[3]: https://github.com/rlespinasse/wints/actions
[4]: https://img.shields.io/github/license/rlespinasse/wints
[5]: https://github.com/rlespinasse/wints/blob/v0.x/LICENSE

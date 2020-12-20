# WINTS

You may have a lot of URLs for a lot of context.

If the question is **What I Need To See in this context?**, then the answer is `wints`.

## Installation

```sh
cargo install wints
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

- `wints repository` will open all URLs
- `wints issues` will only open the URLs on **repository issues** context

Fuzzy-matching is also possible,

- `wints repo code` will only open the URLs on **repository code** context

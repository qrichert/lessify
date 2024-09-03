# very nice table

[![license: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/license/mit)
![GitHub Tag](https://img.shields.io/github/v/tag/qrichert/lessify?sort=semver&filter=*.*.*&label=release)
[![crates.io](https://img.shields.io/crates/d/lessify?logo=rust&logoColor=white&color=orange)](https://crates.io/crates/lessify)

_Output text through a pager._

It uses `less` by default, or any pager set in the `PAGER` environment
variable.

## Example

```rust
use lessify::Pager;

fn main() {
    let text = very_long_text();

    Pager::page_or_print(text);
}
```

```
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut ex suscip
it, elementum tortor et, pretium odio. Suspendisse quis lacus vel nulla mollis m
alesuada. Sed orci purus, auctor ut tempor vitae, convallis ut lorem. Donec sem
augue, efficitur condimentum mauris non, sagittis porttitor lacus. Proin maximus
 suscipit pellentesque. Vestibulum ante eros, ultrices ac varius a, porttitor ne
c libero. Proin tempus dui vel leo rutrum eleifend.

Nullam est libero, posuere vitae tellus ut, volutpat aliquam erat. Fusce vitae u
rna nibh. Proin luctus, augue non aliquam elementum, purus magna consequat ligul
a, id lobortis magna leo nec nunc. Donec at turpis dapibus, malesuada massa vita
:
```

# Repository

The purpose of a README is for people to get a very brief introduction to what your project does.
For open-source projects it is essential, when people decide if your crate solves the issue they are
trying to solve. It does not need to be a comprehensive documentation document, rather a very dense
summary that contains some vital pieces of information of what your crate does, how it compares to
other crates that achieve similar goals, and what limitations it has.

There are some common patterns that make for useful README files, and this chapter will attempt to
illustrate them.

## Badges

Badges are little images that you can embed into your README that show up-to-date information on
your Rust project. These are useful because they do not need need to be updated manually.

Generally, you can put them in your README like this:

```markdown
# Project Name

[![crates.io version](https://img.shields.io/crates/v/imstr.svg)](https://crates.io/crates/imstr)
[![docs.rs documentation](https://img.shields.io/docsrs/imstr)](https://docs.rs/imstr)
```

### Common badges for Rust crates

These badges pull information on crates published on [crates.io][]. By definition, these will not
pull data from source control, but rather from whatever is published. They render information such
as the most recent version, status of automatically built documentation, download counts, and health
checks for dependencies.

Crate: <input id="crate-input" type="text" value="serde" /><br />

| <div style="width: 150px;">Badge</div>    | Markdown                                           |
| ----------------------------------------- | -------------------------------------------------- |
| <img id="badge-image-version" />          | <code id="badge-markdown-version"></code>          |
| <img id="badge-image-license" />          | <code id="badge-markdown-license"></code>          |
| <img id="badge-image-downloads-total" />  | <code id="badge-markdown-downloads-total"></code>  |
| <img id="badge-image-downloads-recent" /> | <code id="badge-markdown-downloads-recent"></code> |
| <img id="badge-image-downloads-latest" /> | <code id="badge-markdown-downloads-latest"></code> |
| <img id="badge-image-docs" />             | <code id="badge-markdown-docs"></code>             |
| <img id="badge-image-deps" />             | <code id="badge-markdown-deps"></code>             |

<script>
function badges_update() {
    function badge_hook(name, alt, image, link) {
        const crate = document.getElementById("crate-input").value;
        const image_url = image(crate);
        const link_url = link(crate);
        document.getElementById(`badge-image-${name}`).src = image_url;
        document.getElementById(`badge-markdown-${name}`).innerText = `[![${alt}](${image_url})](${link_url})`;
    }

    badge_hook("version", "crates.io version",
        (name) => `https://img.shields.io/crates/v/${name}.svg`,
        (name) => `https://crates.io/crates/${name}`);

    badge_hook("license", "crates.io license",
        (name) => `https://img.shields.io/crates/l/${name}.svg`,
        (name) => `https://crates.io/crates/${name}`);

    badge_hook("downloads-total", "crates.io downloads",
        (name) => `https://img.shields.io/crates/d/${name}.svg`,
        (name) => `https://crates.io/crates/${name}`);

    badge_hook("downloads-recent", "crates.io recent downloads",
        (name) => `https://img.shields.io/crates/dr/${name}.svg`,
        (name) => `https://crates.io/crates/${name}`);

    badge_hook("downloads-latest", "crates.io downloads",
        (name) => `https://img.shields.io/crates/dv/${name}.svg`,
        (name) => `https://crates.io/crates/${name}`);

    badge_hook("docs", "docs.rs documentation",
        (name) => `https://img.shields.io/docsrs/${name}.svg`,
        (name) => `https://docs.rs/${name}`);

    badge_hook("deps", "dependency status",
        (name) => `https://deps.rs/crate/${name}/latest/status.svg`,
        (name) => `https://deps.rs/crate/${name}`);
}

window.addEventListener("load", (event) => {
    document.getElementById("crate-input").addEventListener("input", (event) => {
        badges_update();
    });
    badges_update();
});
</script>

### Generating a readme file from crate-level documentation

The [Readme](../tools/repository.md) section shows some tools that you can use to generate a README
file from crate-level documentation.

[crates.io]: https://crates.io/

# Dependency Extractor

This is a static analysis tool which parses the imports from
javascript or typescript code, and then outputs them to `.dependencies/**/*.yaml`
files that other tools can process more easily later on.

The `.dependencies` folder has the following properties:

1. The `**/*.yaml` files mirror the original project's source code.
1. The `.yaml` files have imports relative to the project root. So if two files from different
directories import the same util, the `yaml` files will have the same path even though the relative
imports from the original files would be different.

For concrete examples, see [the test cases](./tests/cases). If run on `{case}/project`,
this tool creates a `{case}/project/.dependencies` with contents that match
`{case}/expectations`.

## Usage

```
# Run the CLI script against a project
cargo run -- -b ../path/to/your/project/root

# Run the test suite. Make sure the tool generates `cases/{case}/expectations`
# when run on `cases/{case}/project`.
cargo test
```

## Use cases

The primary value in this tool is its `.dependencies` folder and `.yaml` output.
Although no other known tooling exists (yet), I can foresee a few benefits to
this approach:

### Visualizing the project

Many ecosystems have graph visualizers. Some examples:

- ([pydoit](https://pydoit.org/tutorial-1.html)
- [import-graph-visualizer](https://github.com/rx-angular/import-graph-visualizer)
- [goda](https://github.com/loov/goda)

Unfortunately these tools are monolithic: they parse the code _and_ draw the graph.

Visualization software using `.yaml` files would work across input languages,
making it significantly more reusable.

### Validating dependencies

The [dependency-cruser](https://github.com/sverweij/dependency-cruiser) can also _validate_
dependencies. As a monolithic tool, this one is tied to the Node ecosystem.

A dependency validator using `.yaml` files could be used across all lanaguages.

### Unlocking new code complexity measurements

Common code complexity metrics (lines of code, cyclomatic complexity) examine code
in isolation. I personally find this unsatisfying: "simple" code which imports and
calls a "complex" function is still complex code.

By combining complexity metrics with the import graph, it *seems* to me like we can
improve our metrics overall.

Again, these `.yaml` files may contribute by working out import graph ahead of time. 

### Contributing

**Open an issue** if you:

1. Have suggestions to improve or future-proof the `.yaml` file format. I consider this
the most important thing to get "right."
1. Found a bug, but don't plan to fix it.
1. Have a feature request, even if you're willing to implement it.
1. Want to volunteer to add support for parsing a new language.

**Open a PR directly** if you:

1. Have found/fixed a bug.
1. Have documentaiton improvements.
1. Are refactoring something for clarity or performance.
1. Are adding or enhancing a test to make it more thorough.

Bugfix PRs must include a regression test.

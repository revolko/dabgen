# DABGEN

DAB file structure generator.

# Installation

_Requires `cargo` installed locally. [Installation instruction](https://doc.rust-lang.org/cargo/getting-started/installation.html)._

The tool is uploaded to `crates.io` so you can install `dabgen` as system binary
```bash
cargo install dabgen
```

# How to run

The tool expects `name` of the DAB and `path` to the root directory as positional arguments.
Additionally, the following options are available:
 * `-t, --target <TARGET>` -- Target name. Use the option multiple times to define more targets
 * `-p, --pipeline <PIPELINE>` -- Pipeline name. Use the option multiple times to define more pipelines
 * `-j, --job <JOB>` -- Job name. Use the option multiple times to define more jobs
 * `-a, --alert <ALERT>` -- Alert name. Use the option multiple times to define more alerts
 * `-h, --help` -- Print help

Thus, the following command:
```bash
dabgen dab_name /some/path -t tst -p load -j job
```

creates the following file structure:
```
/some/path/dab_name/dab_name/__init__.py
/some/path/dab_name/job/job.yaml
/some/path/dab_name/pipelines/load.yaml
/some/path/dab_name/targets/tst.yaml
/some/path/dab_name/databricks.yaml
```

# DABGEN

DAB file structure generator.

# How to run

_For now requires `cargo` installed locally. [Installation instruction](https://doc.rust-lang.org/cargo/getting-started/installation.html)._
_In the future a proper rust package (installable as binary) will be published._

The tool expects `name` of the DAB as positional argument. Additionally, the following options are available:
 * `-t, --target <TARGET>` -- Target name. Use the option multiple times to define more targets
 * `-p, --pipeline <PIPELINE>` -- Pipeline name. Use the option multiple times to define more pipelines
 * `-j, --job <JOB>` -- Job name. Use the option multiple times to define more jobs
 * `-a, --alert <ALERT>` -- Alert name. Use the option multiple times to define more alerts
 * `-h, --help` -- Print help

Thus, the following command:
```bash
cargo run -- dab_name -t tst -p load -j job
```

creates the following file structure:
```
./dab_name/dab_name/__init__.py
./dab_name/job/job.yaml
./dab_name/pipelines/load.yaml
./dab_name/targets/tst.yaml
./dab_name/databricks.yaml
```

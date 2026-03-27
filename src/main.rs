use std::fs::{self, File};

use clap::Parser;

enum FileType {
    Python,
    Yaml,
}

impl FileType {
    fn value(&self) -> &str {
        match self {
            FileType::Python => "py",
            FileType::Yaml => "yaml",
        }
    }
}

/// DAB file structure generator
#[derive(Parser)]
struct Cli {
    /// Name of the DAB
    name: String,
    /// Path to the root directory
    path: String,

    /// Target name. Use the option multiple times to define more targets
    #[arg(short, long)]
    target: Vec<String>,
    /// Pipeline name. Use the option multiple times to define more pipelines
    #[arg(short, long)]
    pipeline: Vec<String>,
    /// Job name. Use the option multiple times to define more jobs
    #[arg(short, long)]
    job: Vec<String>,
    /// Alert name. Use the option multiple times to define more alerts
    #[arg(short, long)]
    alert: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let dab_root = format!("{}/{}", args.path, args.name);
    fs::create_dir_all(&dab_root)?;

    // databricks resources
    if !args.target.is_empty() {
        create_sub_dir("targets", &dab_root, &args.target, FileType::Yaml)?
    }
    if !args.pipeline.is_empty() {
        create_sub_dir("pipelines", &dab_root, &args.pipeline, FileType::Yaml)?
    }
    if !args.job.is_empty() {
        create_sub_dir("jobs", &dab_root, &args.job, FileType::Yaml)?
    }
    if !args.alert.is_empty() {
        create_sub_dir("alerts", &dab_root, &args.alert, FileType::Yaml)?
    }

    // subdirectory for python code
    create_sub_dir(
        &args.name,
        &dab_root,
        &vec!["__init__".to_string()],
        FileType::Python,
    )?;

    // root databricks.yaml file
    File::create(format!("{}/databricks.yaml", &dab_root))?;

    Ok(())
}

fn create_sub_dir(
    sub_dir_name: &str,
    root_dir: &str,
    files: &Vec<String>,
    files_type: FileType,
) -> anyhow::Result<()> {
    let sub_dir = format!("{}/{}", &root_dir, &sub_dir_name);
    fs::create_dir_all(&sub_dir)?;

    for file in files {
        File::create(format!("{}/{}.{}", sub_dir, file, files_type.value()))?;
    }

    Ok(())
}

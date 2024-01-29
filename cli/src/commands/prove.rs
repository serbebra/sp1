use std::process::Command;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "prove", about = "(default) Build and prove a Rust program")]
pub struct ProveCmd {
    #[clap(long)]
    target: Option<String>,

    #[clap(last = true)]
    cargo_args: Vec<String>,
}

impl ProveCmd {
    pub fn run(&self) -> Result<()> {
        let mut metadata_cmd = cargo_metadata::MetadataCommand::new();
        let metadata = metadata_cmd.no_deps().exec().unwrap();
        let root_package = metadata.root_package();
        let root_package_name = root_package.as_ref().map(|p| &p.name);

        println!("metadata: {:?}", metadata);

        let build_target = "riscv32im-risc0-zkvm-elf";
        let rust_flags = [
            "-C",
            "passes=loweratomic",
            "-C",
            "link-arg=-Ttext=0x00200800",
            "-C",
            "panic=abort",
        ];
        let target_dir = metadata.target_directory.join(build_target);
        let target_path = target_dir.as_path();
        let target_dir_path = target_path.to_string();
        println!("Building ELF at {:?}", target_dir);
        let success = Command::new("cargo")
            // .env("RUSTUP_TOOLCHAIN", "risc0")
            .env("CARGO_ENCODED_RUSTFLAGS", rust_flags.join("\x1f"))
            .args([
                "build",
                "--release",
                "--target",
                build_target,
                "--target-dir",
                target_dir_path.as_str(),
            ])
            .status()?;

        let elf_path = metadata
            .target_directory
            .join(build_target)
            .join("release")
            .join(root_package_name.unwrap());

        if !success.success() {
            println!("Failed to build ELF at {:?}", elf_path);
            return Err(anyhow::anyhow!("Failed to build ELF"));
        }
        println!("Successfully built ELF at {:?}", elf_path);

        let target_dir = metadata.target_directory.join("target");
        let target_path = target_dir.as_path();
        let target_dir_path = target_path.to_string();
        let result = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--target-dir",
                target_dir_path.as_str(),
                "--",
                "--input",
                "input.json",
            ])
            .output()?;

        let output = result.stdout;

        // Parse last line of stdout as hex to get bytes
        let output = String::from_utf8(output)?;
        let hex = output.trim().lines().last().unwrap();

        // Parse hex to get bytes
        println!("Hex: {}", hex);
        let bytes = hex::decode(hex)?;
        println!("Bytes: {:?}", bytes);

        Ok(())
    }
}

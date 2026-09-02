from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path


DOCS_ROOT = Path(__file__).resolve().parent
CAPSTONE_ROOT = DOCS_ROOT.parent
OUTPUT_ROOT = DOCS_ROOT / "_build" / "html"
CHAPTER_DIRECTORIES = [
    "00-app-runtime",
    "01-plugins-resources-systems",
    "02-debug-visualization",
    "03-ecs-data-model",
    "04-query-access-conflicts",
]


def copy_cargo_project(source: Path, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    for filename in ("Cargo.toml", "Cargo.lock", "README.md", "Rust_in_Bevy.md"):
        source_file = source / filename
        if source_file.is_file():
            shutil.copy2(source_file, destination / filename)
    shutil.copytree(source / "src", destination / "src", dirs_exist_ok=True)


def main() -> None:
    subprocess.run(
        [
            sys.executable,
            "-m",
            "sphinx",
            "--fail-on-warning",
            "--builder",
            "html",
            str(DOCS_ROOT),
            str(OUTPUT_ROOT),
        ],
        check=True,
    )

    for directory in CHAPTER_DIRECTORIES:
        copy_cargo_project(DOCS_ROOT / directory, OUTPUT_ROOT / directory)

    stages_source = DOCS_ROOT / "stages"
    for stage_source in stages_source.iterdir():
        if stage_source.is_dir():
            copy_cargo_project(stage_source, OUTPUT_ROOT /
                               "stages" / stage_source.name)

    copy_cargo_project(CAPSTONE_ROOT, OUTPUT_ROOT / "capstone")


if __name__ == "__main__":
    main()

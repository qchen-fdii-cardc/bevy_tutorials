from __future__ import annotations

import shutil
import subprocess
import sys
import argparse
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
    "05-commands-lifecycle",
    "06-events-change-detection",
    "07-fixed-update",
    "08-input-mapping",
    "09-state-run-conditions",
    "10-2d-coordinates",
    "11-camera-follow",
    "12-sprites-layers-z",
    "13-ui-world-space",
    "14-collision-minimum-model",
    "15-event-driven-damage",
    "16-rule-chain-composition",
    "17-asset-server",
    "18-animation-state",
    "19-audio-ui-feedback",
    "20-localization-config",
    "21-plugin-project-structure",
    "22-testing-ecs-logic",
    "23-performance-profiling",
    "24-save-settings-release",
]


def copy_cargo_project(source: Path, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    for filename in ("Cargo.toml", "Cargo.lock", "README.md", "Rust_in_Bevy.md"):
        source_file = source / filename
        if source_file.is_file():
            shutil.copy2(source_file, destination / filename)
    shutil.copytree(source / "src", destination / "src", dirs_exist_ok=True)


def build_html() -> None:
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


def build_pdf() -> None:
    subprocess.run(
        [
            sys.executable,
            "-m",
            "sphinx.cmd.build",
            "--fail-on-warning",
            "-b",
            "latex",
            str(DOCS_ROOT),
            str(DOCS_ROOT / "_build" / "latex"),
        ],
        check=True,
    )
    latex_directory = DOCS_ROOT / "_build" / "latex"
    subprocess.run(
        [
            "latexmk",
            "-xelatex",
            "-interaction=nonstopmode",
            "-halt-on-error",
            "bevy.tex",
        ],
        cwd=latex_directory,
        check=True,
    )


def main() -> None:
    parser = argparse.ArgumentParser(
        description="构建 Bevy 学习路线文档站，或生成中文 PDF。"
    )
    parser.add_argument(
        "--pdf",
        action="store_true",
        help="使用 XeLaTeX 生成单个中文 PDF；默认只构建 HTML。",
    )
    args = parser.parse_args()

    build_html()
    if args.pdf:
        build_pdf()


if __name__ == "__main__":
    main()

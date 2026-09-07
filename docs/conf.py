from datetime import date

project = "Bevy 学习路线"
author = "qchen-fdii-cardc"
copyright = f"{date.today().year}, {author}"
release = "Bevy 0.19"

extensions = ["myst_parser"]
source_suffix = {".rst": "restructuredtext", ".md": "markdown"}
myst_enable_extensions = ["colon_fence", "fieldlist", "tasklist"]
suppress_warnings = ["myst.xref_missing"]

templates_path = ["_templates"]
exclude_patterns = [
	"_build",
	".venv",
	"contributing.md",
	"**/Rust_in_Bevy.md",
	"Thumbs.db",
	".DS_Store",
]
html_theme = "furo"
html_title = "Bevy 学习路线"
html_static_path = ["_static"]
html_css_files = ["course-links.css"]
html_theme_options = {
	"source_repository": "https://github.com/qchen-fdii-cardc/bevy_tutorials/",
	"source_branch": "main",
	"source_directory": "docs/",
	"footer_icons": [
		{
			"name": "GitHub",
			"url": "https://github.com/qchen-fdii-cardc/bevy_tutorials",
			"html": "GitHub 仓库",
		}
	],
}

latex_engine = "xelatex"
latex_elements = {
    "preamble": r"""
\usepackage{fontspec}
\usepackage{xeCJK}
\setmainfont{TeX Gyre Termes}
\setCJKmainfont{Microsoft YaHei}
\setlength{\headheight}{14pt}
\raggedbottom
\vbadness=10000
\vfuzz=3pt
""",
}

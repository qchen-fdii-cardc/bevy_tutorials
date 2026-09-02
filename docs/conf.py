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
exclude_patterns = ["_build", ".venv", "Thumbs.db", ".DS_Store"]
html_theme = "furo"
html_title = "Bevy 学习路线"

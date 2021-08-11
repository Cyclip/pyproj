// Constants for what some files will contain
pub const FILE_GITIGNORE: &str = "# GitHub's python .gitignore template\n\n# Byte-compiled / optimized / DLL files\n__pycache__/\n*.py[cod]\n*$py.class\n\n# C extensions\n*.so\n\n# Distribution / packaging\n.Python\nbuild/\ndevelop-eggs/\ndist/\ndownloads/\neggs/\n.eggs/\nlib/\nlib64/\nparts/\nsdist/\nvar/\nwheels/\nshare/python-wheels/\n*.egg-info/\n.installed.cfg\n*.egg\nMANIFEST\n\n# PyInstaller\n#  Usually these files are written by a python script from a template\n#  before PyInstaller builds the exe, so as to inject date/other infos into it.\n*.manifest\n*.spec\n\n# Installer logs\npip-log.txt\npip-delete-this-directory.txt\n\n# Unit test / coverage reports\nhtmlcov/\n.tox/\n.nox/\n.coverage\n.coverage.*\n.cache\nnosetests.xml\ncoverage.xml\n*.cover\n*.py,cover\n.hypothesis/\n.pytest_cache/\ncover/\n\n# Translations\n*.mo\n*.pot\n\n# Django stuff:\n*.log\nlocal_settings.py\ndb.sqlite3\ndb.sqlite3-journal\n\n# Flask stuff:\ninstance/\n.webassets-cache\n\n# Scrapy stuff:\n.scrapy\n\n# Sphinx documentation\ndocs/_build/\n\n# PyBuilder\n.pybuilder/\ntarget/\n\n# Jupyter Notebook\n.ipynb_checkpoints\n\n# IPython\nprofile_default/\nipython_config.py\n\n# pyenv\n#   For a library or package, you might want to ignore these files since the code is\n#   intended to run in multiple environments; otherwise, check them in:\n# .python-version\n\n# pipenv\n#   According to pypa/pipenv#598, it is recommended to include Pipfile.lock in version control.\n#   However, in case of collaboration, if having platform-specific dependencies or dependencies\n#   having no cross-platform support, pipenv may install dependencies that don't work, or not\n#   install all needed dependencies.\n#Pipfile.lock\n\n# PEP 582; used by e.g. github.com/David-OConnor/pyflow\n__pypackages__/\n\n# Celery stuff\ncelerybeat-schedule\ncelerybeat.pid\n\n# SageMath parsed files\n*.sage.py\n\n# Environments\n.env\n.venv\nenv/\nvenv/\nENV/\nenv.bak/\nvenv.bak/\n\n# Spyder project settings\n.spyderproject\n.spyproject\n\n# Rope project settings\n.ropeproject\n\n# mkdocs documentation\n/site\n\n# mypy\n.mypy_cache/\n.dmypy.json\ndmypy.json\n\n# Pyre type checker\n.pyre/\n\n# pytype static type analyzer\n.pytype/\n\n# Cython debug symbols\ncython_debug/";
pub const FILE_MANIFEST: &str = "include setup.py\ninclude MANIFEST.in\ninclude LICENSE\ninclude README.md\n\ngraft tests\ngraft examples\ngraft docs\ngraft src";
pub const FILE_SETUP_PY: &str = "from setuptools import setup\n\nsetup()";
pub const FILE_TEST: &str = "# Example test\n\nimport src._test.main as main\n\nmain.main()";
pub const FILE_MAIN_PY: &str = r#"#!/usr/bin/python3

import sys

# ---------- CONSTANTS ----------

# -------------------------------


def main():
	"""Main function."""
	print("Hello world!")
	
	# Success
	return 0


if __name__ == "__main__":
	sys.exit(main())"#;
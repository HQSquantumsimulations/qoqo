#!/usr/bin/env python3
# -*- coding: utf-8 -*-
#
"""Configuration of sphinx documentation module"""
import tomli

main_version = tomli.load(open("../pyproject.toml", "rb"))["project"]["version"]

# -- General configuration ------------------------------------------------

# If your documentation needs a minimal Sphinx version, state it here.
#
# needs_sphinx = '1.0'

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.doctest",
    "sphinx.ext.todo",
    "sphinx.ext.coverage",
    "sphinx.ext.mathjax",
    "sphinx.ext.viewcode",
    "sphinx.ext.napoleon",
    "sphinx.ext.autosummary",
    "nbsphinx",
    "myst_parser",
]
# automatically use sphinx-autogen
autosummary_generate = True
autosummary_imported_members = True
# define mock imports for packages that are difficult to handle / install
alist = []
autodoc_mock_imports = alist

# 'both': class and __init__ docstring are concatenated and inserted
# 'class': only class docstring inserted
# 'init': only init docstring inserted
autoclass_content = "class"
# This value is a list of autodoc directive flags that should be automatically applied to
# all autodoc directives. The supported flags are 'members', 'undoc-members',
# 'private-members', 'special-members', 'inherited-members', 'show-inheritance',
# 'ignore-module-all' and 'exclude-members'.
# autodoc_default_flags = ['members', 'exclude-members']
# The default options for autodoc directives. They are applied to all autodoc directives
# automatically. It must be a dictionary which maps option names to the values.
autodoc_default_options = {
    "members": True,
    "special-members": False,
    "imported-members": False,
    "private-members": False,
    "inherited-members": False,
    #    'member-order': 'bysource',
    "special-members": False,
    "undoc-members": False,
    "exclude-members": "__init__",
}
# This value controls the docstrings inheritance. If set to True the docstring for classes
# or methods, if not explicitly set, is inherited form parents.
autodoc_inherit_docstrings = False
# Add any paths that contain templates here, relative to this directory.
templates_path = ["_templates"]

source_suffix = {
    ".rst": "restructuredtext",
    ".txt": "markdown",
    ".md": "markdown",
}

# The master toctree document.
master_doc = "index"

# General information about the project.
project = "qoqo"
copyright = "2021, HQS Quantum Simulations GmbH"
author = "HQS Quantum Simulations GmbH"

# The version info for the project you're documenting, acts as replacement for
# |version| and |release|, also used in various other places throughout the
# built documents.
#
# The short X.Y version.
version = main_version
# The full version, including alpha/beta/rc tags.
release = version


language = "English"

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This patterns also effect to html_static_path and html_extra_path
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# The name of the Pygments (syntax highlighting) style to use.
pygments_style = "default"

# If true, `todo` and `todoList` produce output, else they produce nothing.
todo_include_todos = True


# -- Options for HTML output ----------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#

html_theme = "sphinx_rtd_theme"
# Theme options are theme-specific and customize the look and feel of a theme
# further.  For a list of options available for each theme, see the
# documentation.
#
# html_theme_options = {}

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
# html_static_path = ['_static']
html_static_path = []


# -- Options for HTMLHelp output ------------------------------------------

# Output file base name for HTML help builder.
htmlhelp_basename = "qoqodoc"


# -- Options for LaTeX output ---------------------------------------------

latex_elements = {
    # The paper size ('letterpaper' or 'a4paper').
    #
    # 'papersize': 'letterpaper',
    # The font size ('10pt', '11pt' or '12pt').
    #
    # 'pointsize': '10pt',
    # Additional stuff for the LaTeX preamble.
    #
    # 'preamble': '',
    # Latex figure (float) alignment
    #
    # 'figure_align': 'htbp',
}

# Grouping the document tree into LaTeX files. List of tuples
# (source start file, target name, title,
#  author, documentclass [howto, manual, or own class]).
latex_documents = [
    (
        master_doc,
        "qoqo.tex",
        "qoqo Documentation",
        "HQS Quantum Simulations GmbH",
        "manual",
    ),
]


# -- Options for manual page output ---------------------------------------

# One entry per manual page. List of tuples
# (source start file, name, description, authors, manual section).
man_pages = [(master_doc, "qoqo", "qoqo Documentation", [author], 1)]


# -- Options for Texinfo output -------------------------------------------

# Grouping the document tree into Texinfo files. List of tuples
# (source start file, target name, title, author,
#  dir menu entry, description, category)
texinfo_documents = [
    (
        master_doc,
        "qoqo",
        "qoqo Documentation",
        author,
        "qoqo",
        "One line description of project.",
        "Miscellaneous",
    ),
]

# Turning off executing notebooks when adding them to Documentation
nbsphinx_execute = "never"

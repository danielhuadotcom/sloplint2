import os  # noqa

value = os.getpid()  # noqa: SLP030
other = 1  # noqa: E501
blanket = 2  # noqa  (kept for the legacy importer)
several = 3  # noqa: SLP020, SLP082


def plain():
    return "# noqa: SLP030 inside a string is not a directive"


def prose():
    return 1  # noqaX is prose, not a directive

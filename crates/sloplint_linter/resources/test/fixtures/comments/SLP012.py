import os  # noqa

value = os.getpid()  # noqa: SLP030
other = 1  # noqa: E501
blanket = 2  # noqa  (kept for the legacy importer)
several = 3  # noqa: SLP020, SLP082


def plain():
    return "# noqa: SLP030 inside a string is not a directive"


def prose():
    return 1  # noqaX is prose, not a directive


upper = 4  # NOQA
chained = 5  # type: ignore # noqa: E501
tight = 6  #noqa:E501
# ruff: noqa
# ruff: noqa: E501
#ruff:noqa:F401
# flake8: noqa
# ruff: disable[E501]
long_line = 7
# ruff: enable[E501]
ignored = 8  # ruff: ignore[F841]
# ruff: file-ignore[E501]
import sys  # isort: skip
# isort: skip_file
# isort: off
# isort: on
# ruff: isort: off
# ruff: isort: on
# isort: split
# ruff: disabled until the parser lands
# ruffles: noqa
# see ruff: noqa in the docs

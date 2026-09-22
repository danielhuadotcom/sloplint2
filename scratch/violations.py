"""Package entry point for the reporting pipeline."""

import os
import sys  # noqa
import pytest
import unittest.mock
from unittest import TestCase
from nose2 import discover

if sys.platform == "win32":
    import winreg
else:
    import termios

try:
    import ujson
except ImportError:
    import json as ujson
finally:
    import gc

with open(os.devnull) as handle:
    import csv

for _ in range(1):
    from json import loads


class Report:
    """A rendered report."""

    import base64

    def render(self):
        """Render the report to a string."""
        import textwrap

        return textwrap.dedent("done")

    def load(self, name):
        return __import__(name)


class TestReport(TestCase):
    """Tests for Report."""

    def test_renders(self):
        """It renders."""
        assert Report().render() == "done"

    def test_loads(self):  # noqa: SLP070
        import importlib

        assert importlib.import_module("json") is not None


class TestParser:
    def test_parses(self):
        assert True


def test_module_level():
    """A module-level test function."""
    import decimal

    return decimal


def test():
    return True


def helper(name):
    """Look the module up by name."""
    return importlib.import_module(name)  # noqa: SLP181


def another_helper():
    mod = __import__("csv")
    if mod:
        import statistics

        return statistics
    return None

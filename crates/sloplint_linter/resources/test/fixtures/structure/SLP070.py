import os
import pytest
import unittest.mock
from unittest import TestCase
from collections import OrderedDict


def test_parses_empty_input():
    assert parse("") == []


async def test_async_path():
    assert await fetch() is None


def test():
    assert True


def testable_values(rows):
    return [row for row in rows if row]


class TestParser:
    def test_method(self):
        assert True


class TestCaseRunner(unittest.TestCase):
    pass


class Tested:
    pass


class Parser:
    pass


def parse(text):
    return []

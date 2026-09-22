import os
import sys
from collections import OrderedDict

VALUE = os.getpid()

if sys.platform == "win32":
    import winreg
else:
    import termios

try:
    import ujson
except ImportError:
    ujson = None
finally:
    import gc

with open("x") as handle:
    import csv

for _ in range(1):
    from json import loads


def load(name):
    return importlib.import_module(name)


def load_builtin():
    return __import__("json")


def plain():
    import json

    return json


class Loader:
    def load(self):
        return importlib.import_module("csv")


def not_an_import():
    return loads("{}")


class Registry:
    import json


def deep():
    if True:
        import base64

        return base64
    return None

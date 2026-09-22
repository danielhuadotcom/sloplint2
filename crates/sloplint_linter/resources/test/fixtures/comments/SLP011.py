"""Module docstring — banned."""


def documented(text):
    """Parse the text and return the result."""
    return _parse(text)


def undocumented(text):
    return _parse(text)


class Documented:
    """A class docstring — banned."""

    def method(self):
        """A method docstring — banned."""
        return 1

    def quiet(self):
        return 2


class Quiet:
    value = 1


def not_a_docstring():
    value = "a string that is not the first statement"
    return value


def _parse(text):
    return text

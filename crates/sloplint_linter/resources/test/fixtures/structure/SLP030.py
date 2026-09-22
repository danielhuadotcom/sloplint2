def reraise(path):
    try:
        return open(path).read()
    except Exception as exc:
        raise exc


def swallow(path):
    try:
        return open(path).read()
    except Exception:
        pass


def log_only(path):
    try:
        return open(path).read()
    except Exception:
        logging.error("could not read")


def specific(path):
    try:
        return open(path).read()
    except FileNotFoundError:
        return ""


def log_and_reraise(path):
    try:
        return open(path).read()
    except Exception:
        logging.exception("could not read")
        raise


def translate(path):
    try:
        return open(path).read()
    except Exception as exc:
        raise RuntimeError("could not read") from exc


def narrow_but_trivial(path):
    try:
        return open(path).read()
    except FileNotFoundError:
        pass


def narrow_but_logs(path):
    try:
        return open(path).read()
    except FileNotFoundError:
        logging.error("missing")


def bare_with_real_body(path):
    try:
        return open(path).read()
    except:
        return ""


def narrow_and_substantial(path):
    try:
        return open(path).read()
    except FileNotFoundError:
        logging.exception("missing")
        return ""

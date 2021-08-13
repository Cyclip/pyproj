// Common file modules
// The module filename and the pip install packag name
// may be different, so refer to this to ensrue that
// most modules are detected properly.
pub const COMMON_MODS: [[&str; 2]; 6] = [
    ["flask", "Flask"],
    ["bs4", "beautifulsoup4"],
    ["flask_cors", "Flask-Cors"],
    ["flask_discord", "Flask-Discord"],
    ["pyqt", "PyQt"],
    ["pyqt5", "PyQt5"],
];

// Built in python modules
pub const BUILTIN_MODULES: [&str; 167] = ["__future__", "__main__", "_thread", "abc", "aifc", "argparse", "array", "ast", "asynchat", "asyncio", "asyncore", "atexit", "audioop", "base64", "bdb", "binascii", "binhex", "bisect", "builtins", "bz2", "calendar", "cgi", "cgitb", "chunk", "cmath", "cmd", "code", "codecs", "codeop", "colorsys", "compileall", "configparser", "contextlib", "contextvars", "copy", "copyreg", "cProfile", "csv", "ctypes", "dataclasses", "datetime", "decimal", "difflib", "dis", "doctest", "ensurepip", "enum", "errno", "faulthandler", "filecmp", "fileinput", "fnmatch", "fractions", "ftplib", "functools", "gc", "getopt", "getpass", "gettext", "glob", "graphlib", "gzip", "hashlib", "heapq", "hmac", "imaplib", "imghdr", "imp", "inspect", "io", "ipaddress", "itertools", "keyword", "lib2to3", "linecache", "locale", "lzma", "mailbox", "mailcap", "marshal", "math", "mimetypes", "mmap", "modulefinder", "netrc", "nntplib", "numbers", "operator", "optparse", "pathlib", "pdb", "pickle", "pickletools", "pkgutil", "platform", "plistlib", "poplib", "pprint", "profile", "pstats", "py_compile", "pyclbr", "pydoc", "queue", "quopri", "random", "re", "reprlib", "rlcompleter", "runpy", "sched", "secrets", "select", "selectors", "shelve", "shlex", "shutil", "signal", "site", "smtpd", "smtplib", "sndhdr", "socket", "socketserver", "sqlite3", "ssl", "stat", "statistics", "string", "stringprep", "struct", "subprocess", "sunau", "symtable", "sys", "sysconfig", "tabnanny", "tarfile", "telnetlib", "tempfile", "textwrap", "threading", "time", "timeit", "token", "tokenize", "trace", "traceback", "tracemalloc", "turtle", "turtledemo", "types", "typing", "unicodedata", "uu", "uuid", "venv", "warnings", "wave", "weakref", "webbrowser", "xdrlib", "zipapp", "zipfile", "zipimport", "zlib", "zoneinfo"];
"""An implementation of PEP-292 written in Rust"""

class Template:
    """A class for supporting $-substitutions"""
    def __init__(self, template: str) -> None: ...
    def substitute(**kw: dict[str,str]) -> str: ...
    def safe_substitute(**kw: dict[str,str]) -> str: ...

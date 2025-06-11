import os
import re
import sys
import pathlib
import yaml
import changelog

_name = "Wretch"
_publisher = "TheSillyBoi"
_identifier = f"{_publisher}.{_name}"

installer = yaml.safe_load(open(os.path.join(pathlib.Path(__file__).parent, f"templates/winget/{_identifier}"), "r", encoding="utf-8")) or {}

print(installer)
import os
import re
import sys

_changelog_path = os.path.join(os.path.dirname(__file__), "../../", "CHANGELOG.md")
try:
  with open(_changelog_path, "r", encoding="utf-8") as f:
    _changelog = f.read()
    _changelog = _changelog.strip()
    _changelog = re.sub(r'\n\n', '\n', _changelog)
    _changelog = re.sub(r'\n( .*)-', r'\n\1*', _changelog)
except FileNotFoundError:
  print("Error: Changelog file not found.")
  sys.exit(1)
except Exception as e:
  print(f"Error reading changelog: {e}")
  sys.exit(1)

# private methods
def __get_latest_version(should_group = True, with_prefix = True) -> str | re.Match[str]:
  """Find the latest version match in the changelog."""
  result = re.search(r'## (v?\d+\.\d+\.\d+)', _changelog) if with_prefix else re.search(r'## v?(\d+\.\d+\.\d+)', _changelog)
  return result.group(1) if should_group else result

def __get_previous_version(back: int = 1, should_group = True, with_prefix = True) -> str | re.Match[str]:
  """Find the version before the latest version in the changelog."""
  if back < 0:
    raise ValueError("The 'back' parameter must be a non-negative integer.")
  if back == 0:
    return __get_latest_version(should_group, with_prefix)
  pattern = r'## (v?\d+\.\d+\.\d+)' if with_prefix else r'## v?(\d+\.\d+\.\d+)'
  matches = list(re.finditer(pattern, _changelog))
  if len(matches) < back:
    return None
  previous_version = matches[back]
  return previous_version.group(1) if should_group else previous_version

# public methods
def get_changelog() -> str:
  """Read the changelog file and return its content."""
  return _changelog

def get_latest_version(with_prefix = True) -> str:
  """Find the latest version match in the changelog."""
  return __get_latest_version(with_prefix=with_prefix)

def get_previous_version(back: int = 1, with_prefix = True) -> str:
  """Find the version before the latest version in the changelog."""
  return __get_previous_version(back, True, with_prefix)

def get_latest_version_changes() -> str:
  """Get the changes for the latest version."""
  latest_version = __get_latest_version(False)
  if not latest_version:
    return ""
  
  # Find the next version section
  next_version = __get_previous_version(should_group=False)
  
  # Extract changes for the latest version
  if next_version:
    return _changelog[latest_version.end():next_version.start()].strip()
  else:
    return _changelog[latest_version.end():].strip()



with open(os.environ["GITHUB_OUTPUT"], "a") as f:
  f.write(f'changelog<<EOF\n{get_latest_version_changes()}\nEOF\n')
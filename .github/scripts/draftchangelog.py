import os
import re
import sys
import changelog

print("Latest Version:", changelog.get_latest_version())
print("Latest Version Changes:")
print(changelog.get_latest_version_changes())
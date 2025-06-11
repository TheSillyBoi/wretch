import os
import re
import sys
import pathlib
import yaml
import changelog

_name = "Wretch"
_publisher = "TheSillyBoi"
_identifier = f"{_publisher}.{_name}"

_winget_manifest_version = "1.10.0"

_installer_schema = f"https://aka.ms/winget-manifest.installer.{_winget_manifest_version}.schema.json"
_defaultLocales_schema = f"https://aka.ms/winget-manifest.defaultLocale.{_winget_manifest_version}.schema.json"
_version_schema = f"https://aka.ms/winget-manifest.version.{_winget_manifest_version}.schema.json"

_wretch_version = changelog.get_latest_version(with_prefix=False)
_wretch_changelog = re.sub(r'( *)\*', r'\1-', changelog.get_latest_version_changes(), flags=re.MULTILINE)

_output_path = os.path.join(os.path.dirname(__file__), "templates", "output", "winget", _publisher, _name, _wretch_version)
os.makedirs(_output_path, exist_ok=True)

installer = yaml.safe_load(open(os.path.join(os.path.dirname(__file__), f"templates/winget/{_identifier}.installer.yaml"), "r", encoding="utf-8")) or {}
defaultLocales = yaml.safe_load(open(os.path.join(os.path.dirname(__file__), f"templates/winget/{_identifier}.locale.en-US.yaml"), "r", encoding="utf-8")) or {}
version = yaml.safe_load(open(os.path.join(os.path.dirname(__file__), f"templates/winget/{_identifier}.yaml"), "r", encoding="utf-8")) or {}

class LiteralStr(str): pass
def literal_str_representer(dumper, data):
    return dumper.represent_scalar('tag:yaml.org,2002:str', data, style='|')
yaml.add_representer(LiteralStr, literal_str_representer)

# need to do hash and release date

installer["PackageVersion"] = _wretch_version
installer["Installers"][0]["InstallerUrl"] = installer["Installers"][0]["InstallerUrl"].replace("{VERSION}", _wretch_version)
installer["ManifestVersion"] = _winget_manifest_version

defaultLocales["PackageVersion"] = _wretch_version
defaultLocales["LicenseUrl"] = defaultLocales["LicenseUrl"].replace("{VERSION}", _wretch_version)
defaultLocales["ReleaseNotesUrl"] = defaultLocales["ReleaseNotesUrl"].replace("{VERSION}", _wretch_version)
defaultLocales["ReleaseNotes"] = LiteralStr(_wretch_changelog)
defaultLocales["ManifestVersion"] = _winget_manifest_version

version["PackageVersion"] = _wretch_version
version["ManifestVersion"] = _winget_manifest_version

# dump new files
with open(os.path.join(_output_path, f"{_identifier}.installer.yaml"), "w", encoding="utf-8") as f:
  f.write("# yaml-language-server: $schema=" + _installer_schema + "\n\n")
  f.write(yaml.dump(installer, allow_unicode=True, sort_keys=False, default_flow_style=False))
with open(os.path.join(_output_path, f"{_identifier}.locale.en-US.yaml"), "w", encoding="utf-8") as f:
  f.write("# yaml-language-server: $schema=" + _defaultLocales_schema + "\n\n")
  f.write(yaml.dump(defaultLocales, allow_unicode=True, sort_keys=False, default_flow_style=False))
with open(os.path.join(_output_path, f"{_identifier}.yaml"), "w", encoding="utf-8") as f:
  f.write("# yaml-language-server: $schema=" + _version_schema + "\n\n")
  f.write(yaml.dump(version, allow_unicode=True, sort_keys=False, default_flow_style=False))

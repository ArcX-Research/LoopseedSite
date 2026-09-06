#!/usr/bin/env python3
"""Connect this website to an explicitly chosen Loopseed checkout."""

from __future__ import annotations

import argparse
import json
import os
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
CONFIG_LIMIT = 64 * 1024
SETUP = 'Run scripts/connect-loopseed.sh "/absolute/path/to/Loopseed".'


def config_path() -> Path:
    configured = os.environ.get("LOOPSEEDSITE_CONFIG")
    return (
        Path(configured).expanduser().resolve()
        if configured
        else ROOT / ".loopseedsite/config.json"
    )


def load_config() -> dict:
    path = config_path()
    if not path.exists():
        return {"version": 1}
    if path.stat().st_size > CONFIG_LIMIT:
        raise ValueError(f"Connection settings exceed {CONFIG_LIMIT} bytes: {path}")
    try:
        config = json.loads(path.read_text(encoding="utf-8"))
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise ValueError(f"Invalid connection settings in {path}: {exc}") from exc
    if (
        not isinstance(config, dict)
        or type(config.get("version")) is not int
        or config["version"] != 1
    ):
        raise ValueError(
            f"Connection settings in {path} must be a version 1 JSON object."
        )
    return config


def validate_checkout(path: Path) -> Path:
    try:
        checkout = path.expanduser().resolve(strict=True)
    except OSError as exc:
        raise ValueError(f"Loopseed checkout is unavailable: {path}. {SETUP}") from exc
    if not checkout.is_dir() or not all(
        (checkout / marker).is_file()
        for marker in ("SOUL.md", "fish/daemon/Cargo.toml")
    ):
        raise ValueError(f"Not a Loopseed checkout: {checkout}. {SETUP}")
    return checkout


def loopseed_root(explicit: Path | None = None) -> Path:
    if explicit is not None:
        return validate_checkout(explicit)
    override = os.environ.get("LOOPSEED")
    if override:
        return validate_checkout(Path(override))
    configured = load_config().get("loopseed")
    if configured is None:
        raise ValueError(f"Loopseed is not connected. {SETUP}")
    if (
        not isinstance(configured, str)
        or not configured
        or not Path(configured).is_absolute()
    ):
        raise ValueError(
            f"The loopseed path in {config_path()} must be absolute. {SETUP}"
        )
    return validate_checkout(Path(configured))


def connect(path: Path) -> Path:
    checkout = validate_checkout(path)
    config = load_config()
    config["loopseed"] = str(checkout)
    destination = config_path()
    destination.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w",
            encoding="utf-8",
            dir=destination.parent,
            prefix=".config-",
            delete=False,
        ) as stream:
            temporary = Path(stream.name)
            os.fchmod(stream.fileno(), 0o600)
            json.dump(config, stream, indent=2)
            stream.write("\n")
        temporary.replace(destination)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
    return checkout


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    setup = commands.add_parser(
        "connect", help="save a Loopseed checkout in local settings"
    )
    setup.add_argument("checkout", type=Path)
    commands.add_parser("path", help="print the selected Loopseed checkout")
    args = parser.parse_args()
    try:
        if args.command == "connect":
            print(f"Connected Loopseed: {connect(args.checkout)}")
            print(f"Saved settings: {config_path()}")
        else:
            print(loopseed_root())
    except (OSError, ValueError) as exc:
        parser.error(str(exc))


if __name__ == "__main__":
    main()

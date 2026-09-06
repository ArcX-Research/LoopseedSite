"""Isolated connection and read-only export checks; no model data is opened."""

import json
import os
import shutil
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import loopseed_connection as connection


ROOT = Path(__file__).resolve().parent.parent


class ConnectionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="loopseed site connection ")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name).resolve()
        self.site = self.root / "web projects" / "deeply nested" / "LoopseedSite"
        (self.site / "scripts").mkdir(parents=True)
        for name in ("loopseed_connection.py", "export_skin.py", "connect-loopseed.sh"):
            shutil.copy2(ROOT / "scripts" / name, self.site / "scripts" / name)
        shutil.copy2(ROOT / "Makefile", self.site / "Makefile")
        self.config = self.site / ".loopseedsite/config.json"
        self.checkout = self.model("research elsewhere/Loopseed's #1 ? model")
        self.environment = {
            key: value
            for key, value in os.environ.items()
            if key not in {"LOOPSEED", "LOOPSEEDSITE_CONFIG", "MAKEFLAGS", "MFLAGS"}
        }
        self.environment["LOOPSEEDSITE_CONFIG"] = str(self.config)
        self.environment_patch = patch.dict(os.environ, self.environment, clear=True)
        self.environment_patch.start()
        self.addCleanup(self.environment_patch.stop)

    def model(self, relative, theta=0.17):
        checkout = self.root / relative
        (checkout / "fish/daemon").mkdir(parents=True)
        (checkout / "fish/daemon/Cargo.toml").write_text(
            '[package]\nname = "fish-daemon"\n'
        )
        (checkout / "SOUL.md").write_text(f"theta = {theta}\n")
        return checkout

    def run_command(self, *args, env=None, cwd=None):
        return subprocess.run(
            args,
            cwd=cwd or self.root,
            env=env or self.environment,
            capture_output=True,
            text=True,
            timeout=20,
        )

    def cli(self, *args, env=None):
        return self.run_command(
            sys.executable,
            str(self.site / "scripts/loopseed_connection.py"),
            *args,
            env=env,
        )

    def write_config(self, value):
        self.config.parent.mkdir(parents=True, exist_ok=True)
        self.config.write_text(json.dumps(value))

    def database(self, checkout):
        path = checkout / "fish/sediment.db"
        with sqlite3.connect(path) as db:
            db.execute("CREATE TABLE exchanges (id INTEGER PRIMARY KEY, speaker TEXT)")
            db.execute(
                "CREATE TABLE delta_log (ts TEXT, delta REAL, exchange_id INTEGER, delta_v INTEGER)"
            )
            db.executemany(
                "INSERT INTO exchanges VALUES (?, ?)", [(1, "keeper"), (2, "claude")]
            )
            db.executemany(
                "INSERT INTO delta_log VALUES (?, ?, ?, ?)",
                [
                    ("2026-08-01T10:00:00Z", 0.25, 1, 2),
                    ("2026-08-01T10:05:00Z", 0.15, 2, 2),
                ],
            )
        return path

    def test_unconfigured_never_selects_a_sibling(self):
        self.model("web projects/deeply nested/Loopseed")
        result = self.cli("path")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Loopseed is not connected", result.stderr)
        self.assertIn("scripts/connect-loopseed.sh", result.stderr)
        self.assertFalse(self.config.exists())
        help_result = self.run_command("make", "-C", str(self.site), "help")
        self.assertEqual(help_result.returncode, 0, help_result.stderr)

    def test_helper_persists_a_canonical_path_from_any_directory(self):
        alias = self.root / "a model link"
        alias.symlink_to(self.checkout, target_is_directory=True)
        env = {
            key: value
            for key, value in self.environment.items()
            if key != "LOOPSEEDSITE_CONFIG"
        }
        result = self.run_command(
            "bash",
            str(self.site / "scripts/connect-loopseed.sh"),
            "a model link",
            env=env,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            json.loads(self.config.read_text()),
            {
                "version": 1,
                "loopseed": str(self.checkout),
            },
        )
        self.assertEqual(self.config.stat().st_mode & 0o777, 0o600)
        self.assertEqual(self.cli("path", env=env).stdout.strip(), str(self.checkout))
        self.assertFalse((self.checkout / ".loopseedsite").exists())

    def test_reconnecting_preserves_other_local_settings(self):
        self.write_config(
            {"version": 1, "preview": {"port": 8790}, "loopseed": str(self.checkout)}
        )
        other = self.model("a different/deeply nested/model")
        connection.connect(other)
        self.assertEqual(connection.loopseed_root(), other)
        self.assertEqual(json.loads(self.config.read_text())["preview"], {"port": 8790})
        self.assertEqual(list(self.config.parent.glob(".config-*")), [])

    def test_explicit_and_environment_overrides_precede_saved_settings(self):
        connection.connect(self.checkout)
        other = self.model("another model")
        explicit = self.model("explicit model")
        with patch.dict(os.environ, {"LOOPSEED": str(other)}):
            self.assertEqual(connection.loopseed_root(), other)
            self.assertEqual(connection.loopseed_root(explicit), explicit)
        self.assertEqual(connection.loopseed_root(), self.checkout)
        self.write_config({"version": "invalid"})
        self.assertEqual(connection.loopseed_root(explicit), explicit)

    def test_invalid_configuration_and_missing_checkouts_fail_clearly(self):
        for config in (
            [],
            {"version": 2},
            {"version": True},
            {"version": 1, "loopseed": "../Loopseed"},
        ):
            with self.subTest(config=config):
                self.write_config(config)
                self.assertNotEqual(self.cli("path").returncode, 0)
        self.config.write_text("{")
        self.assertIn("Invalid connection settings", self.cli("path").stderr)
        self.config.write_text(" " * (connection.CONFIG_LIMIT + 1))
        self.assertIn("exceed", self.cli("path").stderr)
        self.config.unlink()
        self.assertIn(
            "Not a Loopseed checkout", self.cli("connect", str(self.root)).stderr
        )
        self.assertIn(
            "unavailable", self.cli("connect", str(self.root / "absent")).stderr
        )
        self.assertFalse(self.config.exists())

    def test_invalid_override_does_not_fall_back_to_a_saved_checkout(self):
        connection.connect(self.checkout)
        with patch.dict(os.environ, {"LOOPSEED": str(self.root / "absent")}):
            with self.assertRaisesRegex(ValueError, "unavailable"):
                connection.loopseed_root()

    def test_make_exports_from_saved_or_explicit_paths_without_writing_the_source(self):
        connection.connect(self.checkout)
        other = self.model("override path/Loopseed's #2 ? model", theta=0.29)
        databases = {path: self.database(path) for path in (self.checkout, other)}
        before = {path: db.read_bytes() for path, db in databases.items()}
        for arguments, expected_theta in (
            ([], 0.17),
            (["LOOPSEED=" + str(other)], 0.29),
        ):
            with self.subTest(arguments=arguments):
                result = self.run_command(
                    "make", "-C", str(self.site), "skin", *arguments
                )
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                output = json.loads(
                    (self.site / "crates/record/data/skin.json").read_text()
                )
                self.assertEqual(output["theta"], expected_theta)
                self.assertEqual(output["keeper_grades"], 1)
                self.assertEqual(output["guest_grades_excluded"], 1)
                self.assertEqual(output["source"], "sediment.db")
        self.assertEqual(
            before, {path: db.read_bytes() for path, db in databases.items()}
        )
        self.assertEqual(
            json.loads(self.config.read_text())["loopseed"], str(self.checkout)
        )

    def test_export_refuses_missing_data_without_creating_a_database(self):
        connection.connect(self.checkout)
        result = self.run_command("make", "-C", str(self.site), "skin")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("No sediment database", result.stderr)
        self.assertFalse((self.checkout / "fish/sediment.db").exists())
        self.assertFalse((self.site / "crates/record/data/skin.json").exists())

    def test_export_argument_precedes_environment_without_saving_settings(self):
        other = self.model("explicit/export source", theta=0.29)
        self.database(other)
        output = self.root / "exported chart.json"
        env = {**self.environment, "LOOPSEED": str(self.checkout)}
        result = self.run_command(
            sys.executable,
            str(self.site / "scripts/export_skin.py"),
            "--loopseed",
            str(other),
            "--out",
            str(output),
            env=env,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(output.read_text())["theta"], 0.29)
        self.assertFalse(self.config.exists())


if __name__ == "__main__":
    unittest.main()

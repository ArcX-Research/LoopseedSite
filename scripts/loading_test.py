"""Exercise cache identity, interrupted builds and direct-page HTTP responses."""
import http.client
from pathlib import Path
import re
import tempfile
import threading
import unittest
from unittest.mock import patch
from urllib.parse import urljoin, urlsplit

import prepare_dist
import serve


class LoadingTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.dist = self.root / "dist"

    def stage(self, name, binary):
        stage = self.root / name
        (stage / "pkg").mkdir(parents=True)
        (stage / "papers").mkdir()
        (stage / "pkg/loopseed_bg.wasm").write_bytes(binary)
        (stage / "pkg/loopseed.js").write_text(
            "const binary = new URL('loopseed_bg.wasm', import.meta.url);"
        )
        (stage / "index.html").write_text(
            '<link href="/styles.css"><script type="module">'
            "await import('/pkg/loopseed.js');</script>"
        )
        (stage / "papers/paper.html").write_text('<link href="/styles.css">Paper')
        (stage / "styles.css").write_text("body { color: black; }")
        return stage

    def entry_dependencies(self, html):
        js_url = re.search(r"import\('([^']+)'\)", html)[1]
        js = (self.dist / js_url.lstrip("/")).read_text()
        wasm_url = urljoin(js_url, re.search(r"new URL\('([^']+)'", js)[1])
        return js_url, wasm_url, (self.dist / wasm_url.lstrip("/")).read_bytes()

    def test_new_build_uses_matching_binary_and_keeps_old_page_loadable(self):
        first = self.stage("first", b"old binary")
        prepare_dist.prepare(first, ["/", "/results"])
        prepare_dist.publish(first, self.dist)
        old_html = (self.dist / "index.html").read_text()
        old_js, old_wasm, _ = self.entry_dependencies(old_html)

        second = self.stage("second", b"new binary")
        prepare_dist.prepare(second, ["/", "/results"])
        prepare_dist.publish(second, self.dist)
        new_html = (self.dist / "index.html").read_text()
        new_js, new_wasm, binary = self.entry_dependencies(new_html)
        self.assertEqual(binary, b"new binary")
        self.assertNotEqual(old_js, new_js)
        self.assertNotEqual(old_wasm, new_wasm)
        self.assertEqual(self.entry_dependencies(old_html)[2], b"old binary")
        self.assertNotIn("?", new_wasm)
        self.assertNotEqual(new_wasm, "/pkg/loopseed_bg.wasm")
        self.assertEqual((self.dist / "results/index.html").read_text(), new_html)
        paper = (self.dist / "papers/paper.html").read_text()
        self.assertRegex(paper, r'/styles.css\?v=[a-f0-9]{16}')

    def test_invalid_glue_does_not_publish_a_broken_build(self):
        first = self.stage("first", b"working")
        prepare_dist.prepare(first, ["/"])
        prepare_dist.publish(first, self.dist)
        expected = (self.dist / "index.html").read_bytes()
        broken = self.stage("broken", b"broken")
        (broken / "pkg/loopseed.js").write_text("unexpected glue format")
        with self.assertRaises(ValueError):
            prepare_dist.prepare(broken, ["/"])
            prepare_dist.publish(broken, self.dist)
        self.assertEqual((self.dist / "index.html").read_bytes(), expected)
        self.assertEqual(self.entry_dependencies(expected.decode())[2], b"working")

    def test_interrupted_asset_copy_keeps_complete_file_and_old_entry(self):
        first = self.stage("first", b"working")
        prepare_dist.prepare(first, ["/"])
        prepare_dist.publish(first, self.dist)
        expected_html = (self.dist / "index.html").read_bytes()
        expected_css = (self.dist / "styles.css").read_bytes()
        second = self.stage("second", b"new")
        prepare_dist.prepare(second, ["/"])
        copy = prepare_dist.shutil.copyfile

        def interrupted(source, target):
            if source.suffix == ".css":
                Path(target).write_bytes(b"partial")
                raise OSError("interrupted copy")
            return copy(source, target)

        with patch.object(prepare_dist.shutil, "copyfile", interrupted):
            with self.assertRaises(OSError):
                prepare_dist.publish(second, self.dist)
        self.assertEqual((self.dist / "index.html").read_bytes(), expected_html)
        self.assertEqual((self.dist / "styles.css").read_bytes(), expected_css)

    def test_http_entry_routes_and_missing_assets(self):
        stage = self.stage("first", b"wasm")
        prepare_dist.prepare(stage, prepare_dist.client_routes())
        prepare_dist.publish(stage, self.dist)
        with patch.object(serve, "DIST", self.dist), patch.object(serve.Handler, "log_message"):
            server = serve.Server(("127.0.0.1", 0), serve.Handler)
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                for path in ("/", "/results", "/results/?v=old", "/method#symbols", "/goals", "/record"):
                    for method in ("GET", "HEAD"):
                        with self.subTest(path=path, method=method):
                            connection = http.client.HTTPConnection(*server.server_address)
                            connection.request(method, urlsplit(path).path + ("?v=old" if "?" in path else ""))
                            response = connection.getresponse()
                            self.assertEqual(response.status, 200)
                            self.assertIn("text/html", response.getheader("Content-Type"))
                            self.assertEqual(response.getheader("Cache-Control"), "no-store")
                            body = response.read()
                            self.assertEqual(bool(body), method == "GET")
                            connection.close()
                for path in ("/pkg/missing.js", "/pkg/missing.wasm", "/missing.css"):
                    connection = http.client.HTTPConnection(*server.server_address)
                    connection.request("GET", path)
                    response = connection.getresponse()
                    self.assertEqual(response.status, 404)
                    self.assertNotIn(b"await import", response.read())
                    connection.close()
            finally:
                server.shutdown()
                server.server_close()
                thread.join()


if __name__ == "__main__":
    unittest.main()

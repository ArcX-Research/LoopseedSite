#!/usr/bin/env python3
"""Version matching JS/Wasm artifacts, then publish assets before HTML."""
import hashlib
from pathlib import Path
import re
import shutil
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def client_routes():
    app = (ROOT / "crates/site/src/app.rs").read_text(encoding="utf-8")
    routes = re.findall(r'path=path!\("([^"]+)"\)', app)
    if not routes:
        raise ValueError("No application routes found")
    return routes


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()[:16]


def prepare(stage, routes):
    wasm = stage / "pkg/loopseed_bg.wasm"
    wasm_name = f"loopseed_bg.{digest(wasm)}.wasm"
    js = stage / "pkg/loopseed.js"
    source = js.read_text(encoding="utf-8")
    if source.count("'loopseed_bg.wasm'") != 1:
        raise ValueError("Expected one wasm-bindgen binary URL; refusing an unmatched build")
    js.write_text(source.replace("'loopseed_bg.wasm'", repr(wasm_name)), encoding="utf-8")
    wasm.rename(wasm.with_name(wasm_name))
    js_name = f"loopseed.{digest(js)}.js"
    js.rename(js.with_name(js_name))

    replacements = {"/pkg/loopseed.js": f"/pkg/{js_name}"}
    for css in stage.rglob("*.css"):
        url = "/" + css.relative_to(stage).as_posix()
        replacements[url] = f"{url}?v={digest(css)}"
    for page in stage.rglob("*.html"):
        text = page.read_text(encoding="utf-8")
        for old, new in replacements.items():
            text = text.replace(f'"{old}"', f'"{new}"').replace(f"'{old}'", f"'{new}'")
        page.write_text(text, encoding="utf-8")

    # Physical entry pages also work when the host adds a trailing slash.
    # They avoid relying on a cached 404 response to start the router.
    for route in routes:
        if route == "/":
            continue
        if not re.fullmatch(r"/[a-z0-9-]+", route):
            raise ValueError(f"Unsupported static route: {route}")
        entry = stage / route.lstrip("/") / "index.html"
        entry.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(stage / "index.html", entry)
    shutil.copyfile(stage / "index.html", stage / "404.html")


def publish(stage, dist):
    paths = [p.relative_to(stage) for p in stage.rglob("*") if p.is_file()]
    # Each file is replaced atomically, with all dependencies ready before any
    # page can refer to them. An interrupted copy never exposes a partial file.
    for path in sorted(paths, key=lambda p: (p.suffix == ".html", str(p))):
        target = dist / path
        target.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=target.parent, delete=False) as handle:
            temporary = Path(handle.name)
        try:
            shutil.copyfile(stage / path, temporary)
            temporary.chmod(0o644)
            temporary.replace(target)
        finally:
            temporary.unlink(missing_ok=True)
    # Keep earlier bundles for pages opened just before a rebuild. Other
    # removed build artifacts should not remain in the deployed site.
    current = set(paths)
    for path in dist.rglob("*"):
        relative = path.relative_to(dist)
        if path.is_file() and relative not in current and relative.parts[0] != "pkg":
            path.unlink()


def main():
    stage, dist = map(Path, sys.argv[1:])
    prepare(stage, client_routes())
    publish(stage, dist)


if __name__ == "__main__":
    main()

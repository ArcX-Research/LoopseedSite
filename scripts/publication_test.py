"""Check the actual static publication artifacts that Scholar will receive."""
from html.parser import HTMLParser
from pathlib import Path
import unittest
from urllib.parse import urlparse
from urllib.robotparser import RobotFileParser
import zipfile

STATIC = Path(__file__).resolve().parents[1] / "static"


class PaperHTML(HTMLParser):
    def __init__(self, text):
        super().__init__()
        self.meta, self.links, self.scripts = {}, [], []
        self.abstract, self.paragraph = False, False
        self.abstract_text = []
        self.feed(text)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "meta":
            self.meta[attrs.get("name")] = attrs.get("content")
        if tag == "a":
            self.links.append(attrs.get("href", ""))
        if tag == "script":
            self.scripts.append(attrs)
        if tag == "section" and attrs.get("aria-labelledby") == "abstract":
            self.abstract = True
        if tag == "p" and self.abstract:
            self.paragraph = True

    def handle_endtag(self, tag):
        if tag == "section":
            self.abstract = False
        if tag == "p":
            self.paragraph = False

    def handle_data(self, text):
        if self.abstract and self.paragraph:
            self.abstract_text.append(text)


class PublicationTests(unittest.TestCase):
    def setUp(self):
        self.page = PaperHTML((STATIC / "papers/dynamical-synthesis.html").read_text())

    def test_abstract_and_identity_match_downloaded_source(self):
        with zipfile.ZipFile(STATIC / "papers/dynamical-synthesis-source.zip") as archive:
            manuscript = archive.read("manuscript.md").decode()
        abstract = manuscript.split("## Abstract\n", 1)[1].split("\n## ", 1)[0].strip()
        self.assertEqual("".join(self.page.abstract_text).strip(), abstract)
        self.assertEqual(self.page.meta["description"], abstract)
        self.assertEqual(self.page.meta["citation_title"], manuscript.splitlines()[0][2:])
        self.assertEqual(self.page.meta["citation_author"], "Ashiundu, Brian")
        self.assertIn("**Brian Ashiundu**", manuscript)
        self.assertEqual(self.page.meta["citation_publication_date"], "2026/09/11")
        self.assertFalse(self.page.scripts)

    def test_pdf_and_downloads_are_real_files(self):
        pdf = urlparse(self.page.meta["citation_pdf_url"])
        self.assertEqual(pdf.scheme, "https")
        self.assertEqual(pdf.netloc, "loopseed.io")
        self.assertEqual(pdf.path, "/papers/dynamical-synthesis.pdf")
        path = STATIC / pdf.path.lstrip("/")
        self.assertTrue(path.read_bytes().startswith(b"%PDF-"))
        self.assertLess(path.stat().st_size, 5_000_000)
        for href in self.page.links:
            if href.startswith("/papers/"):
                self.assertTrue((STATIC / href.lstrip("/")).is_file(), href)

    def test_discovery_does_not_require_javascript(self):
        homepage = PaperHTML((STATIC / "index.html").read_text())
        self.assertIn("/papers/dynamical-synthesis.html", homepage.links)
        robots = RobotFileParser()
        robots.parse((STATIC / "robots.txt").read_text().splitlines())
        for path in ("/", "/papers/dynamical-synthesis.html", "/papers/dynamical-synthesis.pdf"):
            self.assertTrue(robots.can_fetch("Googlebot", "https://loopseed.io" + path))


if __name__ == "__main__":
    unittest.main()

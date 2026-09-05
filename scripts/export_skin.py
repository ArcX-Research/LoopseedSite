#!/usr/bin/env python3
"""Export the keeper's delta sittings from a loopseed sediment, read-only.

The output is the data behind the skin figure on the site: every keeper grade
(guests are recorded in the body but never enter this file) and the median of
each sitting, where a sitting is a run of grades separated by less than thirty
minutes, exactly as fish/lab/UnNaivety.wl defines it. v1 and v2 grades are
different instruments and are exported as separate series.

    scripts/export_skin.py /path/to/loopseed/fish/sediment.db --soul /path/to/loopseed/SOUL.md
"""
import argparse
import json
import re
import sqlite3
import statistics
from datetime import datetime, timedelta, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "crates" / "record" / "data" / "skin.json"
INSTRUMENTATION_END = datetime(2026, 8, 3, 6, 0, tzinfo=timezone.utc)
SITTING_GAP = timedelta(minutes=30)


def parse(ts: str) -> datetime:
    return datetime.fromisoformat(ts.replace("Z", "+00:00")).astimezone(timezone.utc)


def sittings(points):
    out = []
    for point in points:
        if out and point[0] - out[-1][-1][0] < SITTING_GAP:
            out[-1].append(point)
        else:
            out.append([point])
    return out


def theta_from_soul(path: Path | None) -> float:
    if path is None or not path.is_file():
        return 0.35
    match = re.search(r"theta\s*=\s*([0-9.]+)", path.read_text(encoding="utf-8"))
    return float(match.group(1)) if match else 0.35


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("sediment", type=Path)
    ap.add_argument("--soul", type=Path, default=None)
    ap.add_argument("--out", type=Path, default=OUT)
    args = ap.parse_args()

    db = sqlite3.connect(f"file:{args.sediment.resolve()}?mode=ro", uri=True)
    rows = db.execute(
        "SELECT d.ts, d.delta, COALESCE(e.speaker, 'keeper'), COALESCE(d.delta_v, 1) "
        "FROM delta_log d LEFT JOIN exchanges e ON e.id = d.exchange_id ORDER BY d.ts"
    ).fetchall()
    db.close()

    keeper = [(parse(ts), float(delta), int(version)) for ts, delta, speaker, version in rows if speaker == "keeper"]
    guests = sum(1 for _, _, speaker, _ in rows if speaker != "keeper")
    epoch = min((parse(ts) for ts, _, _, version in rows if int(version or 1) == 2), default=None)

    windows = []
    for version in (1, 2):
        for sitting in sittings([point for point in keeper if point[2] == version]):
            windows.append(
                {
                    "v": version,
                    "t": int(sitting[0][0].timestamp()),
                    "end": int(sitting[-1][0].timestamp()),
                    "n": len(sitting),
                    "median": round(statistics.median(point[1] for point in sitting), 4),
                }
            )

    out = {
        "exported_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "source": args.sediment.name,
        "theta": theta_from_soul(args.soul),
        "epoch_unix": int(epoch.timestamp()) if epoch else None,
        "instrumentation_end_unix": int(INSTRUMENTATION_END.timestamp()),
        "keeper_grades": len(keeper),
        "guest_grades_excluded": guests,
        "grades": [{"t": int(t.timestamp()), "d": round(d, 4), "v": v} for t, d, v in keeper],
        "windows": windows,
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(out, separators=(",", ":")) + "\n", encoding="utf-8")
    print(f"wrote {args.out} ({len(keeper)} keeper grades, {len(windows)} sittings, {guests} guest grades excluded)")


if __name__ == "__main__":
    main()

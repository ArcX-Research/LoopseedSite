#!/usr/bin/env python3
"""Export prediction-error scores and session medians without changing the database.

The regular participant's scores are exported one by one with their session medians; guest
scores are exported as daily quartiles and session medians. Teachers and weather are pooled by
group, each classroom keeps its own series. v1 and v2 stay separate. Session gaps follow
fish/lab/UnNaivety.wl.
"""
import argparse
import json
import re
import sqlite3
import statistics
from collections import defaultdict
from datetime import datetime, timedelta, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "crates" / "record" / "data" / "skin.json"
INSTRUMENTATION_END = datetime(2026, 8, 3, 6, 0, tzinfo=timezone.utc)
SITTING_GAP = timedelta(minutes=30)

# Guest speakers by group. A speaker outside every group is counted and omitted from the chart.
# A pooled group draws one series; an unpooled group draws one series per speaker.
GROUPS = (
    ("teachers", "AI teachers", True, lambda speaker: speaker in {"claude", "codex", "fable"}),
    ("classrooms", "teaching programs", False, lambda speaker: speaker.startswith("class")),
    ("weather", "external input", True, lambda speaker: speaker == "ocean"),
)


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


def group_of(speaker: str):
    return next(((key, pooled) for key, _, pooled, member in GROUPS if member(speaker)), None)


def quartiles(values):
    if len(values) < 2:
        return values[0], values[0], values[0]
    q1, q2, q3 = statistics.quantiles(values, n=4)
    return q1, q2, q3


def guest_export(rows):
    grades = defaultdict(list)  # (group, speaker, version) -> [(t, delta)]
    days = defaultdict(list)  # (group, series, version, day) -> [delta]
    omitted = defaultdict(int)
    for ts, delta, speaker, version in rows:
        if speaker == "keeper":
            continue
        found = group_of(speaker)
        if found is None:
            omitted[speaker] += 1
            continue
        group, pooled = found
        t = parse(ts)
        grades[(group, speaker, int(version))].append((t, float(delta)))
        day = t.replace(hour=0, minute=0, second=0, microsecond=0)
        series = group if pooled else speaker
        days[(group, series, int(version), day)].append(float(delta))

    groups = []
    for key, label, pooled, _ in GROUPS:
        members = sorted({speaker for (group, speaker, _) in grades if group == key})
        speakers = []
        for speaker in members:
            values = [delta for (group, who, _), points in grades.items() if group == key and who == speaker for _, delta in points]
            speakers.append({"speaker": speaker, "grades": len(values), "median": round(statistics.median(values), 4)})
        values = [delta for (group, _, _), points in grades.items() if group == key for _, delta in points]
        groups.append(
            {
                "key": key,
                "label": label,
                "pooled": pooled,
                "speakers": speakers,
                "grades": len(values),
                "median": round(statistics.median(values), 4) if values else None,
            }
        )
    day_records = []
    for (group, series, version, day), values in sorted(days.items(), key=lambda item: (item[0][0], item[0][1], item[0][2], item[0][3])):
        q1, q2, q3 = quartiles(sorted(values))
        day_records.append(
            {
                "g": group,
                "s": series,
                "v": version,
                "t": int(day.timestamp()),
                "n": len(values),
                "q1": round(q1, 4),
                "median": round(q2, 4),
                "q3": round(q3, 4),
            }
        )
    sitting_records = []
    for (group, speaker, version), points in sorted(grades.items()):
        for sitting in sittings(points):
            sitting_records.append(
                {
                    "g": group,
                    "speaker": speaker,
                    "v": version,
                    "t": int(sitting[0][0].timestamp()),
                    "end": int(sitting[-1][0].timestamp()),
                    "n": len(sitting),
                    "median": round(statistics.median(delta for _, delta in sitting), 4),
                }
            )
    sitting_records.sort(key=lambda record: record["t"])
    return {
        "groups": groups,
        "omitted": {"speakers": sorted(omitted), "grades": sum(omitted.values())},
        "days": day_records,
        "sittings": sitting_records,
    }


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
        "guests": guest_export(rows),
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(out, separators=(",", ":")) + "\n", encoding="utf-8")
    guest = out["guests"]
    print(
        f"wrote {args.out} ({len(keeper)} keeper grades, {len(windows)} sittings; "
        f"{guests} guest grades: {len(guest['days'])} day records, {len(guest['sittings'])} sittings, "
        f"{guest['omitted']['grades']} omitted)"
    )


if __name__ == "__main__":
    main()

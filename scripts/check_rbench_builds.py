#!/usr/bin/env python3

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    print("Python 3.11+ is required for tomllib", file=sys.stderr)
    raise


@dataclass
class BuildResult:
    crate_dir: Path
    package_name: str
    return_code: int
    duration_seconds: float

    @property
    def ok(self) -> bool:
        return self.return_code == 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run `cargo build` for every crate under RBench."
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="Repository root containing the RBench directory.",
    )
    parser.add_argument(
        "--rbench-dir",
        type=Path,
        help="Override the RBench directory location.",
    )
    parser.add_argument(
        "--fail-fast",
        action="store_true",
        help="Stop after the first build failure.",
    )
    parser.add_argument(
        "--release",
        action="store_true",
        help="Pass --release to cargo build.",
    )
    parser.add_argument(
        "--keep-going",
        action="store_true",
        help="Pass --keep-going to cargo build when supported by your cargo version.",
    )
    parser.add_argument(
        "--crate",
        dest="crate_filters",
        action="append",
        default=[],
        help="Only build crate directories whose path contains this value. Repeatable.",
    )
    return parser.parse_args()


def discover_crates(rbench_dir: Path, crate_filters: list[str]) -> list[tuple[Path, str]]:
    manifests = sorted(rbench_dir.glob("*/Cargo.toml"))
    crates: list[tuple[Path, str]] = []

    for manifest in manifests:
        crate_dir = manifest.parent
        crate_dir_text = str(crate_dir)
        if crate_filters and not all(filter_text in crate_dir_text for filter_text in crate_filters):
            continue

        with manifest.open("rb") as fh:
            package_name = tomllib.load(fh)["package"]["name"]
        crates.append((crate_dir, package_name))

    return crates


def build_crate(crate_dir: Path, package_name: str, cargo_args: list[str]) -> BuildResult:
    command = ["cargo", "build", *cargo_args]
    print(f"==> {crate_dir.relative_to(crate_dir.parents[1])} [{package_name}]", flush=True)
    started_at = time.monotonic()
    completed = subprocess.run(command, cwd=crate_dir, check=False)
    duration_seconds = time.monotonic() - started_at

    status = "ok" if completed.returncode == 0 else "failed"
    print(f"<== {status} in {duration_seconds:.1f}s\n", flush=True)
    return BuildResult(crate_dir, package_name, completed.returncode, duration_seconds)


def main() -> int:
    args = parse_args()

    repo_root = args.root.resolve()
    rbench_dir = (args.rbench_dir or repo_root / "RBench").resolve()
    if not rbench_dir.is_dir():
        print(f"RBench directory not found: {rbench_dir}", file=sys.stderr)
        return 2

    crates = discover_crates(rbench_dir, args.crate_filters)
    if not crates:
        print("No crates matched.", file=sys.stderr)
        return 2

    cargo_args: list[str] = []
    if args.release:
        cargo_args.append("--release")
    if args.keep_going:
        cargo_args.append("--keep-going")

    print(f"Building {len(crates)} crate(s) from {os.fspath(rbench_dir)}\n", flush=True)

    results: list[BuildResult] = []
    for crate_dir, package_name in crates:
        result = build_crate(crate_dir, package_name, cargo_args)
        results.append(result)
        if args.fail_fast and not result.ok:
            break

    failures = [result for result in results if not result.ok]
    successes = len(results) - len(failures)
    total_duration = sum(result.duration_seconds for result in results)

    print("Summary", flush=True)
    print(f"  succeeded: {successes}", flush=True)
    print(f"  failed:    {len(failures)}", flush=True)
    print(f"  total:     {len(results)}", flush=True)
    print(f"  duration:  {total_duration:.1f}s", flush=True)

    if failures:
        print("\nFailures", flush=True)
        for failure in failures:
            relative_dir = failure.crate_dir.relative_to(repo_root)
            print(
                f"  {relative_dir} [{failure.package_name}] "
                f"(exit {failure.return_code}, {failure.duration_seconds:.1f}s)",
                flush=True,
            )
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

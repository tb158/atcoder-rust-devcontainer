#!/usr/bin/env python3
"""Merge VS Code .code-snippets files into a single Zed snippets file.

Zed reads one snippet file per language at ~/.config/zed/snippets/<language>.json
using the same schema as VS Code (name -> {prefix, body, description}). The only
difference is that Zed ignores VS Code's "scope" field, so we drop it.

Usage:
    snippets_to_zed.py OUTPUT.json INPUT1.code-snippets [INPUT2.code-snippets ...]

Later inputs win on name collisions (so hand-written snippets override generated
ones). Prefix collisions across different names are fine and kept as-is.
"""
import json
import sys


def _strip_comments(text: str) -> str:
    """Remove // and /* */ comments, ignoring anything inside string literals.

    Rust doc comments (///) live inside JSON string values, so the scanner must
    stay string-aware and only treat // and /* as comments outside strings.
    """
    out: list[str] = []
    i, n = 0, len(text)
    in_str = esc = False
    while i < n:
        c = text[i]
        if in_str:
            out.append(c)
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
            i += 1
        elif c == '"':
            in_str = True
            out.append(c)
            i += 1
        elif c == "/" and i + 1 < n and text[i + 1] == "/":
            i += 2
            while i < n and text[i] != "\n":
                i += 1
        elif c == "/" and i + 1 < n and text[i + 1] == "*":
            i += 2
            while i + 1 < n and not (text[i] == "*" and text[i + 1] == "/"):
                i += 1
            i += 2
        else:
            out.append(c)
            i += 1
    return "".join(out)


def _remove_trailing_commas(text: str) -> str:
    """Drop commas immediately before } or ], ignoring those inside strings."""
    out: list[str] = []
    i, n = 0, len(text)
    in_str = esc = False
    while i < n:
        c = text[i]
        if in_str:
            out.append(c)
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
            i += 1
            continue
        if c == '"':
            in_str = True
            out.append(c)
            i += 1
            continue
        if c == ",":
            j = i + 1
            while j < n and text[j] in " \t\r\n":
                j += 1
            if j < n and text[j] in "}]":
                i += 1  # skip the trailing comma
                continue
        out.append(c)
        i += 1
    return "".join(out)


def load_jsonc(path: str) -> dict:
    """Load a VS Code .code-snippets file, tolerating comments / trailing commas."""
    with open(path, encoding="utf-8") as f:
        text = f.read()
    return json.loads(_remove_trailing_commas(_strip_comments(text)))


def main() -> int:
    if len(sys.argv) < 3:
        sys.stderr.write(__doc__)
        return 2

    out_path = sys.argv[1]
    in_paths = sys.argv[2:]

    merged: dict[str, dict] = {}
    collisions: list[str] = []
    for path in in_paths:
        data = load_jsonc(path)
        for name, entry in data.items():
            if name in merged:
                collisions.append(name)
            # Drop VS Code-only "scope"; keep prefix/body/description/etc.
            cleaned = {k: v for k, v in entry.items() if k != "scope"}
            merged[name] = cleaned

    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(merged, f, ensure_ascii=False, indent=2)
        f.write("\n")

    print(f"Wrote {len(merged)} snippets to {out_path}")
    if collisions:
        print(f"Note: {len(collisions)} name collision(s) overridden: "
              + ", ".join(sorted(set(collisions))))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

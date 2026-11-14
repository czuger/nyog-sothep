"""Detect the outer black frame lines of the map and of the token sheet.

For each side we scan from outside (wood -> white paper) inward and record the
first dark pixel after a run of white paper.
"""
import sys, json
import cv2
import numpy as np

src = sys.argv[1]
img = cv2.imread(src)
H, W = img.shape[:2]
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY).astype(np.int32)
hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
white = (gray > 170) & (hsv[..., 1] < 60)
dark = (gray < 110).astype(np.uint8)
# keep only long straight runs, so text and torn paper edges are ignored
hdark = cv2.morphologyEx(dark, cv2.MORPH_OPEN, np.ones((1, 31), np.uint8)).astype(bool)
vdark = cv2.morphologyEx(dark, cv2.MORPH_OPEN, np.ones((31, 1), np.uint8)).astype(bool)


def scan(line_white, line_dark, min_white=6):
    """Return index of first dark pixel after a run of >= min_white white pixels."""
    run, armed = 0, False
    for i in range(len(line_white)):
        if line_white[i]:
            run += 1
            if run >= min_white:
                armed = True
        else:
            run = 0
            if line_dark[i] and armed:
                return i
    return None


def scan_side(side, ranges, start, stop):
    """side: 'top','bottom','left','right'. ranges: positions along the side.
    start/stop: scan range across the side (in scan direction)."""
    pts = []
    for p in ranges:
        if side in ("top", "bottom"):
            idx = np.arange(start, stop, 1 if stop > start else -1)
            lw, ld = white[idx, p], hdark[idx, p]
        else:
            idx = np.arange(start, stop, 1 if stop > start else -1)
            lw, ld = white[p, idx], vdark[p, idx]
        i = scan(lw, ld)
        if i is not None:
            q = int(idx[i])
            pts.append((p, q) if side in ("top", "bottom") else (q, p))
            # store as (x, y)
    if side in ("top", "bottom"):
        return [(x, y) for x, y in pts]
    return [(x, y) for x, y in pts]


def fit(pts, vertical, deg=2, iters=6, tol=4.0):
    pts = np.array(pts, dtype=float)
    a, b = (pts[:, 1], pts[:, 0]) if vertical else (pts[:, 0], pts[:, 1])
    # RANSAC line first, so that a cluster of bad points cannot bend the fit
    rng = np.random.default_rng(0)
    best = None
    for _ in range(2000):
        i, j = rng.choice(len(a), 2, replace=False)
        if a[i] == a[j]:
            continue
        c = np.polyfit(a[[i, j]], b[[i, j]], 1)
        n = (np.abs(np.polyval(c, a) - b) < 6).sum()
        if best is None or n > best[0]:
            best = (n, c)
    keep = np.abs(np.polyval(best[1], a) - b) < 6
    for _ in range(iters):
        c = np.polyfit(a[keep], b[keep], deg)
        keep = np.abs(np.polyval(c, a) - b) < tol
    return c.tolist(), int(keep.sum()), len(a), float(np.abs(np.polyval(c, a[keep]) - b[keep]).max())


def main():
    out = {}
    # ---- map outer frame (approx full-res: x 520..3260, y 88..2312)
    xs = list(range(600, 3200, 10))
    ys = list(range(180, 2240, 10))
    sides = {
        "top": scan_side("top", xs, 5, 400),
        "bottom": scan_side("bottom", xs, H - 5, 2000),
        "right": scan_side("right", ys, W - 5, 3000),
        "left": scan_side("left", ys, 495, 800),
    }
    res = {}
    for k, pts in sides.items():
        res[k] = fit(pts, vertical=k in ("left", "right"))
        res[k + "_pts"] = pts
        print("map", k, res[k][1:], file=sys.stderr)
    out["map"] = res

    # ---- token sheet outer border (approx x 72..500, y 88..1868)
    xs = list(range(110, 470, 6))
    ys = list(range(140, 1820, 8))
    sides = {
        "top": scan_side("top", xs, 5, 400),
        "bottom": scan_side("bottom", xs, 2100, 1600),
        "left": scan_side("left", ys, 5, 300),
        "right": scan_side("right", ys, 515, 300),
    }
    res = {}
    for k, pts in sides.items():
        res[k] = fit(pts, vertical=k in ("left", "right"))
        res[k + "_pts"] = pts
        print("tokens", k, res[k][1:], file=sys.stderr)
    out["tokens"] = res
    json.dump(out, open(sys.argv[2], "w"))


main()

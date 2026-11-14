"""Rectify a region bounded by four fitted edge curves.

A homography from the four corners removes the perspective; a Coons-patch
correction on top of it straightens the remaining bow of the paper.
"""
import json
import sys

import cv2
import numpy as np


def curve(c, vertical):
    c = np.array(c)
    if vertical:
        return lambda t: np.stack([np.polyval(c, t), t], -1)  # x = f(y)
    return lambda t: np.stack([t, np.polyval(c, t)], -1)  # y = f(x)


def intersect(ch, cv_, guess):
    """Intersection of a horizontal curve y=fh(x) and a vertical curve x=fv(y)."""
    x, y = guess
    for _ in range(50):
        y = np.polyval(ch, x)
        x = np.polyval(cv_, y)
    return np.array([x, y])


def rectify(img, e, scale=1.0, pad=0):
    top, bot, left, right = (np.array(e[k][0]) for k in ("top", "bottom", "left", "right"))
    tl = intersect(top, left, (0, 0))
    tr = intersect(top, right, (img.shape[1], 0))
    bl = intersect(bot, left, (0, img.shape[0]))
    br = intersect(bot, right, img.shape[1::-1])

    w = (np.linalg.norm(tr - tl) + np.linalg.norm(br - bl)) / 2 * scale
    h = (np.linalg.norm(bl - tl) + np.linalg.norm(br - tr)) / 2 * scale
    W, H = int(round(w)), int(round(h))

    src = np.float32([tl, tr, br, bl])
    dst = np.float32([[0, 0], [W, 0], [W, H], [0, H]])
    Hm = cv2.getPerspectiveTransform(src, dst)
    Hinv = np.linalg.inv(Hm)

    def to_rect(p):
        return cv2.perspectiveTransform(p.reshape(-1, 1, 2).astype(np.float64), Hm).reshape(-1, 2)

    n = 400
    # Edge curves in rectified space: deviation from the ideal straight side,
    # as a function of the position along the side.
    tp = to_rect(curve(top, False)(np.linspace(tl[0], tr[0], n)))
    bp = to_rect(curve(bot, False)(np.linspace(bl[0], br[0], n)))
    lp = to_rect(curve(left, True)(np.linspace(tl[1], bl[1], n)))
    rp = to_rect(curve(right, True)(np.linspace(tr[1], br[1], n)))

    u = np.arange(W + 2 * pad, dtype=np.float64) - pad
    v = np.arange(H + 2 * pad, dtype=np.float64) - pad
    dt = np.interp(u, tp[:, 0], tp[:, 1])          # y of the top edge at x=u
    db = np.interp(u, bp[:, 0], bp[:, 1]) - H      # offset of the bottom edge
    dl = np.interp(v, lp[:, 1], lp[:, 0])          # x of the left edge at y=v
    dr = np.interp(v, rp[:, 1], rp[:, 0]) - W

    U, V = np.meshgrid(u, v)
    fu, fv = np.clip(U / W, 0, 1), np.clip(V / H, 0, 1)
    X = U + (1 - fu) * dl[:, None] + fu * dr[:, None]
    Y = V + (1 - fv) * dt[None, :] + fv * db[None, :]

    # rectified -> photo
    den = Hinv[2, 0] * X + Hinv[2, 1] * Y + Hinv[2, 2]
    mx = ((Hinv[0, 0] * X + Hinv[0, 1] * Y + Hinv[0, 2]) / den).astype(np.float32)
    my = ((Hinv[1, 0] * X + Hinv[1, 1] * Y + Hinv[1, 2]) / den).astype(np.float32)
    out = cv2.remap(img, mx, my, cv2.INTER_LANCZOS4, borderMode=cv2.BORDER_REPLICATE)
    return out, dict(corners=[tl.tolist(), tr.tolist(), br.tolist(), bl.tolist()], size=(W, H))


if __name__ == "__main__":
    img = cv2.imread(sys.argv[1])
    edges = json.load(open(sys.argv[2]))
    region, outfile = sys.argv[3], sys.argv[4]
    pad = int(sys.argv[5]) if len(sys.argv) > 5 else 0
    out, info = rectify(img, edges[region], pad=pad)
    print(region, info)
    cv2.imwrite(outfile, out)

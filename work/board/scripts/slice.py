"""Split the rectified token sheet into its 4 x 20 cells."""
import os
import sys

import cv2
import numpy as np

ROWS = [
    "poirot hercule hastings le_capitaine",
    "sandy lemon professor nyogsothep",
    "fanatiques fanatiques fanatiques fanatiques",
    "fanatiques fanatiques fanatiques fanatiques",
    "fanatiques fanatiques goules goules",
    "goules goules goules goules",
    "goules goules profonds profonds",
    "profonds profonds profonds profonds",
    "profonds profonds reves reves",
    "reves reves reves reves",
    "horreur_volante choses_brume choses_brume choses_brume",
    "habitants habitants habitants habitants",
    "teleportation teleportation tempete tempete",
    "sign sign sign sign",
    "sign sign weapon weapon",
    "weapon weapon weapon weapon",
    "medaillon medaillon medaillon medaillon",
    "medaillon medaillon destroyed_city destroyed_city",
    "destroyed_city destroyed_city destroyed_city destroyed_city",
    "destroyed_city destroyed_city destroyed_city destroyed_city",
]
NCOL, NROW = 4, len(ROWS)

img = cv2.imread(sys.argv[1])
outdir = sys.argv[2]
os.makedirs(outdir, exist_ok=True)
H, W = img.shape[:2]
dark = (cv2.cvtColor(img, cv2.COLOR_BGR2GRAY) < 90).astype(np.float64)


def grid_lines(profile, n, size, win=10):
    """Snap the n+1 evenly spaced positions to the darkest nearby line."""
    pos = []
    for k in range(n + 1):
        e = int(round(k * size / n))
        lo, hi = max(0, e - win), min(len(profile), e + win + 1)
        pos.append(lo + int(np.argmax(profile[lo:hi])))
    return pos


# The grid lines are thin and continuous, so their dark-pixel fraction is high
xs = grid_lines(dark.mean(0), NCOL, W)
ys = grid_lines(dark.mean(1), NROW, H)
print("cols", xs)
print("rows", ys)

inset = 3  # drop the black grid line itself
counts = {}
for r, row in enumerate(ROWS):
    for c, name in enumerate(row.split()):
        counts[name] = counts.get(name, 0) + 1
        cell = img[ys[r] + inset:ys[r + 1] - inset + 1, xs[c] + inset:xs[c + 1] - inset + 1]
        cv2.imwrite(os.path.join(outdir, f"{name}_{counts[name]}.png"), cell)
print({k: v for k, v in counts.items()})

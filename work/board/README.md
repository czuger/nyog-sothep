# Cutting the board and the counters out of the photo

These scripts made `work/board/map.png` (2242×2732) and `work/board/tokens/*.png` from
`work/6a013486df16fc970c0133f3bce849970b.jpg`, the photo of the board with the counter sheet on its left.

```sh
python3 -m venv /tmp/board-venv
/tmp/board-venv/bin/pip install opencv-python numpy
/tmp/board-venv/bin/python work/board/scripts/detect.py work/6a013486df16fc970c0133f3bce849970b.jpg /tmp/edges.json
/tmp/board-venv/bin/python work/board/scripts/warp.py work/6a013486df16fc970c0133f3bce849970b.jpg /tmp/edges.json map work/board/map.png
/tmp/board-venv/bin/python work/board/scripts/warp.py work/6a013486df16fc970c0133f3bce849970b.jpg /tmp/edges.json tokens work/board/tokens_sheet.png
/tmp/board-venv/bin/python work/board/scripts/slice.py work/board/tokens_sheet.png work/board/tokens
```

- `detect.py` finds the black frame lines of the map and of the counter sheet and fits a curve to each side.
- `warp.py` straightens one region (`map` or `tokens`): a homography removes the perspective, then a Coons
  patch removes the bow of the paper.
- `slice.py` cuts the counter sheet into its 4×20 cells; which cell is which counter is listed at the top of
  the script.

The images served by the game come from these files:

```sh
magick work/board/map.png -strip -quality 90 -sampling-factor 4:4:4 img/board.jpg
cp work/board/tokens/goules_1.png img/token-ghouls.png   # and so on, one copy per kind
```

Every copy of a counter is identical, so only the first slice of each kind is used (`token-<kind>.png`,
`token-investigator-<name>.png`, `token-professor.png`, `token-nyog-sothep.png`, `token-destroyed-city.png`,
`token-weapon.png`, `token-sign.png`, `token-medallion.png`). `img/boat.png`, drawn under an investigator at
sea, comes from the old Rails app.

The search windows in `detect.py` are measured on this photo: another photo needs them measured again. The
city positions in `src/domain/map.rs` were measured on `map.png` by detecting the red city dots and the
green port dots with ImageMagick (`-color-threshold` then `-connected-components`).

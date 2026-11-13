# Fan-made object tokens

Tokens drawn for the fan-made version (`fan_mode_proposition.md`), imitating the original counters: black line art
on the pale yellow of the object counters, with a slight wobble (`feTurbulence` filter) and an upper-case label.

Each `<code>.svg` here is the source of `img/token-<code>.png` (82×80): `shotgun`, `dynamite`, `mnar_stone`,
`pass`, `laudanum`, `amulet`, `ibn_ghazi_powder`, `necronomicon`.

To rebuild a token after editing its SVG (Quick Look renders it, ImageMagick shrinks it and adds the blur and grain
of the scans):

```sh
qlmanage -t -s 656 -o . shotgun.svg
magick shotgun.svg.png -resize '656x640!' -resize '82x80!' -blur 0x0.5 -attenuate 0.25 +noise Gaussian ../../../img/token-shotgun.png
rm shotgun.svg.png
```

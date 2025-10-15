https://imagemagick.org/script/command-line-tools.php

https://github.com/imagemagick/imagemagick

`magick identify  -verbose .webp`

---

https://www.libpng.org/pub/png/apps/pngcheck.html

https://github.com/pnggroup/pngcheck

`pngcheck -cvt .png`

---

https://xiph.org/flac/

https://github.com/xiph/flac/blob/master/man/metaflac.md

`metaflac --list .flac`

`flac -a .flac`

---

https://aomedia.org/specifications/avif

https://github.com/AOMediaCodec/libavif/blob/main/doc/avifdec.1.md

`avifdec --info .avif`

`avifgainmaputil`

---

https://github.com/libjxl/libjxl/blob/main/doc/man/cjxl.txt

`jxlinfo -v .jxl`

---

https://mediaarea.net/en/MediaInfo

https://github.com/MediaArea/MediaInfo

`mediainfo -f .ogg`

---

https://github.com/xiph/vorbis-tools

---

https://exiftool.org

https://github.com/exiftool/exiftool

`exiftool -v .png`

---

https://gpac.io/downloads/gpac-nightly-builds

https://github.com/gpac/gpac

`mp4box -info .mp4`
`gpac -info .mp4`

---

https://ffmpeg.org

https://github.com/FFmpeg/FFmpeg

`ffprobe -v info nine.mp4`

ffmpeg -i 1.mp4 -vf "scale=512:512:force_original_aspect_ratio=decrease,fps=30" -c:v libvpx-vp9 -b:v 20000k -crf 20 -an -deadline best 1.webm

<!-- `ffprobe -v trace .mp4` -->

---

https://github.com/xiph/rav1e

`rav1e`

---

https://libjpeg-turbo.org

https://github.com/libjpeg-turbo/libjpeg-turbo

`djpeg`

---

https://developers.google.com/speed/webp

https://chromium.googlesource.com/webm/libwebp

`webpinfo -diag -summary -bitstream_info .webp`

### compile

`nmake /f Makefile.vc CFG=release-static RTLIBCFG=static OBJDIR=output ARCH=x64`

```sh
mkdir build
cd build
cmake -G "NMake Makefiles" -DWEBP_BUILD_EXTRAS=ON -DWEBP_BUILD_WEBPINFO=ON ..
nmake
```

---

https://ezgif.com

https://www.gzip.org

---

https://drawabox.com/

https://www.ctrlpaint.com/

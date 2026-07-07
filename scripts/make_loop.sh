#!/bin/bash

set -e

INPUT="$1"
OUTPUT="${2:-resources/static/assets/flower-loop.webm}"

if [ -z "$INPUT" ]; then
    echo "Usage: $0 <input> [output]"
    exit 1
fi

if [ ! -f "$INPUT" ]; then
    echo "Error: input file '$INPUT' not found"
    exit 1
fi

echo "Input:  $INPUT"
echo "Output: $OUTPUT"
echo ""

ffmpeg -i "$INPUT" \
    -filter_complex \
        "[0:v]split[fwd][rev];
         [rev]reverse[revout];
         [fwd][revout]concat=n=2:v=1[loop];
         [loop]scale=iw/8:ih/8,hue=s=0,fps=15[out]" \
    -map "[out]" \
    -c:v libvpx-vp9 \
    -crf 50 \
    -b:v 0 \
    -an \
    "$OUTPUT" \
    -y

echo ""
echo "Done: $(du -sh "$OUTPUT" | cut -f1)"

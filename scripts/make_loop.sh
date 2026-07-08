#!/bin/bash

set -e

INPUT="$1"
OUTPUT_WEBM="${2:-resources/static/assets/flower-loop.webm}"
OUTPUT_MP4="${3:-resources/static/assets/flower-loop.mp4}"

if [ -z "$INPUT" ]; then
    echo "Usage: $0 <input> [output]"
    exit 1
fi

if [ ! -f "$INPUT" ]; then
    echo "Error: input file '$INPUT' not found"
    exit 1
fi

echo "Input:      $INPUT"
echo "Output WebM: $OUTPUT_WEBM"
echo "Output MP4:  $OUTPUT_MP4"
echo ""

FILTER="[0:v]split[fwd][rev];[rev]reverse[revout];[fwd][revout]concat=n=2:v=1[loop];[loop]scale=iw/8:ih/8,hue=s=0,fps=15[out]"

echo "Encoding WebM (VP9)..."
ffmpeg -i "$INPUT" \
    -filter_complex "$FILTER" \
    -map "[out]" \
    -c:v libvpx-vp9 \
    -crf 50 \
    -b:v 0 \
    -an \
    "$OUTPUT_WEBM" \
    -y

echo "Encoding MP4 (H.264 — Safari)..."
ffmpeg -i "$INPUT" \
    -filter_complex "$FILTER" \
    -map "[out]" \
    -c:v libx264 \
    -crf 28 \
    -preset slow \
    -pix_fmt yuv420p \
    -movflags +faststart \
    -an \
    "$OUTPUT_MP4" \
    -y

echo ""
echo "WebM: $(du -sh "$OUTPUT_WEBM" | cut -f1)"
echo "MP4:  $(du -sh "$OUTPUT_MP4" | cut -f1)"

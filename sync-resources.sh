#!/bin/bash
# Script to sync gresource file from meson build to development locations

set -e

BUILDDIR="builddir/data/rustfy.gresource"
TARGET="data/rustfy.gresource"

if [ ! -f "$BUILDDIR" ]; then
    echo "Error: $BUILDDIR not found!"
    echo "Please run 'meson compile -C builddir' first."
    exit 1
fi

echo "Copying gresource file..."
cp "$BUILDDIR" "$TARGET"
echo "✓ Copied to $TARGET"

echo "Resources synced successfully!"

#!/usr/bin/env bash
set -e

VERSION="0.14.0-alpha.1"
TARGET="linux-x86_64" # Assuming linux x86_64 for this build environment
ARCHIVE_NAME="goat-$VERSION-$TARGET"

echo "Packaging GOAT Alpha $VERSION..."

# Run the build
./scripts/build-release.sh

echo "Creating archive $ARCHIVE_NAME.tar.gz..."
# Create a staging directory for the archive contents
mkdir -p "$ARCHIVE_NAME"
cp -r dist/* "$ARCHIVE_NAME/"

# Compress
tar -czvf "$ARCHIVE_NAME.tar.gz" "$ARCHIVE_NAME"

# Generate checksums
sha256sum "$ARCHIVE_NAME.tar.gz" > "$ARCHIVE_NAME.tar.gz.sha256"

# Cleanup staging
rm -rf "$ARCHIVE_NAME"

echo "Packaging complete!"
echo "Artifact: $ARCHIVE_NAME.tar.gz"
echo "Checksum: $ARCHIVE_NAME.tar.gz.sha256"

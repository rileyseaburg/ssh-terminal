#!/bin/bash
# Patch Xcode project to link zlib

PROJECT_FILE="gen/apple/ssh-terminal.xcodeproj/project.pbxproj"

if [ ! -f "$PROJECT_FILE" ]; then
    echo "Error: Project file not found at $PROJECT_FILE"
    exit 1
fi

echo "Patching Xcode project to link zlib..."

# Create a backup
cp "$PROJECT_FILE" "$PROJECT_FILE.bak"

# Add -lz to the linker flags by modifying the build settings
# Look for "libapp" and add "-lz" after it in the same line
python3 << 'EOF'
import re

with open('gen/apple/ssh-terminal.xcodeproj/project.pbxproj', 'r') as f:
    content = f.read()

# Pattern to find library references and add -lz after -lapp
# This adds zlib to the linked libraries
content = re.sub(
    r'(-lapp)',
    r'\1 -lz',
    content
)

# Also try to add to LIBRARY_SEARCH_PATHS if needed
# But primarily we need to link against libz.dylib

with open('gen/apple/ssh-terminal.xcodeproj/project.pbxproj', 'w') as f:
    f.write(content)

print("Patched project.pbxproj")
EOF

echo "Done patching Xcode project"

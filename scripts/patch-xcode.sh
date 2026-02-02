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

# Use Python to properly modify the Xcode project file
python3 << 'EOF'
import re

with open('gen/apple/ssh-terminal.xcodeproj/project.pbxproj', 'r') as f:
    content = f.read()

# Method 1: Add -lz after libraries in "libraries" sections
# Look for patterns like: libraries = (\n\t\t\t\t"-lapp",\n\t\t\t);
content = re.sub(
    r'(libraries\s*=\s*\([^)]*"-lapp")',
    r'\1,\n\t\t\t\t"-lz"',
    content
)

# Method 2: Add -lz to OTHER_LDFLAGS if present
content = re.sub(
    r'(OTHER_LDFLAGS\s*=\s*"[^"]*)"',
    r'\1 -lz"',
    content
)

# Method 3: Add libz.tbd framework reference
# Find a framework section and add libz after
content = re.sub(
    r'(frameworks\s*=\s*\([^)]*\\bSecurity\\b[^)]*\))',
    r'\1;\n\t\t\t\tlibraries = (\n\t\t\t\t\t"-lz",\n\t\t\t\t);',
    content
)

with open('gen/apple/ssh-terminal.xcodeproj/project.pbxproj', 'w') as f:
    f.write(content)

print("Patched project.pbxproj")

# Verify the patch was applied
with open('gen/apple/ssh-terminal.xcodeproj/project.pbxproj', 'r') as f:
    verify_content = f.read()
    if '-lz' in verify_content:
        print("SUCCESS: -lz found in project file")
    else:
        print("WARNING: -lz not found in project file")
EOF

echo "Done patching Xcode project"

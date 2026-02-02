#!/bin/bash
ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null rileyseaburg@192.168.50.248 << 'EOF'
echo "spike2"
EOF

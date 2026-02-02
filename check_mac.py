import paramiko
import time
import sys

# SSH connection details
hostname = "192.168.50.248"
username = "riley"
password = "riley"

# Create SSH client
client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

print("Connecting to Mac at 192.168.50.248...")
client.connect(hostname, username=username, password=password, timeout=30)
print("Connected!")

# Check GitHub Actions runner status
print("\n=== Checking GitHub Actions Runner Status ===")
stdin, stdout, stderr = client.exec_command("ps aux | grep -i 'actions-runner' | grep -v grep")
runner_processes = stdout.read().decode().strip()
if runner_processes:
    print("Runner processes found:")
    print(runner_processes)
else:
    print("No runner processes running")

# Check if runner directory exists
print("\n=== Checking Runner Installation ===")
stdin, stdout, stderr = client.exec_command("ls -la ~/actions-runner 2>/dev/null || echo 'Runner directory not found'")
runner_dir = stdout.read().decode().strip()
print(runner_dir)

# Try to find the runner service
print("\n=== Checking Runner Service ===")
stdin, stdout, stderr = client.exec_command("launchctl list | grep -i actions 2>/dev/null || echo 'No launchctl service found'")
service_status = stdout.read().decode().strip()
if service_status:
    print("Service status:")
    print(service_status)

# Check recent runner logs
print("\n=== Recent Runner Logs ===")
stdin, stdout, stderr = client.exec_command("find ~/actions-runner -name '*.log' -type f -mtime -1 2>/dev/null | head -5")
logs = stdout.read().decode().strip()
if logs:
    print("Recent log files:")
    print(logs)
    # Show last 20 lines of most recent log
    stdin, stdout, stderr = client.exec_command("find ~/actions-runner -name '*.log' -type f -mtime -1 -exec tail -20 {} \; 2>/dev/null | head -40")
    log_content = stdout.read().decode().strip()
    if log_content:
        print("\nLast log entries:")
        print(log_content)
else:
    print("No recent log files found")

# Check for iOS build artifacts
print("\n=== Checking for iOS Build Artifacts ===")
stdin, stdout, stderr = client.exec_command("find ~ -name '*.app' -path '*/ssh-terminal/*' -o -name '*.ipa' -path '*/ssh-terminal/*' 2>/dev/null | head -10")
artifacts = stdout.read().decode().strip()
if artifacts:
    print("iOS build artifacts found:")
    print(artifacts)
else:
    print("No iOS build artifacts found yet")

# Check if the runner needs to be started
print("\n=== Checking if runner needs to be started ===")
stdin, stdout, stderr = client.exec_command("ls ~/actions-runner/run.sh 2>/dev/null || echo 'run.sh not found'")
run_script = stdout.read().decode().strip()
if 'not found' not in run_script:
    print("Runner script found at:", run_script)
    print("\nTo start the runner, run:")
    print("cd ~/actions-runner && ./run.sh")
else:
    print("Runner not installed or in different location")

# Check installed apps on iPhone
print("\n=== Checking iPhone for installed apps ===")
stdin, stdout, stderr = client.exec_command("xcrun devicectl list devices 2>/dev/null || echo 'devicectl not available'")
devices = stdout.read().decode().strip()
if devices:
    print("Connected devices:")
    print(devices)
else:
    print("No devices found or devicectl not available")

print("\n=== Disconnecting ===")
client.close()
print("SSH connection closed")

$password = "spike2"
$securePassword = ConvertTo-SecureString $password -AsPlainText -Force
$credential = New-Object System.Management.Automation.PSCredential("riley", $securePassword)

# For SSH in PowerShell, we need to use sshpass or expect
# Since those aren't available, let's try ssh-keyscan first and then connect
ssh-keyscan -H 192.168.50.248 2>/dev/null | Out-File ~/.ssh/known_hosts -Append

# Use ssh with password via sshpass if available, otherwise manual
$sshCommand = "ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null riley@192.168.50.248"
Invoke-Expression $sshCommand

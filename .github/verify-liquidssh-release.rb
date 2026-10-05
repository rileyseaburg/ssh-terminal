# Read-only release audit on the authorized Mac runner. No credentials, private
# keys, provisioning contents, or device identifiers are printed or written.
require 'open3'
require 'rexml/document'
require 'base64'
require 'digest'
require 'json'
require 'time'
require 'yaml'

def command(*args, input: nil)
  output, error, status = Open3.capture3(*args, stdin_data: input)
  raise "Command failed: #{args.first}: #{error}" unless status.success?
  output
end

def require_fact(value, message)
  raise message unless value
end

def parse_element(element)
  case element.name
  when 'dict'
    element.elements.to_a.each_slice(2).to_h { |key, value| [key.text, parse_element(value)] }
  when 'array' then element.elements.map { |child| parse_element(child) }
  when 'data' then Base64.decode64(element.text.to_s)
  when 'true' then true
  when 'false' then false
  else element.text.to_s
  end
end

def plist(xml)
  parse_element(REXML::Document.new(xml).root.elements[1])
end

home = Dir.home
repo = File.join(home, 'actions-runner-ssh/_work/ssh-terminal/ssh-terminal')
run = File.join(home, 'LiquidSSH-distribution/20261005T133822Z')
nas = File.join(home, 'NAS/LiquidSSH/20261005T144031Z')
ipa = File.join(nas, 'LiquidSSH-1.0.0-build1-development.ipa')
exported = File.join(run, 'export-development-20261005T144031Z/LiquidSSH.ipa')
expected_sha256 = '18e131b8815c44a63197b40698ca4a1bce9c99e10648af3b872cf903ec0776ab'
origin = command('git', '-C', repo, 'remote', 'get-url', 'origin').strip
require_fact(origin.sub(/\.git\z/, '') == 'https://github.com/rileyseaburg/ssh-terminal', 'Wrong Mac repository origin')
project = File.join(repo, 'ios-native/LiquidSSH/project.yml')
built_project = File.join(run, 'source/project.yml')
original_project = File.join(run, 'source-before/project.yml')
require_fact(File.read(project) == File.read(built_project), 'Mac native project differs from release snapshot')
settings = YAML.load_file(project).fetch('settings').fetch('base')
require_fact(settings['MARKETING_VERSION'].to_s == '1.0.0' && settings['CURRENT_PROJECT_VERSION'].to_s == '1', 'Wrong native project version')
require_fact(settings['PRODUCT_BUNDLE_IDENTIFIER'] == 'com.sshterminal.liquid', 'Wrong native bundle')
require_fact(settings['DEVELOPMENT_TEAM'] == 'J9YRM3U37D', 'Wrong native team')
require_fact(File.file?(original_project), 'Original native snapshot missing')
source_count = Dir.glob(File.join(run, 'source/**/*.swift')).length
original_count = Dir.glob(File.join(run, 'source-before/**/*.swift')).length
require_fact(source_count > 0 && original_count == source_count, 'Missing preserved native Swift files')
mounts = command('mount')
require_fact(mounts.lines.any? { |line| line.include?(" on #{home}/NAS (smbfs") }, 'NAS is not mounted over SMB')
nas_hash = Digest::SHA256.file(ipa).hexdigest
export_hash = Digest::SHA256.file(exported).hexdigest
require_fact(nas_hash == expected_sha256 && export_hash == nas_hash, 'NAS/export checksum mismatch')
command('cmp', exported, ipa)
raw_info = command('unzip', '-p', ipa, 'Payload/LiquidSSH.app/Info.plist')
info = plist(command('plutil', '-convert', 'xml1', '-o', '-', '-', input: raw_info))
require_fact(info['CFBundleIdentifier'] == 'com.sshterminal.liquid', 'Wrong IPA bundle ID')
require_fact(info['CFBundleVersion'] == '1' && info['CFBundleShortVersionString'] == '1.0.0', 'Wrong IPA version')
raw_profile = command('unzip', '-p', ipa, 'Payload/LiquidSSH.app/embedded.mobileprovision')
embedded = plist(command('security', 'cms', '-D', input: raw_profile))
reference_path = File.join(home, 'Library/MobileDevice/Provisioning Profiles/e8d37d31-323b-4c24-971f-13f6204eb02a.mobileprovision')
reference = plist(command('security', 'cms', '-D', '-i', reference_path))
required_devices = reference.fetch('ProvisionedDevices')
actual_devices = embedded.fetch('ProvisionedDevices')
require_fact(!required_devices.empty? && (required_devices - actual_devices).empty?, 'Registered phone not covered by embedded profile')
require_fact(embedded['TeamIdentifier'] == ['J9YRM3U37D'], 'Wrong embedded team')
require_fact(Time.iso8601(embedded['ExpirationDate']) > Time.now, 'Expired embedded profile')
require_fact(embedded['Entitlements']['get-task-allow'] == true, 'Expected development profile')
app = File.join(run, 'inspect-20261005T144031Z/Payload/LiquidSSH.app')
command('codesign', '--verify', '--deep', '--strict', '--verbose=2', app)
puts JSON.pretty_generate(
  validation_level: 'focused CI-like; read-only Mac/NAS audit',
  host: command('hostname').strip, user: command('whoami').strip,
  mac_checkout: repo, mac_origin: origin,
  native_project: project, native_project_sha256: Digest::SHA256.file(project).hexdigest,
  original_project_sha256: Digest::SHA256.file(original_project).hexdigest,
  original_swift_file_count: original_count, release_swift_file_count: source_count,
  current_native_project_matches_release: true,
  preserved_source_snapshots_present: true, nas_smb_mount_confirmed: true,
  nas_ipa: ipa, exported_ipa: exported,
  nas_sha256: nas_hash, exported_sha256: export_hash, byte_for_byte_match: true,
  bundle_id: info['CFBundleIdentifier'], version: info['CFBundleShortVersionString'],
  build: info['CFBundleVersion'], minimum_ios: info['MinimumOSVersion'],
  team: embedded['TeamIdentifier'], profile_uuid: embedded['UUID'],
  profile_expires: embedded['ExpirationDate'], profile_device_count: actual_devices.length,
  reference_profile_name: reference['Name'], reference_device_count: required_devices.length,
  registered_phone_covered: true, strict_signature_check: 'exit 0',
  physical_installation: 'not-run', device_identifiers: 'not emitted'
)
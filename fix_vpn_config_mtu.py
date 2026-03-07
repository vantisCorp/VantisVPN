import re

# Read the file
with open('src/core/hardware/comprehensive_tests.rs', 'r') as f:
    content = f.read()

# Pattern to find mtu: 1500 within VpnRouterConfig blocks
# We need to be careful to only remove mtu from VpnRouterConfig, not WanConfig

# First, let's identify and fix all VpnRouterConfig blocks
# The pattern is: VpnRouterConfig { ... mtu: 1500 ... }

def fix_vpn_router_config(match):
    block = match.group(1)
    # Remove mtu: 1500 from the block
    block = re.sub(r',?\s*mtu:\s*1500\s*,?\s*', '', block)
    # Clean up any double commas or trailing commas
    block = re.sub(r',\s*,', ',', block)
    return f'VpnRouterConfig {{ {block} }}'

# Find all VpnRouterConfig blocks and remove mtu from them
# We'll do this by finding VpnRouterConfig { ... } and processing the content
pattern = r'VpnRouterConfig\s*\{([^}]+)\}'
matches = list(re.finditer(pattern, content))

# Process matches in reverse order to preserve positions
for match in reversed(matches):
    block_content = match.group(1)
    # Remove mtu: 1500 from this block
    new_content = re.sub(r',?\s*mtu:\s*1500\s*,?\s*', '', block_content)
    # Replace the block
    start = match.start()
    end = match.end()
    content = content[:start] + f'VpnRouterConfig {{{new_content}}}' + content[end:]

# Write the file back
with open('src/core/hardware/comprehensive_tests.rs', 'w') as f:
    f.write(content)

print("Fixed VpnRouterConfig mtu fields")
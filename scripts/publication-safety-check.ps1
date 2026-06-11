$ErrorActionPreference = "Stop"

$blockedPathPatterns = @(
  '\.vhdx?$',
  '\.avhdx$',
  '(^|[\\/])backups([\\/]|$)',
  '(^|[\\/])evidence([\\/]|$)',
  '(^|[\\/])refs([\\/]|$)',
  '(^|[\\/])state([\\/]|$)',
  '(^|[\\/])wallet([\\/]|$)',
  'wallet\.db$',
  'drk-testnet\.toml$',
  'darkfid-testnet\.toml$',
  '\.tx$',
  '\.call$',
  '\.calls-map$',
  'HCS_.*_RAW.*\.txt$',
  'WSL_STORAGE_FILE_SEARCH_RAW\.txt$'
)

$blockedContentPatterns = @(
  ('github' + '_pat_'),
  ('gh' + 'p_'),
  'BEGIN (RSA|OPENSSH|PRIVATE) KEY',
  'seed phrase\s*[:=]',
  'wallet_pass\s*=',
  'password\s*=',
  'PRIVATE_KEY\s*=',
  'ADMIN_SECRET\s*=',
  'ADDRESS_HASH_SECRET\s*=',
  'IP_HASH_SECRET\s*=',
  'Secret Key\s*[:=]'
)

function Get-GitFiles {
  $repoRoot = (Get-Location).Path -replace '\\', '/'
  $files = git -c "safe.directory=$repoRoot" ls-files
  if ($LASTEXITCODE -ne 0) {
    throw "git ls-files failed"
  }
  return $files
}

$trackedFiles = @(Get-GitFiles)
$failures = New-Object System.Collections.Generic.List[string]

foreach ($file in $trackedFiles) {
  $normalized = $file -replace '\\', '/'
  foreach ($pattern in $blockedPathPatterns) {
    if ($normalized -match $pattern) {
      $failures.Add("Blocked tracked path: $file")
      break
    }
  }
}

$textFiles = $trackedFiles | Where-Object {
  $_ -notmatch '\.(png|jpg|jpeg|gif|webp|wasm|zip|gz|tar|7z|pdf|exe|dll|node|vhdx?)$'
}

foreach ($file in $textFiles) {
  if (-not (Test-Path -LiteralPath $file)) {
    continue
  }

  $content = Get-Content -LiteralPath $file -Raw -ErrorAction SilentlyContinue
  if ($null -eq $content) {
    continue
  }

  foreach ($pattern in $blockedContentPatterns) {
    if ($content -match $pattern) {
      $failures.Add("Sensitive-looking content pattern '$pattern' in tracked file: $file")
    }
  }
}

if ($failures.Count -gt 0) {
  Write-Host "Publication safety check failed:"
  $failures | ForEach-Object { Write-Host "- $_" }
  exit 1
}

Write-Host "Publication safety check passed."

param(
    [string]$ProjectPath = "C:\Users\pbess\Desktop\qr-studio-ultra-upload",
    [string]$OutputFile = "qr-studio-ultra-project-scan.txt"
)

$ErrorActionPreference = "Continue"

$ExcludedDirs = @(
    "node_modules",
    "target",
    ".git",
    ".svelte-kit",
    "dist",
    "build",
    ".tauri",
    ".vscode",
    ".idea"
)

$ImportantFiles = @(
    "package.json",
    "vite.config.js",
    "vite.config.ts",
    "svelte.config.js",
    "svelte.config.ts",
    "tsconfig.json",
    "src-tauri\Cargo.toml",
    "src-tauri\tauri.conf.json",
    "src-tauri\capabilities\default.json",
    "src-tauri\src\main.rs",
    "src-tauri\src\lib.rs",
    "README.md"
)

if (!(Test-Path $ProjectPath)) {
    Write-Host "Project path not found: $ProjectPath" -ForegroundColor Red
    exit 1
}

$OutputPath = Join-Path $ProjectPath $OutputFile

function Write-Section {
    param([string]$Title)

    Add-Content $OutputPath ""
    Add-Content $OutputPath "============================================================"
    Add-Content $OutputPath $Title
    Add-Content $OutputPath "============================================================"
    Add-Content $OutputPath ""
}

function Should-ExcludePath {
    param([string]$FullPath)

    foreach ($dir in $ExcludedDirs) {
        if ($FullPath -match "\\$([regex]::Escape($dir))(\\|$)") {
            return $true
        }
    }

    return $false
}

Remove-Item $OutputPath -ErrorAction SilentlyContinue

Add-Content $OutputPath "QR Studio Ultra Project Scan"
Add-Content $OutputPath "Generated: $(Get-Date)"
Add-Content $OutputPath "Project Path: $ProjectPath"
Add-Content $OutputPath ""

Write-Section "SYSTEM INFO"

Add-Content $OutputPath "PowerShell Version:"
Add-Content $OutputPath $PSVersionTable.PSVersion.ToString()
Add-Content $OutputPath ""

Add-Content $OutputPath "Node Version:"
try { Add-Content $OutputPath (node --version 2>&1) } catch { Add-Content $OutputPath "Node not found" }

Add-Content $OutputPath ""
Add-Content $OutputPath "NPM Version:"
try { Add-Content $OutputPath (npm --version 2>&1) } catch { Add-Content $OutputPath "NPM not found" }

Add-Content $OutputPath ""
Add-Content $OutputPath "Rust Version:"
try { Add-Content $OutputPath (rustc --version 2>&1) } catch { Add-Content $OutputPath "Rust not found" }

Add-Content $OutputPath ""
Add-Content $OutputPath "Cargo Version:"
try { Add-Content $OutputPath (cargo --version 2>&1) } catch { Add-Content $OutputPath "Cargo not found" }

Write-Section "TOP-LEVEL DIRECTORY"

Get-ChildItem -Path $ProjectPath -Force |
    Sort-Object PSIsContainer, Name |
    ForEach-Object {
        if ($_.PSIsContainer) {
            Add-Content $OutputPath "[DIR]  $($_.Name)"
        } else {
            Add-Content $OutputPath "[FILE] $($_.Name)"
        }
    }

Write-Section "PROJECT TREE"

Get-ChildItem -Path $ProjectPath -Recurse -Force |
    Where-Object {
        -not (Should-ExcludePath $_.FullName)
    } |
    Sort-Object FullName |
    ForEach-Object {
        $relative = $_.FullName.Substring($ProjectPath.Length).TrimStart("\")
        $depth = ($relative -split "\\").Count - 1
        $indent = "  " * $depth

        if ($_.PSIsContainer) {
            Add-Content $OutputPath "$indent[DIR]  $($_.Name)"
        } else {
            Add-Content $OutputPath "$indent[FILE] $($_.Name)"
        }
    }

Write-Section "IMPORTANT FILE CONTENTS"

foreach ($file in $ImportantFiles) {
    $fullPath = Join-Path $ProjectPath $file

    Add-Content $OutputPath ""
    Add-Content $OutputPath "------------------------------------------------------------"
    Add-Content $OutputPath $file
    Add-Content $OutputPath "------------------------------------------------------------"

    if (Test-Path $fullPath) {
        try {
            Get-Content $fullPath -Raw | Add-Content $OutputPath
        } catch {
            Add-Content $OutputPath "Could not read file."
        }
    } else {
        Add-Content $OutputPath "File not found."
    }
}

Write-Section "PACKAGE SCRIPTS"

$packageJson = Join-Path $ProjectPath "package.json"

if (Test-Path $packageJson) {
    try {
        $package = Get-Content $packageJson -Raw | ConvertFrom-Json

        if ($package.scripts) {
            $package.scripts.PSObject.Properties | ForEach-Object {
                Add-Content $OutputPath "$($_.Name): $($_.Value)"
            }
        } else {
            Add-Content $OutputPath "No scripts section found."
        }
    } catch {
        Add-Content $OutputPath "Could not parse package.json."
    }
} else {
    Add-Content $OutputPath "package.json not found."
}

Write-Section "TAURI BUILD OUTPUT CHECK"

$releasePath = Join-Path $ProjectPath "src-tauri\target\release"
$bundlePath = Join-Path $ProjectPath "src-tauri\target\release\bundle"

Add-Content $OutputPath "Release path exists: $(Test-Path $releasePath)"
Add-Content $OutputPath "Bundle path exists:  $(Test-Path $bundlePath)"

if (Test-Path $bundlePath) {
    Get-ChildItem $bundlePath -Recurse -File |
        ForEach-Object {
            Add-Content $OutputPath $_.FullName
        }
}

Write-Host ""
Write-Host "Project scan complete." -ForegroundColor Green
Write-Host "Saved to:" -ForegroundColor Cyan
Write-Host $OutputPath
Write-Host ""
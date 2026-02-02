#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

Write-Host "=== SSH Terminal Build Script ===" -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
Write-Host "Checking prerequisites..." -ForegroundColor Yellow

$hasRust = Get-Command rustc -ErrorAction SilentlyContinue
$hasCargo = Get-Command cargo -ErrorAction SilentlyContinue
$hasNode = Get-Command node -ErrorAction SilentlyContinue

if (-not $hasRust) {
    Write-Host "Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

if (-not $hasCargo) {
    Write-Host "Cargo not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

if (-not $hasNode) {
    Write-Host "Node.js not found. Please install from https://nodejs.org/" -ForegroundColor Red
    exit 1
}

Write-Host "Prerequisites check passed!" -ForegroundColor Green
Write-Host ""

# Parse arguments
$BuildType = "debug"
$Target = ""

for ($i = 0; $i -lt $args.Length; $i++) {
    switch ($args[$i]) {
        "--release" { $BuildType = "release" }
        "--target" { 
            $i++
            $Target = $args[$i] 
        }
        "--help" {
            Write-Host "Usage: .\build.ps1 [OPTIONS]"
            Write-Host ""
            Write-Host "Options:"
            Write-Host "  --release       Build release version"
            Write-Host "  --target        Specify target triple (e.g., x86_64-pc-windows-msvc)"
            Write-Host "  --help          Show this help message"
            Write-Host ""
            exit 0
        }
        default {
            Write-Host "Unknown option: $($args[$i])" -ForegroundColor Red
            exit 1
        }
    }
}

# Check for Visual Studio Build Tools on Windows
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    Write-Host "Windows detected. Checking for Visual Studio Build Tools..." -ForegroundColor Yellow
    
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vsWhere) {
        $vsPath = & $vsWhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if ($vsPath) {
            Write-Host "Visual Studio Build Tools found at: $vsPath" -ForegroundColor Green
        } else {
            Write-Host "Visual Studio Build Tools with C++ workload not found!" -ForegroundColor Red
            Write-Host "Please install from: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
            Write-Host "Required: Desktop development with C++ workload" -ForegroundColor Yellow
        }
    } else {
        Write-Host "Visual Studio Build Tools not found!" -ForegroundColor Red
        Write-Host "Please install from: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
    }
}

Write-Host ""

# Build
Set-Location src-tauri

if ($BuildType -eq "release") {
    Write-Host "Building release version..." -ForegroundColor Green
    if ($Target) {
        cargo build --release --target $Target
    } else {
        cargo build --release
    }
    Write-Host ""
    Write-Host "Build complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Binary location:"
    if ($Target) {
        Write-Host "  target\$Target\release\ssh-terminal.exe"
    } else {
        Write-Host "  target\release\ssh-terminal.exe"
    }
} else {
    Write-Host "Building debug version..." -ForegroundColor Yellow
    if ($Target) {
        cargo build --target $Target
    } else {
        cargo build
    }
    Write-Host ""
    Write-Host "Build complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "To run the application:"
    if ($Target) {
        Write-Host "  .\target\$Target\debug\ssh-terminal.exe"
    } else {
        Write-Host "  .\target\debug\ssh-terminal.exe"
    }
}

Write-Host ""
Write-Host "To build for distribution, run:"
Write-Host "  cargo tauri build"
Write-Host ""

Set-Location ..

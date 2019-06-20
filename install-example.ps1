cargo build --example main

$manifest = Join-Path $PSScriptRoot "AppxManifest.xml"

Add-AppxPackage -Register $manifest

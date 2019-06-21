Param(
    [String] $Example = 'main'
)

Push-Location $PSScriptRoot

cargo build --example $Example

$name = $Example
$capital_name = (Get-Culture).TextInfo.ToTitleCase($name)
$display_name = "Rust WinRT Core App Example ($capital_name)"
$ident_name = "RustWinRTCoreAppExample$capital_name"

$manifest = @"
<?xml version="1.0" encoding="utf-16"?>
<Package
        xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
        xmlns:mp="http://schemas.microsoft.com/appx/2014/phone/manifest"
        xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
        xmlns:uap5="http://schemas.microsoft.com/appx/manifest/uap/windows10/5"
        xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
        IgnorableNamespaces="uap mp">
    <Identity
            Name="$ident_name"
            Publisher="CN=kai-p"
            Version="1.0.0.0" />
    <Properties>
        <DisplayName>$display_name</DisplayName>
        <PublisherDisplayName>thiskaiguy</PublisherDisplayName>
        <Logo>assets\StoreLogo.png</Logo>
    </Properties>
    <Dependencies>
        <TargetDeviceFamily Name="Windows.Universal" MinVersion="10.0.0.0" MaxVersionTested="10.0.16266.0" />
    </Dependencies>
    <Resources>
        <Resource Language="en-gb" />
    </Resources>
    <Applications>
        <Application Id="App" Executable="target\debug\examples\$name.exe" EntryPoint="App">
            <uap:VisualElements DisplayName="$display_name" Description="A minimal Universal Windows Platform (UWP) app example in rust"
                                Square150x150Logo="assets\Logo.png" Square44x44Logo="assets\SmallLogo.png" BackgroundColor="#464646">
                <uap:SplashScreen Image="assets\SplashScreen.png" />
            </uap:VisualElements>
            <Extensions>
                <uap5:Extension
                        Category="windows.appExecutionAlias"
                        Executable="target\debug\examples\$name.exe"
                        EntryPoint="App">
                    <uap5:AppExecutionAlias>
                        <uap5:ExecutionAlias Alias="winrt-core-app-example-$name.exe" />
                    </uap5:AppExecutionAlias>
                </uap5:Extension>
            </Extensions>
        </Application>
    </Applications>
    <Capabilities>
        <rescap:Capability Name="runFullTrust"/>
        <rescap:Capability Name="broadFileSystemAccess" />
    </Capabilities>
</Package>
"@

$manifest_path = ".\AppxManifest.xml"
$manifest | Out-File $manifest_path

Add-AppxPackage -Register $manifest_path

Remove-Item $manifest_path
Pop-Location

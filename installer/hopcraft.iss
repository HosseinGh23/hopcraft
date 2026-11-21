; HopCraft — Modern Windows Installer (Inno Setup 6)
#ifndef AppVersion
  #define AppVersion "1.2.1"
#endif
#ifndef AppArch
  #define AppArch "x64"
#endif
#ifndef BinaryPath
  #define BinaryPath "..\src-tauri\target\x86_64-pc-windows-msvc\release\HopCraft.exe"
#endif
#ifndef OutputDir
  #define OutputDir "..\dist"
#endif
#ifndef OutputBaseFilename
  #define OutputBaseFilename "HopCraft-windows-x64-setup"
#endif

[Setup]
AppId={{8D6B2A1E-9C4F-4E7A-B1C3-2F5A8E9D0C14}
AppName=HopCraft
AppVersion={#AppVersion}
AppVerName=HopCraft {#AppVersion}
AppPublisher=HosseinGh23
AppPublisherURL=https://github.com/HosseinGh23/hopcraft
AppSupportURL=https://github.com/HosseinGh23/hopcraft/issues
AppUpdatesURL=https://github.com/HosseinGh23/hopcraft/releases
DefaultDirName={autopf}\HopCraft
DefaultGroupName=HopCraft
DisableProgramGroupPage=yes
AllowNoIcons=yes
LicenseFile=..\LICENSE
SetupIconFile=..\assets\logo.ico
UninstallDisplayIcon={app}\HopCraft.exe
UninstallDisplayName=HopCraft
OutputDir={#OutputDir}
OutputBaseFilename={#OutputBaseFilename}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
WizardResizable=no
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
CloseApplications=yes
RestartApplications=yes
#if AppArch == "x64"
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
#elif AppArch == "arm64"
ArchitecturesAllowed=arm64
ArchitecturesInstallIn64BitMode=arm64
#endif

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#BinaryPath}"; DestDir: "{app}"; DestName: "HopCraft.exe"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\HopCraft"; Filename: "{app}\HopCraft.exe"
Name: "{autodesktop}\HopCraft"; Filename: "{app}\HopCraft.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\HopCraft.exe"; Description: "{cm:LaunchProgram,HopCraft}"; Flags: nowait postinstall skipifsilent

# prex

Run Windows executables in Steam Proton environments.

### List all installed games
```sh
prex ls
1245620 ELDEN RING      default
1888160 ARMORED CORE VI FIRES OF RUBICON      proton_experimental
2050650 Resident Evil 4 default
2622380 ELDEN RING NIGHTREIGN   default
```

### List all users
```sh
prex users
59710912        Dark Empath Amogus
1516607315      zilfeejel
```

### List currently running games
```sh
prex ps
1245620 ELDEN RING
```

### Run an installer
```sh
prex run 2050650 ~/Downloads/Fluffy\ Mod\ Manager-818-3-068-1765672670/Modmanager.exe
```

### Reuse a running app (single-instance apps)
```sh
prex run --single-instance 489830 "/games/steam/steamapps/compatdata/489830/pfx/drive_c/Program Files/Black Tree Gaming Ltd/Vortex/Vortex.exe" --download nxm://example
```

### NXM handling (mod manager registration)
Register a mod manager for a game (the exe must live inside that game's prefix):
```sh
prex mm add 489830 "/games/steam/steamapps/compatdata/489830/pfx/drive_c/Program Files/Black Tree Gaming Ltd/Vortex/Vortex.exe"
prex mm ls
489830  The Elder Scrolls V: Skyrim Special Edition  inactive  /games/steam/steamapps/compatdata/489830/pfx/drive_c/Program Files/Black Tree Gaming Ltd/Vortex/Vortex.exe
prex mm set-active 489830
```

Remove a mod manager registration:
```sh
prex mm remove 489830
```

Handle NXM links with the active mod manager:
```sh
prex nxm "nxm://skyrimspecialedition/mods/12345"
```

Example desktop entry:
```ini
[Desktop Entry]
Name=Prex NXM Handler
Comment=Handle NXM links for active mod manager
Type=Application
NoDisplay=true
MimeType=x-scheme-handler/nxm;
Exec=prex nxm %u
```

### Print a game's Proton prefix path (pfx)
```sh
prex path 2050650
/games/steam/steamapps/compatdata/2050650/pfx
cd "$(prex path 2050650)"
```

### Start something alongside a running game
```sh
prex attach 1245620 "/games/steam/steamapps/compatdata/1245620/prex/drive_c/Program Files/Cheat Engine/Cheat Engine.exe"
```

#### Hack to bypass gamescope
```sh
prex attach --bypass-gamescope 1245620 "/games/steam/steamapps/compatdata/1245620/prex/drive_c/Program Files/Cheat Engine/Cheat Engine.exe"
```

### Launch another executable while preserving Steam launch options

Keeps any launch options you set in Steam, for example:
```sh
LD_PRELOAD= gamescope -f -H 1440 -h 1440 -r 75 --mangoapp -- env LD_PRELOAD="$LD_PRELOAD" gamemoderun %command%
```

#### Seamless Co-op for Elden Ring example
```sh
prex launch 1245620 --user-id 59710912 "Game/ersc_launcher.exe" 
```

### Shell completions

Dynamic completions are generated at shell runtime. These examples source the generated script on startup:

**Bash**
```sh
echo 'source <(COMPLETE=bash prex)' >> ~/.bashrc
```

**Zsh**
```sh
echo 'source <(COMPLETE=zsh prex)' >> ~/.zshrc
```
Supported shells: bash, zsh, fish, elvish, (powershell ಠ_ಠ).

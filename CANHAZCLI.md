# Let's find some options

We already know about some options from spying on `Preprocess.log` and `Launcher.log`:
```
Stage 2 Updating:
"C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -silent -log:/Preprocess.log -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -applet:/EE/Types/Framework/ContentUpdate
Repair game content:
"C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -silent -log:/Repair.log -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -applet:/EE/Types/Framework/CacheRepair
Running the game:
"C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -fullscreen:0
```

But, what else can we find out? Well... from running the game, not a whole lot. `-help`, `--help`, and `/?` all result in the same thing:
```
[...]
2.631 Sys [Error]: Error
2.631 Sys [Error]: Please run Warframe from the Launcher.
[...]
```

But, who says we have to play by the game's rules?

## hallo strings
`strings` is a utility that finds and spits out any string in a binary file. It's very helpful. So:

`strings [...]/Warframe/Downloaded/Public/Warframe.exe > strings.txt`

## Applet
The first thing that interests me is the `-applet` in the update and repair commands. Can we get anywhere with that?
```
$ rg '/EE/Types/Framework' strings.txt
164604:/EE/Types/Framework/ProceduralLevel
164605:/EE/Types/Framework/JsonProceduralLevel
165377:</EE/Types/Framework/RegionMgrImpl
165566:/EE/Types/Framework/ContextImpl
167927:/EE/Types/Framework/UIRegionMgrImpl
172284:/EE/Types/Framework/
172627:/EE/Types/Framework/DemoFrameworkImpl
173158:/EE/Types/Framework/ClientImpl
```
... no. not really.

But, what if we approach this from the other direction?

```
$ rg 'ContentUpdate' strings.txt
172836:ContentUpdateApplet
$ rg 'Applet' strings.txt
150213:Applet
152235:AppletName
152237:AppletArgs
152587:ScriptApplet
165925:GameApplet
172596:CacheRepairApplet
172832:CrashApplet
172836:ContentUpdateApplet
173087:CacheCfgApplet
173092:CacheCfgApplet unrecognized platform:
173094:CacheCleanerApplet
173097:CacheDefraggerApplet
173108:CacheDefraggerAsyncApplet
182160:DedicatedServerApplet
182165:DedicatedServerApplet::PreInitialize -- not enough arguments
```

Heeeey! Here we go. If we exclude the things that pretty obviously aren't applets, we get 9: `ScriptApplet`, `GameApplet`, `CacheRepairApplet`, `CrashApplet`, `ContentUpdateApplet`, `CacheCfgApplet`, `CacheCleanerApplet`, `CacheDefraggerApplet`, and `DedicatedServerApplet`. We already know what three of them do (`CacheRepair`, `CacheDefragger`, and `ContentUpdate`), so let's look at the others.

All of them fail with the same error:
```
[...]
2.477 Sys [Error]: Could not find game rules: /EE/Types/GameRules/MultiplayerGameRules
2.477 Sys [Error]: Required by game config /EE/Types/GameRules/GameConfig
2.477 Sys [Error]: Failed to create run-time type: MultiplayerGameRules (parent GameRules is abstract)
2.477 Sys [Error]: Could not create /EE/Types/GameRules/MultiplayerGameRules as a GameRules
[...]
```
grr. Dead end then. Let's go back to strings.

# Options
```
$ rg '^-[a-z]' strings.txt
149943:-limitcpu
149944:-config:/Configs/System/BasicSmoke
149945:-config:/Configs/System/ServerSmoke
149946:-config:/Configs/System/CompileAllSmoke
149947:-client
149948:-applet
149949:-server
150146:-silent
150147:-allowmultiple
150148:-log
150149:-config
150150:-nop
150151:-dx10
150152:-dx11
150153:-fullscreen
150154:-threadedworker
150155:-language
150156:-cluster
150157:-relaunch
150158:-debugsession
150159:-onlive
150160:-inputrec
150162:-server and -client:address are mutually exclusive; -server supersedes.
150244:-nop
150245:-silent
150562:-editor
150563:-dll
150564:-censored
150565:-placeable
150566:-fragile
150567:-volatile
150568:-save
150569:-lazy-loader
150570:-ephemeral
150571:-flyweight
150573:-perception
150574:-recycle
166382:-partner
172249:-editor
182166:-missionNode:
193895:-core
```

`-server` appears to do, uh, something. It listens on a local UDP port, but I can't get a client to connect to it.

`-onlive` seems to be silently ignored.

I haven't tested any of the other flags yet.

# cmd_alias
Simple alias implementation (?) for windows using doskey

## Usage
List available alias
```bash
alias
```
Set an alias
```bash
alias set key=value
```
Remove an alias
```bash
alias del key
```
Reload alias
```bash
alias load
```

## Build
Not much to say, just build it, I guess.
```
git clone einjerjar/cmd_alias
cd cmd_alias
cargo build --release
```

## Installation
* After building, expose the release file (`/target/release/alias.exe`) to path (either copy it to an existing exposed folder or add the build folder to path (not a good idea)).
* Open Regedit, go to `Computer\HKEY_CURRENT_USER\SOFTWARE\Microsoft\Command Processor`, add a new `String Value` with the name `AutoRun` and the value "`%AppData%\cmd_alias\aliases.cmd`".

> ![image](images/regedit.png)

* Run the file at least once to generate the alias file (there will likely be an error on the first run, you can ignore it).
* Profit.

### Installation without Administrator rights
* Figure out a way to add the alias file to the cmd/ps startup (mb, too lazy to see if possible).
* Profit.

### Wanna support me stuff? :3
<a href='https://ko-fi.com/X8X831J1L' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://cdn.ko-fi.com/cdn/kofi1.png?v=2' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>
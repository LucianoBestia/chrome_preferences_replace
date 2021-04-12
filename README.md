# chrome_preferences_replace

It is annoying that Chrome shows the "Restore" dialog after it crashes for any reason.  
Chrome on start reads the file `preferences` and search for `"exit_type":"Crashed","exited_cleanly":false,`  
The file is in the `profile` directory that Chrome uses.  
The profile directory can be changed when starting chrome like this:  
`c:\Program Files\Google\Chrome\Application\chrome.exe --profile-directory="profile name"`  
Usually it is in the Users folder like this:  
`c:\Users\Luciano\AppData\Local\Google\Chrome\User Data\Default\Preferences`

I will change this file prior to open Chrome to contain this:  
`"exit_type":"Normal","exited_cleanly":true,`  
So it will avoid to show the dreadful "Restore" dialog.  

## run from win10 command prompt

The first and only argument is the path to the Preferences file.  
The program is compiled in WSL2 and it is a Linux program.  
It will work only on computers that have WSL2 enabled with a Linux OS.  
It is simple to run this little program from Win10 command prompt:  
`wsl ~/rustprojects/chrome_preferences_replace/target/release/chrome_preferences_replace "/mnt/c/Users/Luciano/AppData/Local/Google/Chrome/User Data/Default/Preferences"`  

## development

Cargo-make is a utility to write simple "scripts" to use in development.  
I use it to store in one place all the commands that I frequently use in development.  
<https://github.com/sagiegurari/cargo-make>  

## for my Thinkpad computer

I enabled wsl2 and installed Debian.  
After copying the chrome_preferences_replace file, make it executable in Linux
`sudo chmod +x chrome_preferences_replace`

put this action in the win10 scheduler on logon:

`wsl ~/chrome_preferences_replace/chrome_preferences_replace "/mnt/c/Users/happy guest/Documents/ChromeProfiles/Profile1/Default/Preferences"`  

And after the first trial I crashed the wsl2 somehow. Now in `cmd` when I write `wsl` I get the error `The system cannot find the path specified.`  
I didn't do anything special, just started the `wsl` on logon from the Scheduler. It looks like there is no way to recover. I must unregister and install Debian again. What a disappointment.  
And so Microsoft gently pushes people to write a Win exe instead of a linux exe. Pretty standard for MS.  

## building a Win exe

On my development machine Win10 I installed rustup from <https://www.rust-lang.org/tools/install>. That installation was painless. A little slow on the rust-docs part (interestingly in Linux this part is super quick), but all in all not bad.  
But I need also the `MSVC C++ Build tools` from MS. That was a pain. It is downloading 1.6Gb of something. What is so big? It installed over 4GB of something? The installation lasted forever.  
First I downloaded `Build Tools for Visual Studio 2019` from <https://visualstudio.microsoft.com/downloads/> under `Tools for Visual Studio 2019`.
It is something like `vs_buildtools__1867578767.1618229149.exe`.  
Run it and select `C++ Tools`. While in the C++ Tools you have to select "Windows 10 SDK" as well. There will be multiple options, go with the highest version number.  
![C++ build tools](https://github.com/LucianoBestia/chrome_preferences_replace/raw/main/img/2020_04_19_add_sdk.png)

I then used `PowerShell` to work with Rust, because it can work with UNC paths. The command prompt cannot use the `\\wsl$\Debian` path as active directory.  
`cd \\wsl$\Debian\home\luciano\rustprojects\chrome_preferences_replace\`  
and then `cargo build --release`.  
Now I have the windows exe in the `target/release` folder.  
All from the same rust source code. Nice.  

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
It is simple to run this little program from Win10:  
`wsl ~/rustprojects/chrome_preferences_replace/target/release/chrome_preferences_replace "/mnt/c/Users/Luciano/AppData/Local/Google/Chrome/User Data/Default/Preferences"`  

## development

Cargo-make is a utility to write simple "scripts" to use in development.  
I use it to store in one place all the commands that I frequently use in development.  
<https://github.com/sagiegurari/cargo-make>  

# AnyFile
An easier way to store and reference asset files from your public repository. 
  
### Introducing a new look and feel. 

<img width="60%" alt="images/anyfile_1.0.4_view_yxCv8o9pxc1oiX9u.png" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/anyfile_1.0.4_view_yxCv8o9pxc1oiX9u.png">. 


Drag and Drop any file into the AnyFile app to upload to your GitHub repository  

<img width="60%" alt="images/anyfile_v1.0.4_dd_mk64EsNZaQht6yFE.gif" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/anyfile_v1.0.4_dd_mk64EsNZaQht6yFE.gif">  

## About
I needed an easier method to store screenshots and .gifs for github docs: readme, issues, discussions, etc...

I was tired of opening the github web page to upload a new screenshot so I created this little app.  Dragging files onto the app will 
upload the file to your selected repository. You can view the file in the web, click on the file to obtain an http image tag, and can remove files from your repository. 

This is a small simple app that I use often to save time, so I decided to share it.  

Anyfile is writtern purely in Rust!  Egui is the frontend library.

## Screenshots
Generate an http image tag by clicking on the file:  
<img width="60%" alt="images/anyfile_v1.0.4_image_tag_ObOArkWUNbbCWmqV.gif" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/anyfile_v1.0.4_image_tag_ObOArkWUNbbCWmqV.gif">  
  
Remove files:  
<img width="60%" alt="images/Screen Recording 2022-05-01 at 8.35.53 AM_xAOBcchGccgzEPk9.gif" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/Screen Recording 2022-05-01 at 8.35.53 AM_xAOBcchGccgzEPk9.gif">
  
## Installation
#### Hombebrew
```
brew tap mjehrhart/anyfile
brew install mjehrhart/anyfile/anyfile
```

#### Upgrade
```
brew update
brew upgrade mjehrhart/anyfile/anyfile
```
or

```
brew uninstall mjehrhart/anyfile/anyfile
brew install mjehrhart/anyfile/anyfile
``` 

If you have any question, comments, or concerns please visit here:  
https://github.com/mjehrhart/anyfile/discussions/ 


This app is free to use and is a work in progress. No information is collected at all (it is open source after all)

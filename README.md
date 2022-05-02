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
Generate an http image tag by clicking on the file name:  
<img width="60%" alt="images/anyfile_v1.0.4_image_tag_ObOArkWUNbbCWmqV.gif" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/anyfile_v1.0.4_image_tag_ObOArkWUNbbCWmqV.gif">  
  
Double click to remove files:  
<img width="60%" alt="images/Screen Recording 2022-05-01 at 8.35.53 AM_xAOBcchGccgzEPk9.gif" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/Screen Recording 2022-05-01 at 8.35.53 AM_xAOBcchGccgzEPk9.gif">  
  
View files:  
<img width="60%" alt="images/anyfile_1.0.4_view_image_MdnYgEH0e2xnru3z.gif" src="https://raw.githubusercontent.com/mjehrhart/assets/main/images/anyfile_1.0.4_view_image_MdnYgEH0e2xnru3z.gif">  

## Installation
#### Hombebrew
```
brew tap mjehrhart/anyfile
brew install mjehrhart/anyfile/anyfile
```

#### Upgrade
```
brew update
brew upgrade anyfile
```
or

```
brew uninstall anyfile
brew install anyfile
``` 

## Config
When the app run for the first time, it will look for and if not found, the app will create an empty config file located in your home directory ~/.anyfile/config.json.

```json 
{ "author" : "", "auth_token": "", "email": "", "username": "", }
```

The username should be the same as your GitHub username. The auth_token is a GitHub personal auth token. And email and author can be whatever you want for the official commit.

This is a basic setup for the config, something I did quickly to get the project going. I'd be happy to hear more ways to handle the config file.

If you have any question, comments, or concerns please visit here:  
https://github.com/mjehrhart/anyfile/discussions/ 


This app is free to use and is a work in progress. No information is collected at all (it is open source after all)

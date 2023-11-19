
<div align="center">

![tmplt logo](https://github.com/youssefhmidi/tmplt/blob/main/.assets/1.png)

![Static Badge](https://img.shields.io/badge/version-1.0v-858522)
![Static Badge](https://img.shields.io/badge/lang-rust-orange)
![Static Badge](https://img.shields.io/badge/license-MIT-red)

</div>

> Side note: this is a side-project so I can learn rust even more
> I just want to share it for others who can use it 
> yes I'm a beginner so I would appreciate any feedbacks about the code 

### templating language for creating folders structures

tmplt, a templating language (also a scripting language) for creating a reusable files that can be used to create a folders structures along side with
some initial scripts (for js developper it would be "npm init -y"). this project is mainly focused on creating a file that can be used to setup
working directories

## Why?

One day I was making a golang backend that used a similar folder/file structure to another [project](https://github.com/youssefhmidi/Backend_in_go) 
I had done, and the only issue I had was making the "folder structure", copy-pasting wasn't a great idea because I found myself tweaking a lot of 
the original code so it would match the way I want it to be. what did I do after that? I did nothing, I rebuild the folders from scratch and it wasted
a lot of time I would've wasted thinking should I use gorm or sqlx ( I did chose gorm and I was a mistake).

so after that I decided to make a cli tool that would take a file containing all the information about where files should be and execute some
commands like go mod init and others

and basically that is the purpose of this templating language

### Table of index
# Slippi Stat Tracker
Slippi Stat Tracker is an application for compiling/viewing a player's stats based on their Slippi files. While stats are provided by the Slippi launcher, there is currently no way to see compiled stats for several games at a time, and to have access to these stats the user is forced to hold onto the Slippi files, which can quickly pile up. 

![alt text](https://i.imgur.com/nfXyyzf.png "it can get out of hand")

# What's Slippi/Melee? 
Melee AKA Super Smash Brothers Melee, is a game for the Nintendo Gamecube, originally released in 2001, Through the efforts of the community, this game now has rollback netcode, and a full replay system. Slippi is a community project implenting the mentioned features, and Slippi files (.slp) are files generated everytime a game of Melee is played on Slippi. 

For more details, see the Slippi page: https://github.com/project-slippi/project-slippi

This project was also made possible through the use of py-slippi, which is a parser that converts the UBJSON format of Slippi to a much more usable/readable format. (https://github.com/hohav/py-slippi)

# How to Run 

Download the repository, open command line and run 

```
docker-compose build
docker-compose up
```
The application should then be running at localhost:3000/home

A test ZIP file containing some Slippi files has been provided in the repository. Simply upload the zip file to the website, and enter the relevent code (ex: CACT_404) to see the player CACT#404's player information. 

# Project Structure 
Frontend: React (JavaScript)\
Backend: Django(Python) + Rust for calculations \
Database: SQLite



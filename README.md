# TotalMix Volume Control

Provides control over RME TotalMix master volume via OSC.  This repository contains upcoming version of the app which is developed in Rust.  In addition to serving as a great learning experience for me, this new version offers the following advantages over the version written in .NET:

- Lower resouorce usage (this version spawns less than half the threads of the .NET version)
- Much smaller distribution (the entire application is a single executable which is only a few megabytes)
- All aspects of the application from theme colours, window position, scaling, volume increments and connection are configurable
- The cursor will now pass through the window which means that you can click on items behind the window so it doesn't interrupt your work wh en appearing
- The code is generally more OS agnostic and could be adapted to macOS if the need arises and I got access to a Mac to test on

## Current State

The application is fully functional but not ready for day to day use yet.  In particular error handling is not yet taken care of elegantly so the app or various threads could crash if something unexpected occurs.  Furthermore, the loading of a custom configuration file is not yet implemented (although everything is in place to make this possible).

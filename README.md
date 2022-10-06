# Lambda


## KNOWN ISSUES

- Font rendering sucks! This is an issue related to SDL2_ttf 
- Game runs horribly slow in debug builds, simply run in a release build.


## Questions

### Why are items included using ```include_bytes!(...)```?
Items are not assets, so they can be included into the built executable
# To run this script, install vhs and run `vhs tape/reflection.tape`
# Make sure to delete the request history/ folder before.
Output "tape/reflection.gif"
Set Margin 10
Set Padding 2
Set BorderRadius 10
Set Width 1200
Set Height 750
Set PlaybackSpeed 0.5

Hide
Type 'alias wireman="../target/debug/wireman"'
Enter
Type 'clear'
Enter
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/empty"
Enter
Type "wireman"
Enter
Sleep 1.5s
Set TypingSpeed 500ms
Show

# Initialization
Sleep 1.0s

# Reflection
Ctrl+r
Sleep 1.0s
Type 'jjlh'
Sleep 0.5s
Enter
Sleep 0.5s

# Select
Type 'j'
Sleep 1s
Enter
Sleep 1s
Enter
Sleep 1s
Enter
Sleep 2s

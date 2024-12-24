# To run this script, install vhs and run `vhs tape/streaming.tape`
# Make sure to delete the request history/ folder before.
Output "tape/streaming.gif"
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
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/default"
Enter
Type "wireman"
Enter
Sleep 1.5s
Set TypingSpeed 500ms
Show

# Initialization
Sleep 1.0s

Sleep 1.0s
Type 'jjj'
Sleep 0.5s
Enter
Sleep 0.5s
Enter
Sleep 0.5s
Enter
Sleep 0.5s
Enter
Sleep 10s

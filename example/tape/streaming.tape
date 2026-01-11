# VHS demo script for Wireman
# Run `vhs tape/streaming.tape`
# Generates tape/streaming.gif

Output "tape/streaming.gif"
Set Margin 10
Set Padding 2
Set BorderRadius 10
Set FontSize 24
Set Width 1200
Set Height 800
Set PlaybackSpeed 0.35
Set TypingSpeed 50ms

Hide
Type 'alias wireman="../target/debug/wireman"'
Enter
Type 'clear'
Enter
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/default"
Enter
Type "wireman"
Enter
Sleep 0.3s
Show

# initialization
Sleep 0.2s

# navigation
Sleep 0.2s
Type 'jjj'
Sleep 0.2s
Enter
Sleep 0.2s
Enter
Sleep 0.2s
Enter

# streaming
Sleep 0.2s
Enter
Sleep 5.s

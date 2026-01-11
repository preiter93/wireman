# VHS demo script for Wireman
# Run `vhs tape/reflection.tape`
# Generates tape/reflection.gif

Output "tape/reflection.gif"
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
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/empty"
Enter
Type "wireman"
Enter
Sleep 0.3s
Show

# initialization
Sleep 0.8s

# reflection
Ctrl+r
Sleep 0.2s
Type 'j'
Sleep 0.2s
Type 'j'
Sleep 0.2s
Enter
Sleep 0.2s

# select
Type 'j'
Enter
Sleep 0.2s
Enter
Sleep 0.2s
Enter
Sleep 0.2s
Type 'jjwwwwa123'
Escape
Enter
Sleep 0.4s

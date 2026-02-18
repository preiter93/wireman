# VHS demo script for Wireman
# Run `vhs tape/demo.tape`
# Generates tape/demo.gif

Output "tape/demo.gif"
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

# initialization
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/default"
Enter
Sleep 0.3s
Type "wireman"
Enter
Sleep 1.0s
Show

# show help
Sleep 0.2s
Type '?'
Sleep 0.8s
Type '?'

# selection navigation
Sleep 0.2s
Type 'j'
Sleep 0.2s
Enter
Sleep 0.2s
Tab
Sleep 0.2s

# address and token insertion
Type 'j'
Sleep 0.1s
Type 'ji'
Type@30ms '$(getToken.sh)'
Sleep 0.1s
Escape
Sleep 0.1s
Tab
Sleep 0.1s

# make the request
Sleep 0.1s
Type 'jwwwwa'
Type 'John'
Escape
Sleep 0.1s
Enter
Sleep 0.3s

# edit the request
Type '2'
Sleep 0.1s
Type 'hviwdi'
Type 'Paul'
Escape
Enter
Sleep 0.3s

# highlight the response
Type 'J'
Sleep 0.1s
Type 'Vjj'
Sleep 0.3s

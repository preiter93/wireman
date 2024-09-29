# To run this script, install vhs and run `vhs tape/demo.tape`
# Make sure to delete the rquest history/ folder before.
Output "tape/demo.gif"
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

# Initialization
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/default"
Enter
Sleep 0.5s
Type "wireman"
Enter
Sleep 1.0s
Show

# Help
Sleep 1.0s
Type '?'
Sleep 2.0s
Type '?'

# Selection
Set TypingSpeed 100ms
Sleep 0.5s
Type 'j'
Sleep 0.5s
Enter
Sleep 0.5s
Tab
Sleep 0.5s

# Address and token
Type 'jji'
Set TypingSpeed 40ms
Type 'Here we can paste in a token or use a bash script enclosed in $()'
Sleep 0.5s
Escape
Type 'ddi'
Type '$(getToken.sh)'
Escape
Sleep 0.5s
Tab
Sleep 0.5s

# Make the request
Type 'jwwwwa'
Type 'John'
Escape
Sleep 0.5s
Enter
Sleep 1s

# Edit the request
Type '2'
Sleep 0.5s
Type 'hciwdi'
Type 'This request will be saved in your history.'
Escape
Enter
Sleep 0.5s

# Show more features
Type 'jjo'
Type 'If autosave is enabled, each request is saved after send.'
Sleep 0.2s
Enter
Type 'We can also save this request manually with Ctrl+s.'
Sleep 0.2s
Enter
Type 'Or copy the request as grpCurl command with Ctrl+y.'
Escape
Sleep 0.5s
Ctrl+c

Type "You can also change the theme..."
Sleep 0.5s
Hide
Backspace 35
Show

Type "export WIREMAN_CONFIG_DIR=$PWD/configs/dracula"
Enter
# Type 'tail -n 2 config_dracula/wireman.toml'
# Enter
Sleep 1.0s

# Reopen and show that previous request is saved
Type "wireman"
Enter
Sleep 1.0s
Set TypingSpeed 100ms
Type 'j'
Enter
Tab
Sleep 1.5s
Tab
Sleep 1.5s
Type '2'
Sleep 3.0s

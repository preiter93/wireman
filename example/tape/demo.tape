# To run this script, install vhs and run `vhs tape/demo.tape`
# Make sure to delete the rquest history/ folder before.
Output "tape/demo.gif"
Set Margin 10
Set Padding 2
Set BorderRadius 10
Set Width 1200
Set Height 700
Set PlaybackSpeed 0.5

Hide
Type 'alias wireman="../target/debug/wireman"'
Enter
Type 'clear'
Enter
Show

# Initialization
Set TypingSpeed 20ms
Type "export WIREMAN_CONFIG_DIR=$HOME/Rust/wireman/example"
Enter
Sleep 1.5s
Type "wireman"
Enter
Sleep 1.5s

# Selection
Set TypingSpeed 100ms
Type 'j'
Sleep 0.5s
Enter
Sleep 0.5s
Shift+Tab
Sleep 0.5s

# Address and token
Type 'jji'
Set TypingSpeed 20ms
Type 'Here we can paste in a token or use a bash script enclosed in $()'
Sleep 0.5s
Escape
Type 'ddi'
Set TypingSpeed 80ms
Type '$(getToken.sh)'
Escape
Sleep 0.8s
Shift+Tab
Sleep 0.8s

# Make the request
Type 'jwwwwa'
Type 'John'
Escape
Sleep 0.5s
Enter
Sleep 1..5s

# Edit the request
Type '2'
Sleep 0.8s
Type 'hcwdi'
Type 'Doe'
Escape
Enter
Sleep 0.8s

# Show more features
Type 'jjo'
Set TypingSpeed 20ms
Type 'If autosave is enabled, each request is saved after send.'
Sleep 0.2s
Enter
Type 'We can also save this request manually with Ctrl+s.'
Sleep 0.2s
Enter
Type 'Or copy the request as grpCurl command with Ctrl+y.'
Escape
Set TypingSpeed 80ms
Sleep 0.8s
Type 'q'

# Reopen and show that previous request is saved
Sleep 1.0s
Type "wireman"
Enter
Sleep 1.5s
Set TypingSpeed 100ms
Type 'j'
Enter
Enter
Sleep 1.5s
Type '2'
Sleep 1.5s

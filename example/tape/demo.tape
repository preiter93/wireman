# To run this script, install vhs and run `vhs tape/demo.tape`
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
Sleep 1.0s
Type "wireman"
Enter
Sleep 1.5s

# Selection
Set TypingSpeed 100ms
Type 'j'
Sleep 0.5s
Enter
Sleep 1.s
Enter
Sleep 1.s

# Message
Type 'jwwwwa'
Type 'John'
Escape
Sleep 1.s
Tab
Sleep 1.s

# Address and token
Type 'jji'
Set TypingSpeed 20ms
Type 'Here we can paste in a token or use a bash script enclosed in $()'
Sleep 0.5s
Escape
Type 'ddi'
Set TypingSpeed 100ms
Type '$(getToken.sh)'
Escape
Sleep 1.s
Shift+Tab
Sleep 1.s

# Make a request
Enter

# Edit the request
Sleep 1.s
Type 'jwwwwcwd'
Type 'Doe'

# Show more features
Type 'jjo'
Set TypingSpeed 20ms
Type 'We can save this request with Ctrl+s (here in spot 2)'
Sleep 0.5s
Enter
Type 'Or copy the request as grpCurl command with Ctrl+y'
Escape
Sleep 1.0s
Type 'dddd'
Type '2'
Ctrl+'s'
Ctrl+'y'
Escape

# Copy as grpcurl
Paste
Sleep 1.0s

# Reopen and show that previous request is saved
Type "wireman"
Enter
Sleep 1.5s
Set TypingSpeed 100ms
Type 'j'
Enter
Enter
Sleep 0.5s
Type '2'
Sleep 1.5s

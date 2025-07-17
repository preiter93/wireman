# To run this script, install vhs and run `vhs tape/demo.tape`
# Make sure to delete the request history/ folder before.
Output "tape/demo.gif"
Set Margin 10
Set Padding 2
Set BorderRadius 10
Set FontSize 44
Set Width 2100
Set Height 1500
Set PlaybackSpeed 0.4

Hide
Type 'alias wireman="../target/debug/wireman"'
Enter
Type 'clear'
Enter

# Initialization
Type "export WIREMAN_CONFIG_DIR=$PWD/configs/default"
Enter
Sleep 2.0s
Type "wireman"
Enter
Sleep 2.5s
Show

# Help
Sleep 2.5s
Type '?'
Sleep 5.0s
Type '?'

# Selection
Set TypingSpeed 180ms
Sleep 2.5s
Type 'j'
Sleep 1.5s
Enter
Sleep 1.5s
Tab
Sleep 2.0s

# Address and token
Type 'j'
Sleep 1.0s
Type 'ji'
Type '$(getToken.sh)'
Sleep 1.0s
Escape
Sleep 1.0s
Tab
Sleep 1.0s

# Make the request
Sleep 1.0s
Type 'jwwwa'
Type 'John'
Escape
Sleep 1.0s
Enter
Sleep 1.5s

# Edit the request
Type '2'
Sleep 1.0s
Type 'hviwdi'
Type 'Paul'
Escape
Enter
Sleep 3.0s

# Highlight the response
Type 'J'
Sleep 1.0s
Type 'Vjj'
Sleep 3.0s


# Show more features
# Type 'jjo'
# Type 'If autosave is enabled, each request is saved after send.'
# Sleep 0.4s
# Enter
# Type 'You can save this request manually with Ctrl+s.'
# Sleep 0.4s
# Enter
# Type 'Or copy the request as grpCurl command with Ctrl+y.'
# Escape
# Sleep 4.0s
# # Ctrl+c

# Type "You can even change the theme."
# Sleep 1.0s
# Hide
# Backspace 36
# Show

# Type "export WIREMAN_CONFIG_DIR=$PWD/configs/dracula"
# Enter
# Type 'tail -n 2 config_dracula/wireman.toml'
# Enter
# Sleep 1.5s

# Reopen and show that previous request is saved
# Type "wireman"
# Enter
# Sleep 1.5s
# Set TypingSpeed 150ms
# Type 'j'
# Enter
# Tab
# Sleep 2.0s
# Tab
# Sleep 2.0s
# Type '2'
# Sleep 4.0s

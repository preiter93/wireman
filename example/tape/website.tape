# vhs tape/website.tape
# cd tape/website
# ffmpeg -i website.gif website_%03d.png
# cp website_001.png website_026.png website_080.png website_170.png website_225.png website_249.png website_277.png website_326.png website_460.png website_526.png ../../../website/src/lib/assets/.

Output "tape/website/website.gif"
# Output "tape/website/" # does not work

Set Margin 10
Set Padding 2
Set BorderRadius 2
Set FontSize 16
Set Width 830
Set Height 570
Set PlaybackSpeed 1.
Set Framerate 1
Set TypingSpeed 1ms

Type "export WIREMAN_CONFIG_DIR=$PWD/configs/website"
Enter
Type 'alias wireman="../target/debug/wireman"'
Enter
Type "wireman"
Enter
Type "?"
Sleep 1s
Type "?"
Sleep 1s
Enter
Sleep 1s
Type "j"
Sleep 1s
Enter

Sleep 1s
Type "j"
Type "j"
Type "i$(getToken.sh)"
Sleep 1s
Escape
Sleep 1s
Tab

# Request
Sleep 1s
Type "jwwwwa123"
Escape
Sleep 1s
# Open external editor (nvim) in request field
Ctrl+e
Sleep 1s
Type ":wq"
Enter
Sleep 1s
Enter
Sleep 1s
Type "2"
Sleep 1s
Type "J"
Sleep 1s
Type "v"
Sleep 1s
Type "%"
Sleep 1s
Type "y"
Sleep 1s

# Selection
Tab
Sleep 1s
Ctrl+e
Sleep 1s
Ctrl+S
Sleep 1s
Escape
Sleep 1s
Ctrl+r
Sleep 1.0s
Sleep 1.0s
Ctrl+r
Sleep 1.0s

# Toggle horizontal split
# This is usually Alt+S but alt does not seem to work in vhs
Ctrl+S
Sleep 1.0s

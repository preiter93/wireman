# vhs tape/website.tape
# cd tape/website
# ffmpeg -i website.gif website_%03d.png
# cp website_088.png website_111.png website_143.png website_163.png website_231.png website_264.png website_306.png website_360.png website_377.png ../../../website/src/lib/assets/.

Output "tape/website/website.gif"
# Output "tape/website/" # does not work

Set Margin 10
Set MarginFill "#1d262f"
Set Padding 2
Set BorderRadius 2
Set FontSize 16
Set Width 830
Set Height 550
Set PlaybackSpeed 1.
Set Framerate 1

Type "export WIREMAN_CONFIG_DIR=$PWD/configs/website"
Enter
Type "wireman"
Enter
Type "?"
Sleep 1s
Type "?"
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

Sleep 1s
Type "jwwwa123"
Escape
Sleep 1s
Enter
Sleep 1s
Type "J"
Sleep 1s
Type "v"
Sleep 1s
Type "%"
Sleep 1s
Type "y"
Sleep 1s
Ctrl+C

# vhs tape/website.tape
# cd tape/website
# ffmpeg -i website.gif website_%03d.png
# cp website_036.png website_038.png website_049.png website_060.png website_078.png website_095.png website_106.png website_118.png website_138.png ../../../website/src/lib/assets/.

Output "tape/website/website.gif"
# Output "tape/website/" # does not work

Set Margin 10
Set Padding 2
Set BorderRadius 10
Set FontSize 44
Set Width 2200
Set Height 1500
Set PlaybackSpeed 0.5

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

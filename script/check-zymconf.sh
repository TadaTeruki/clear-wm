#!/bin/bash
zym_config_file=$1
zym_default_config_file=$2
xinitrc=$3
xinit_command=$4

zym_config_file_exists=0
xinitrc_exists=0

if [ -d $zym_config_file ]; then 
    zym_config_file_exists=1
fi

if [ -f $xinitrc ]; then
    xinitrc_exists=1
fi

if [ $zym_config_file_exists -eq 1 -a $xinitrc_exists -eq 1 ]; then
    echo "$xinit_command" > $xinitrc
    return 0
fi

echo ""
echo "[Zym Setup Notification]"
echo "WARNING:"

if [ $zym_config_file_exists -eq 0 ]; then
    echo " - Zym configuration file '$zym_config_file' was not found."
fi

if [ $xinitrc_exists -eq 0 ]; then
    echo " - '$xinitrc' was not found."
fi

echo ""
echo "Before starting the session, Zym needs to:"

if [ $zym_config_file_exists -eq 0 ]; then
    echo " - create a new configuration file '$zym_config_file'."
fi

if [ $xinitrc_exists -eq 0 ]; then
    echo " - create '$xinitrc'."
fi


echo ""
echo -n "Do you want to continue it? [Y/n]: "

while true; do
read yn

if [ "$yn" = "Y" -o "$yn" = "y" ]; then

    touch $xinitrc
    echo "$xinit_command" > $xinitrc
    echo "set $xinitrc."

    if [ $zym_config_file_exists -eq 0 ]; then
        cp -r $zym_default_config_file $zym_config_file
        echo "created $zym_config_file."
    fi
    return 0

elif [ "$yn" = "N" -o "$yn" = "n" ]; then
    echo "Aborted"
    return 1
fi

echo -n "'Y' or 'n' [Y/n]: "
done
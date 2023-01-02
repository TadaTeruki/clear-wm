#!/bin/bash
wm_config_file=$1
wm_default_config_file=$2
xinitrc=$3
xinit_command=$4
project_name=$5

wm_config_file_exists=0
xinitrc_exists=0

if [ -d $wm_config_file ]; then 
    wm_config_file_exists=1
fi

if [ -f $xinitrc ]; then
    xinitrc_exists=1
fi

if [ $wm_config_file_exists -eq 1 -a $xinitrc_exists -eq 1 ]; then
    echo "$xinit_command" > $xinitrc
    return 0
fi

echo ""
echo "[$project_name setup notification]"
echo "WARNING:"

if [ $wm_config_file_exists -eq 0 ]; then
    echo " - $project_name configuration file '$wm_config_file' was not found."
fi

if [ $xinitrc_exists -eq 0 ]; then
    echo " - '$xinitrc' was not found."
fi

echo ""
echo "before starting the session, $project_name needs to:"

if [ $wm_config_file_exists -eq 0 ]; then
    echo " - create a new configuration file '$wm_config_file'."
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

    if [ $wm_config_file_exists -eq 0 ]; then
        cp -r $wm_default_config_file $wm_config_file
        echo "created $wm_config_file."
    fi
    return 0

elif [ "$yn" = "N" -o "$yn" = "n" ]; then
    echo "Aborted"
    return 1
fi

echo -n "'Y' or 'n' [Y/n]: "
done
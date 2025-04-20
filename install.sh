cargo build --release

default_package_name="kenji-bluetooth"
read -p "Package name: ($default_package_name)" package_name

if [[ -z "$package_name" ]]
then
		package_name=$default_package_name
fi

read -p "Your window manager: [hypr,i3]" wm

package_dir="/bin/$package_name"
sudo mkdir $package_dir

sudo cp ./target/release/rustybluetooth $package_dir

script_content=""

if [[ $wm = "hypr" ]]
then
		read -p "Terminal you want the app to launch in:" terminal
		script_content='hyprctl dispatch exec "[float;size 800 500;center] '$terminal' -e '$package_dir'/rustybluetooth"'
elif [[ $wm = "i3" ]]
then
		script_content='
		i3-msg floating enable > /dev/null;
		i3-msg resize set width 800 height 500 > /dev/null;
		i3-msg move position center > /dev/null;
		exec /bin/bluetooth/rustybluetooth
		'
else
		echo "Must select either i3 or hypr"
		exit 1
fi

echo "$script_content" | sudo tee -a $package_dir/launch.sh > /dev/null
sudo chmod +x $package_dir/launch.sh

desktop_entry='
[Desktop Entry]
Name=Bluetooth
Comment=Connect to your devices with ease using the Terminal
Exec='$package_dir'/launch.sh
Icon=
Type=Application
Categories=Bluetooth;System;Settings
Terminal=true
Keywords=Bluetooth;System;Settings
'

while true; do
		read -p "Would you like to create a desktop entry? [Y,n]" yn
		if [[ -z "$yn" ]]
		then
				break
		fi
		case $yn in
				[Yy]* ) break;;
				[Nn]* ) exit;;
				* ) echo "Please answer yes or no";;
		esac
done
default_desktop_entry_path="/usr/share/applications"
read -p "Desktop entry path (default $default_desktop_entry_path)" desktop_entry_path

if [[ -z "$desktop_entry_path" ]]
then
		desktop_entry_path=$default_desktop_entry_path
fi

echo "$desktop_entry" | sudo tee -a $desktop_entry_path/$package_name.desktop > /dev/null


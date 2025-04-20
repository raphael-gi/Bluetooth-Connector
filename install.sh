cargo build --release

default_package_name="bluetooth"
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

echo $script_content | sudo tee -a $package_dir/launch.sh > /dev/null
sudo chmod +x $package_dir/launch.sh

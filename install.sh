#!/bin/bash

#alpine arch and suse have packages
#need to build on fedora and deb*

ROOT=$(pwd)

if [ -f /usr/bin/apt ]; then
	distro="deb"
elif [ -f /usr/bin/zypper ]; then
	distro="suse"
elif [ -f /usr/bin/pacman ]; then
	distro="arch"
elif [ -f /usr/bin/dnf ]; then
	distro="fedora"
elif [ -f /usr/bin/apk ]; then
	distro="alpine"
fi

if ! [ -f /usr/bin/keyd ]; then
	# if keyd isnt installed
	echo "Installing keyd dependencies"
	case $distro in
	deb)
		sudo apt install -y build-essential git
		;;
	arch)
		sudo pacman -S --noconfirm base-devel git
		;;
	fedora)
		sudo dnf groupinstall -y "Development Tools" "Development Libraries"
		;;
	esac

	echo "Installing keyd"
	case $distro in
	suse)
		sudo zypper --non-interactive install keyd
		;;
	arch)
		git clone https://aur.archlinux.org/keyd.git
		cd keyd
		makepkg -si --noconfirm
		cd ..
		;;
	alpine)
		doas apk add --no-interactive keyd
		;;
	*)
		git clone https://github.com/rvaiya/keyd
		cd keyd
		make
		sudo make install
		cd ..
		;;
	esac
fi

echo "Generating config"
echo "Not implemented"
exit 1
# python3 cros-keyboard-map.py

echo "Installing config"
sudo mkdir -p /etc/keyd
sudo cp cros.conf /etc/keyd

echo "Enabling keyd"
case $distro in
alpine)
	doas rc-update add keyd
	doas rc-service keyd restart
	;;
*)
	sudo systemctl enable keyd
	sudo systemctl restart keyd
	;;
esac

echo "Installing libinput configuration"
sudo mkdir -p /etc/libinput
if [ -f /etc/libinput/local-overrides.quirks ]; then
	cat $ROOT/local-overrides.quirks | sudo tee -a /etc/libinput/local-overrides.quirks >/dev/null
else
	sudo cp $ROOT/local-overrides.quirks /etc/libinput/local-overrides.quirks
fi

echo "Done"

i3-msg floating enable > /dev/null;
i3-msg resize set width 800 height 500 > /dev/null
i3-msg move position center > /dev/null;

exec /bin/bluetooth/rustybluetooth

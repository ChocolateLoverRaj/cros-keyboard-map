<h1 align="center">Utility to generate keyd configurations for use on Chromebooks</h1>

# Instructions
1. Download the file `chromebook-keyd-config` from GitHub Releases. 
2.     # Copy the file to `/usr/local/bin`
       cd ~/Downloads
       sudo cp chromebook-keyd-config /usr/local/bin
3.     git clone https://github.com/ChocolateLoverRaj/cros-keyboard-map
4.     cd cros-keyboard-map
5.     ./install.sh

## Customizing
You can add ur own keyd config in addition to the chromebook config by creating the file `/etc/chromebook-keyd-config/keyd.conf`. The format is the same as the keyd config, but do not include the `[ids]` section. Here is an example custom config:
```toml
[main]
# It's nice for the lock button to be delete. For sleep you can just close the lid and for lock you can just do Search + L
sleep = delete

# When search key is held make the top row act like function keys
[meta]
sleep = f14
backspace = delete

######## ChromeOS shortcuts ########
[alt]
# alt + meta = capslock
meta = capslock
```

## Building
### Prerequisites
[Install Rust](https://www.rust-lang.org/learn/get-started)

#### Fedora
```
sudo dnf install -y fuse-devel
```

### Testing Service
#### Stop the installed service
```
sudo systemctl stop chromebook-keyd-config
```

#### Build
Run this command every time u wanna test changes.
```
cargo build
```

#### Run
```
sudo target/debug/chromebook-keyd-config
```

#### Test keyd
```
sudo keyd reload
```


Thanks to rvaiya for creating [keyd](https://github.com/rvaiya/keyd).

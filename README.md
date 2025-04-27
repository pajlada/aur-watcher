# aur-watcher

This is a little rust program that polls the [Aurweb RPC interface](https://wiki.archlinux.org/title/Aurweb_RPC_interface) for packages defined in your `config.toml` file, compares it against versions stored in your `$(pwd)/.versions`, and pushes notifications to your configured Gotify instance if a version change has been detected.

See example.config.toml for example usage

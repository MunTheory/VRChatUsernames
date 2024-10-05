# 🛁 VRChat Usernames

Provides a simple script which reads VRChat-formatted logs in its directory and writes all display names into a separate file for ease-of-use.

*More specifically,* the script goes through each `output_log` named file in its directory and tries to find the `OnPlayerJoined` and `OnPlayerLeft` events. These events also log the display name, thus the script parses the line and appends the name to a separate file.

### 🛠️ Setup

1. Open your VRChat logs folder, which it is at `%UserProfile%/AppData/LocalLow/VRChat/VRChat` on Windows.
2. Place the compiled executable in the folder.
3. Optionally, create a shortcut to your desktop for easier access.

### 🪶 License

This repository is distributed under the terms of either the MIT license, the Apache License (Version 2.0), or both.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

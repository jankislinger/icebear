# IceBear - a simple GUI for Data Frames

A simple binary to view parquet files.
This allows you to open parquet files in an app by double-clicking them.
You can also run simple SQL query on top of the data to see easily what's in there.

## Build

### Prerequisites

```shell
sudo apt install -y build-essential \
  librust-atk-dev libcairo2-dev libpango1.0-dev \
  libgtk-4-dev libgdk-pixbuf2.0-dev
```

### Release build

```shell
cargo install --path .
```

Then you can verify by running:

```shell
icebear --help
```

If that doesn't work, make sure you have `~/.cargo/bin` in your `PATH`.

## Associate with parquet files

Taken
from [stack exchange](https://unix.stackexchange.com/questions/490487/how-to-associate-file-extensions-with-my-own-python-script-in-linux).

### Step 1: Create a new mime type

In `/usr/share/mime/packages/` or `~/.local/share/mime/packages/` create a file `parquet.xml` with content:

```
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/x-parquet">
    <comment>Parquet file</comment>
    <glob pattern="*.parquet"/>
  </mime-type>
</mime-info>
```

With this xml file in place, update the mime type database:

```shell
update-mime-database /usr/share/mime
# update-mime-database ~/.local/share/mime
```

### Step 2: Create a .desktop file for the application

In `/usr/share/applications/` or `~/.local/share/applications/` create a file `parquet.desktop` with content:

```
[Desktop Entry]
Exec=/path-to-project/icebear/target/release/icebear %f
Icon=/path-to-project/icebear/logo.png
Name=Parquet GUI
Terminal=false
Type=Application
```

Just change `path-to-project` in `Exec` and `Icon`.
We'll move the binary somewhere outside the project at some point.

### Step 3: Add logo to the mimetype

This is just experimental.
Adding logo in previous step should have worked.
The downside is that the project needs to stay at the same location.

```shell
export SIZE=256
convert logo.png -resize $SIZEx$SIZE /tmp/logo.png
xdg-icon-resource install --context mimetypes --size $SIZE /tmp/logo.png application-x-parquet
```

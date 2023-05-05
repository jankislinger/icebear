# GUI for Data Frames


```shell
cargo build --release
```

## Associate with parquet files

Taken from [stack exchange](https://unix.stackexchange.com/questions/490487/how-to-associate-file-extensions-with-my-own-python-script-in-linux).

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
Exec=/path-to-project/dfgui/target/release/dfgui %f
Icon=/path-to-project/dfgui/logo.png
Name=Parquet GUI
Terminal=false
Type=Application
```

Just change `path-to-project` in `Exec` and `Icon`.
We'll move the binary somewhere outside the project at some point.
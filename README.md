# rsvcf2mutt

A small program to create a address book for mutt from vCards.

## Configuration

Configuration is done through a file in `XDG_CONFIG_HOME/rsvcf2mutt` called
`config.toml`. It must be created by the user and needs the following content:

```toml
contact_path = "/path/to/vCard/folder"
mutt_config_path = "/path/to/mutt/config"
```

Shortening the home directory to `~` is not yet supported, so it needs to be
spelled out explicitly.

`rsvcf2mutt` will write a file called `rsvcf2mutt_addressbook.muttrc` in the
directory specified in `mutt_config_path`. This file can than be sourced in the
`muttrc` to be accessible as a address book in mutt.

## systemd

If `rsvcf2mutt` should be run automatically wit `systemd` the following files
need be created:

### rsvcf2mutt-oneshot.service

```systemd
[Unit]
Description=rsvcf2mutt (oneshot)

[Service]
Type=oneshot
ExecStart=$(PATH_TO_REPOSITORY)/target/release/rsvcf2mutt
TimeoutStopSec=120
```

`$(PATH_TO_REPOSITORY` needs to be replaced with the path to this repo.
`rsvcf2mutt` also needs to be compiled with the `--release` flag.

### rsvcf2mutt-oneshot.timer

```systemd
[Unit]
Description=rsvcf2mutt timer

[Timer]
OnBootSec=15m
OnUnitInactiveSec=24h

[Install]
WantedBy=default.target
```

Afterward the timer needs to be started with `systemctl --user start rsvcf2mutt-oneshot.timer` and enabled to run on login with `systemctl --user enable rsvcf2mutt-oneshot.timer`.

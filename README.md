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

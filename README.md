# Arch Server Notifier Daemon

This Rust application notifies you when an Arch Linux server becomes available after downtime. 

The project now features a modular architecture with monitoring logic separated into its own module. The monitoring includes retrying failed connection attempts multiple times with delays, improving reliability.

Future development aims at enhancing user experience further, including extended configuration options, notification methods, and test coverage — all to provide a robust tool for users of Arch-based distributions.


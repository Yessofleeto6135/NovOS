#!/bin/bash

# Initial setup script for NovOS - Debian Minimal

# Update package index and upgrade existing packages
sudo apt update && sudo apt upgrade -y

# Uninstall unnecessary software
sudo apt remove --purge <unwanted-packages> -y

# Setup for 32-bit architecture
sudo dpkg --add-architecture i386

# Install lightweight desktop environment (Openbox)
sudo apt install openbox -y

# Placeholders for custom apps
# Nov Browser: A lightweight web browser for NovOS
# TODO: Install Nov Browser using 'apt install nov-browser'

# Nov File Manager: A simple file manager for NovOS
# TODO: Install Nov File Manager using 'apt install nov-file-manager'

# Nov Settings: Configuration tool for NovOS
# TODO: Install Nov Settings using 'apt install nov-settings'

# Finalize the installation
sudo apt autoremove -y

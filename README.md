![WirtBot logo](Interface/public/logo.svg)

# WirtBot
**Easily and securely extend your network into the cloud!**

<br/>


Welcome to the [WirtBot](https://wirtbot.com) repository.

## Run in Docker

This image needs **NET_ADMIN** capabilities and:

- Linux Kernel > 5.6 || WireGuard® Kernel Module

Here is an example `docker-compose.yml` with WireGuard:
```
version: "3"

services:
  wirtbot:
    build: 
      context: ../../..
      dockerfile: build-automation/WirtBot/Dockerfile
      
    ports: [3030:3030]
    restart: always
    volumes: ["../..:/app"]
    environment:
      "PUBLIC_KEY": "1lLU3VhXsrSGMxESmqfY4m2oEVkpfEHyKlCQU6MMPsI="
      "ALLOWED_ORIGIN": "http://localhost:8080"
      "RUST_LOG": "debug"
      "MANAGED_DNS_ENABLED": 1
    cap_add:
      - NET_ADMIN
    volumes:
      - ./test-server.conf:/etc/wireguard/server.conf
      - ./test-Corefile:/dns/CoreFile
    ports:
      - 80:80
      - 3030:3030
      - 10101:10101/udp
    container_name: development_wirtbot
```

## Setup WireGuard VPS

**Install:**
- [ansible](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html)
- [sshpass](https://gist.github.com/arunoda/7790979) (necessary to login via SSH to your servers root user via password for the setup)
- [node](https://nodejs.org/en/download/)

**Run:**
```
npm install -g @wirtbot/installer && wirt-installer
```
**Join the network with generated WireGuard config**

**Assume control of WirtBot by importing generated initial configuration**

**Done**

## What is this for?

Think of it as your LAN extended over the whole internet, but only accessible to your devices.

- Do you want **replace your [GoogleDrive, AppleCloud etc.]** with [Nextcloud](https://nextcloud.com/)?
- Want to **safely resolve DNS**?
- Developing an App/Website on your machine and want to **show it off live on your phone via the internet**?
- Have loads of **IOT devices** that you want to be able to **communicate safely** with each other?

A WirtBot got you covered. 


## How?

**WirtBot** is a system/robot that is designed after the UNIX philosophy. 
Many small parts that do a job and do it well. Putting special importance into correct configuration.
Everything is controlled via a UI that focusses on **ease of use** and all is glued together using Infrastructure as code.
If you set it up you can rest assured that it is done properly.

Ready to get started?

Read the setup documentation at [wirt.network/docs/setup](https://wirt.network/docs/setup) and get your own cloud network with **1 command on the terminal** and a **few clicks in your browser**!

## Features
- Based on WireGuard for **security** and **speed**
- Find all your devices by their name! **DNS** is included
- Every configuration in the network is kept **up to date** on changes
- **Export and Import** feature to keep track of multiple networks and generate Backups
- **Absolutely no tracking** + everything is uniquely tied to your browser!
- **Configuration over code**. Leverage existing tooling and provide a clean interface for configuring it correctly

## Contributing
![WirtBot System Tests](https://github.com/b-m-f/WirtBot/workflows/run%20WirtBot%20system%20tests/badge.svg)
![WirtBot Tests](https://github.com/b-m-f/WirtBot/workflows/run%20WirtBot%20tests/badge.svg)
![WirtBot Shared-Libraries Tests](https://github.com/b-m-f/WirtBot/workflows/run%20WirtBot%20shared-lib%20tests/badge.svg)

![WirtBot Interface Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/bmff/wirtbot-interface?label=WirtBot%20Interface%20build)
![WirtBot Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/bmff/wirtbot?label=WirtBot%20build)
![WirtBot Website Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/bmff/wirtbot-website?label=WirtBot%20Website%20build)

You want to help out making WirtBot better? **AWESOME**! 

Please feel free to open up an issue here in Github to discuss what you would want to do, and what kind of help I can give you.

If you want to learn about the inner workings of the system check out the [developer documentation](https://wirtbot.com/developer-documentation/).

I also encourage you to ask questions! My goal is to document as many of those questions as I can, to allow more people to understand what is going on in the code.


For an overview of things that need help check the [issues](https://github.com/b-m-f/WirtBot/issues).


## LICENSE

The WirtBot and Interface are licensed with `GNU Affero General Public License v3.0`

The documentation and WirtBot logo are provided under `Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)`

## Donate

Would you like to contribute a bit to the server costs? ~10$ a month.

You can easily do so to the Ethereum address: `0x39DF85776c626117B945afECF08C4DD262817522`

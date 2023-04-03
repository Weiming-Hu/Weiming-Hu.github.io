---
layout: post
title: "FAQ: Dedicated Server for Don't Starve Together (2021 Version)"
description: "Dont't starve together!"
tags: Game
giscus_comments: true
date: 2021-06-16T18:00:00
related_posts: false
---

This post is a memo for the problems I encountered during setting up a dedicated server for [Don't Starve Together](https://www.klei.com/games/dont-starve-together).

### What is my setup for the dedicated server?

I used Amazon AWS services. It is a cloud service and a pay-as-you-use service. Specifically, I chose the `t3.xlarge` instance running the latest version of Ubuntu. I have tried the free tier instance but that seems not to have enough RAM.

You should also choose the physical location of your server, preferably closest to your physical location. This will affect your connection lag.

### How to set up the dedicated server?

There is a detailed tutorial on how to set up dedicated servers on Linux [here](https://steamcommunity.com/sharedfiles/filedetails/?id=590565473). You can also find related tutorials [here](https://steamcommunity.com/id/ToNiO44/myworkshopfiles/?section=guides&appid=322330), for example, setting up servers for other operating systems.

### What is my username on the AWS server?

Before you try to do anything on the server, you need to [connect to the server](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AccessingInstancesLinux.html) and you need a username. Where to find this username? It is usually just a default username based on your selected distribution, for example, `ubuntu` for Ubuntu. See details [here](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/connection-prereqs.html#connection-prereqs-get-info-about-instance).

### What is admin on the dedicated server?

At certain point in the tutorial, you will see the steps to add admin. This is to enable you to act like the owner of the world when playing games, so that you can do things like stop the servers, revert days, and spawn items with `c_spawn`.

You can definitely location your ID as it is mentioned in the tutorial. Or you can go [here](https://accounts.klei.com/account/info). Under `USER INFO` and `Klei User ID`, you will a string of characters like `KU_***`. That is your ID to use when adding an admin. You might need to login first.

### How to avoid world reset after server restart?

The problem is I don't want to leave the server running all the time because it is pay-as-you-use. I want to only run the server when I play the game and stop the server when I'm done. The way to stop the server mentioned in the tutorial does not quite work because I **lose my world**! So I need a way to make the world persistent after server reboot.

You would want to shut down your world when you are in the game. Press &#96; (backtick) on your keyboard to bring up a prompt and then you enter `c_shutdown()`. You wait for a while and you will receive a message of connection loss.

At this point, you are safe to stop your server and the world is properly saved.

### How to add mods?

Follow [this tutorial](https://steamcommunity.com/sharedfiles/filedetails/?id=591543858) to install mods on servers. There is a lot information and tutorials. Make sure you are reading the correct one for dedicated servers.

There is one caveat: this file `~/server_dst/mods/dedicated_server_mods_setup.lua` will be reset every time you update the server using the `update.sh` created during [this tutorial](https://steamcommunity.com/sharedfiles/filedetails/?id=590565473). So I back up my `.lua` file and added the following line after `sleep 10`:

```bash
cp ~/dedicated_server_mods_setup_backup.lua ~/server_dst/mods/dedicated_server_mods_setup.lua
```

`~/dedicated_server_mods_setup_backup.lua` is basically just a backup. The update process won't affect your `modoverrides.lua` under `~/.klei/DoNotStarveTogether/MyDediServer/Master` and `~/.klei/DoNotStarveTogether/MyDediServer/Caves`.

### How to move the AWS server to a different location?

[To be continued]

### How to move the world to a different device?

[To be continued]

### Still have questions?

Please leave a comment. Thank you!

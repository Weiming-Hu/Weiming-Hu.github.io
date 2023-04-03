---
layout: post
title: "Code Signing GNU GDB on Mac for Debugging C++"
description: "A small issue with using GNU GDB on Mac"
tags: C++
giscus_comments: true
date: 2018-03-24T20:00:00
related_posts: false
---

## Purpose

Debugging C++ code on a Mac machine is definitely not the most pleasant thing to do. I decided to keep a cumulative note here for my experience of code signing [gdb](https://www.gnu.org/software/gdb/) on my Mac machine as tools are constantly changing. And actually it is always hard to find the right suggestion for a specific issue. This would be helpful if you are using a Mac, GDB as your debugger, and some kind of IDE like [NetBeans](https://netbeans.org).

Specifically, my configuration is as follow:

- macOS High Sierra Version 10.13.3
- GNU gdb (GDB) 8.0.1
- NetBeans IDE 8.2
- Homebrew 1.5.12-27


## Some Known Issues

### please check gdb is codesigned - see taskgated(?)

[xdavidliu](https://stackoverflow.com/questions/49184931/subject-cannot-codesign-system-certificate-for-gdb-in-keychain-access-in-mac-os) explains and solves this problem very well. Please refer to the answer.

Please make sure that you notice that **the latest version of gdb (8.1) cannot be code signed according to the above answer**.

### An Error Occurred Unknown Error = -2,147,414,007

This can probably happen when you try to create a certificate directly in the `System`. A workaround for this is to first, create the certificate in `Login` and export it as a file. Then import the certificate to `System`. You can find the steps [Here](https://stackoverflow.com/questions/49184931/subject-cannot-codesign-system-certificate-for-gdb-in-keychain-access-in-mac-os).


## References

- [A most recent post on codesigning an older version of GDB](https://stackoverflow.com/questions/49184931/subject-cannot-codesign-system-certificate-for-gdb-in-keychain-access-in-mac-os)
- [The guide from installing the older version 8.0.1 of GDB](https://sourceware.org/gdb/wiki/BuildingOnDarwin)
- [How to install and codesign GDB on OS X El Capitan](https://medium.com/@royalstream/how-to-install-and-codesign-gdb-on-os-x-el-capitan-aab3d1172e95)

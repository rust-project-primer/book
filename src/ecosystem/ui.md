# User Interface

While most development these days targets web or mobile, there are situations
where a traditional local GUI applications is needed. This section explains
some approaches that are popular in Rust.

In general, most Rust development targets places that the end user does not
directly interact with: backend applications, servers, firmware. But there are
cases where it makes sense to slap together a quick GUI for something, for
prototyping or to be able to use the ecosystem of libraries that Rust offers.

## Tauri

[Tauri](https://tauri.app/)
is a project that achieves something similar to Electron: it embeds a web
view into an application, and allows you to use web technology to write your
user interface.  This can be combined with a [Rust frontend
application](./web-frontend), or it can be a traditional JavaScript
application. In addition, Tauri offers some ways to expose an API to the
application.

Tauri is very lightweight and is a good choice for anything from quick
prototyping to releasing production applications that work cross-platform.

- example: tauri with yew rs

## GTK-rs

[GTK](https://www.gtk.org/) is a library that spawned out of the GIMP image
editor, and has since become the standard UI framework for the GNOME desktop
environment, which is used by many Linux distributions. GTK works on most
platforms and is conceptually quite simple.

The [GTK-rs](https://gtk-rs.org/)
project aims to create wrappers around it to expose it's
functionality natively to Rust, making it possible to write portable GUI
applications. They have succeeded in making it somewhat idiomatic, working
around the quirks of GTK with decent documentation and procedural macros.

- example: gtk rs calculator

## egui

## Reading

[Are We GUI Yet](https://areweguiyet.com/)

---
title: components
---

Upon startup, components get as arguments:

- A rodio stereo channel for playing samples
- A channel handle for reporting state change

Components need to accept the following methods:

.play() -> Result<(),E>
.pause() -> Result<(),E>
.stop() -> Result<(),E>
.status() -> Result<CompStatus,E>
.load(id:ulnd) -> Result<(),E>
.set_repeat(status:bool) -> Result<(),E>
.set_random(status:bool) -> Result<(),E>
.get_playables_menu() -> Result<CompMenu,E>
.get_playables_json() -> Result<String, E>

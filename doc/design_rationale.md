---
title: Design Rationale
---

The idea here is to present an audio playback appliance, that accepts UI commands from n sources, and uses those enqueued commands to manipulate the audio stream coming out of the appliance.

Audio streams are available from components.

Clients are only intended to be used to manipulate appliance state, not play music themselves.

When in doubt, make no noise.

Isn't this an arbitrarily limited system? You betcha!


Do I want the components to respond to commands on the internal bus, or to functions?

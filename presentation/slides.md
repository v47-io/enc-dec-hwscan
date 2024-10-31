# Going Full Platform-Specific

> Nobody really needs anything else than 64-bit Linux.

![bg left:25%](./images/korea-palace-nature.png)

<!--
At least until we get some proper consumer ARM hardware other
than those godawful "Copilot PCs"

Original "quote" was: This shit's gonna have Rust in it.
Allegedly Deadpool said that.
-->

---

## `$ whoami`

- __Alex Katlein__
- Freelancing software consultant
- Avatar of NIH
- Drawing from vacation high

<!--
Spent most of my career doing consulting work, except one small exception where
I actual worked for the same company that paid my salary.
Nowadays I mainly do software architecture but I'm a coder at heart.
-->

> TODO: add vacation photo collage

---

### Tech

- Kotlin since 2015 (`M13`)
- Rust since 2018 (`1.26.0`)

![bg right](./images/kotlin-and-rust.png)

<!--
Standout features:
  - Kotlin
    - lateinit keyword (basically it's you knowing more than the compiler)
    - current visibility rules (which are simple but sometimes not enough)
      (there's currently a huge debate about additional rules on YouTrack)
  - Rust
    - main can return Result (no more need to create a wrapper for your)
      (actual main function which just adds boilerplate)
    - nicer match bindings for Option (no more requirement to specify)
      (variants as references)
-->

---

### Work

- IoT for media streaming (🚄)
- Neobanking (until bankruptcy)
- Single point of failure for e-commerce
- Rearchitecting core insurance system

<!--
Just a small excerpt and overview:

- IoT
  - Basically treated trains like huge IoT devices
  - Had to contend with bad network infrastructure in Germany
  - Clever caching and batching required for monitoring data
- Neobanking
  - For gamers 🤪
  - Spent more time debugging external core banking system
    than anything else
- e-commerce
  - Team ran the reverse proxy for the entire platform
  - 1/2 hour of outage was a loss of 2 million EUR
- Core Insurance System
  - Project was sadly cancelled before we could really get started
  - Although we did get some major observability and deployment wins
-->

---

### Projects

- The Movie Database API client
- Jaffree Fork
- ez-dyndns-rs
- ...

<!--
Most of my project seem to stem from Media Server 47, which is
the reason for this talk. Kinda fits the pattern TBH.

Jaffree is an FFmpeg API for JVM (wraps the ffmpeg executable).
Hostile fork because of fundamental differences between original creator
and me. For example: I want to use standard Java concurrency APIs like
CompletionStage (which incidentally integrate nicely with other concurrency
libraries like Kotlinx Coroutines or rxJava) but the original creator wasn't
interested. Also he is managing all his process specific threads himself,
which I think is unnecessary with libraries like NuProcess.
Oh well, I will link to the project at the end of the presentation and you
can check for yourself.
-->

---

## Thanks

- His Majesty, Rainer
- Specific-Group Austria

> TODO: add SPG logo

---

## `$ ls -l`

- Media Server 47
- Constraints
- The Problem
- Rust
- INTERMISSION
- Kotlin
- The Solution
- Code & Demo

---

## Media Server 47

- A streaming server
- Zeroconf where possible
- Testbed for things I want to try

<!--
I can't believe it's been ten years since I started working on something
like it xD
It just took me that long to finally learn YAGNI and focus, even though
I still sometimes fail at that.
The main objectives for my media server are first and foremost to learn
new stuff, to try new ways of structuring software, and also to test
exciting technologies in the confines of a proper project instead of creating
toy projects for problems that don't exist.
I want it to adapt to the environment it runs on, instead of the user having
to adapt the environment to it. It should detect most settings on its own,
only requiring dynamic configuration by the user for critical stuff, e.g.
security.
For now the database and Valkey server are the only things a user would
have to specify using environment variables.
-->

---

### Jellyfin

- Fork of Emby when it went closed-source
- Very good transcoding capabilities
- _NOT_ zeroconf

<!--
Also, it's written for .NET using C# and ASP.NET, which I absolutely don'y
care about. But that's my idiological stance, which doesn't have to align
with others'.
It's transcoding capabilities however are excellent, they even maintain
their own FFmpeg port with their own patches (which take a while to upstream).
My media server is going to use their FFmpeg port, simply because it's proven
and feature-complete for such a scenario. So, shout out to Jellyfin!
However, hardware accelerated transcoding requires a bit of configuration, and
the client capabilities aren't reliably detected which can lead to playback
failures with unsupported media formats.
-->

<!-- TODO: add jellyfin screenshot, stylized -->

---

### Plex

- It's got everything: Local media, TV, Radio, etc...
- Not very customer friendly (not FOSS)
- Important features behind paywall

<!--
Plex has turned into a business. It used to be the one-stop shop for all
homestreamers' desires, but it has turned against them. Now it's just for
the normies who don't necessarily have the required technical acumen to
run a proper homelab (i.e. people who use Synology or QNAP "NAS" solutions).
The part where it's hostile comes from the fact that they require a user-
registration and login even when self-hosting.
And if you want to use your hardware for hardware accelerated transcoding
you need to pay a monthly fee, just for the privilege of utilizing your
own hardware to the fullest. (yuck)
-->

<!-- TODO: add plex promo art -->

---

### Others...

- Emby
- and a boatload of other proprietary solutions...

<!--
Don't want to spend too much time on stuff that doesn't matter,
like Emby (ever since they went closed-source) and other proprietary
software. Just know that it exists, I don't like it, and move on.
-->
---

## Constraints

<!-- limited computing power -->
<!-- limited storage -->
<!-- hard requirement for faster than real time transcoding -->

---

### Consumer GPUs

<!-- Nvidia best -->
<!-- AMD catching up -->
<!-- Intel most common due to iGPUs -->

<!-- add GPU vendor graphics -->

---

## The Problem

<!-- detection of transcoding capabilities, manual config not acceptable -->
<!-- ask the transcoding hardware -->

---

### Utilities

<!-- provided by driver or extra packages -->
<!-- varying levels of usefulness -->

---

#### `vainfo`

<!-- complicated parsing required -->

---

#### `nvidia-smi`

<!-- output very limited, requires LUT -->
<!-- LUT is a PITA -->

---

## Rust

<!-- what is it -->
<!-- why is it appealing -->

---

### C (the protocol)

<!-- C not as a programming language but as a protocol -->
<!-- static or dynamic compilation -->

---

### Dynamic library loading

<!-- why is it preferable here -->
<!-- patterns -->

---

### Gotchas

<!-- working with native C structs -->
<!-- exposing native C structs -->
<!-- cleanup of memory -->

---

![bg 105%](./images/intermission-upscaled.png)

<!-- insert bollywood intermission text here -->

---

## Kotlin

<!-- full interop with the JVM ecosystem -->
<!-- need this for GraalVM -->
<!-- oh, and the Java Foreign Function and Memory Access API -->

---

### `jextract`

<!-- requires C header -->
<!-- produces base output -->

---

### Working with GraalVM

<!-- Use of a supporting framework advisable, e.g. Quarkus -->
<!-- Some static init gotchas, esp. for MethodHandle -->

---

### Alternatives

- JNI
- JNA
- UniFFI (by Mozilla)
- Kotlin/Native

<!--
- JNI
  - unsound memory model
  - clunky API
- JNA
  - prohibitively reflection heavy (not possible to get it work with GraalVM)
  - large runtime overhead
- UniFFI
  - already got an interface language: C
  - unsound resource management
- Kotlin/Native
  - don't want another garbage collector
  - when do I execute the GC?

Rust allows me to use the lingua franca of the OS, C, and brings a highly
optimized memory management model which lets me focus on the fun parts.
-->

---

## The Solution

<!-- let's ask the GPU vendor -->
<!-- and by that I mean we ask the native transcoding libraries -->

---

### VA-API

<!-- vaapi headers are easy to come by -->
<!-- well documented -->
<!-- have to enumerate the available device files yourself -->

---

#### CUVIDEC and NvEncode

<!-- two different styles of APIs (one C based, the other C++) -->
<!-- cuda headers not included by default, and licensed differently -->
<!-- powerful API, explain floating CUDA contexts -->

---

### Where's AMD?

<!-- actually covered by VAAPI -->
<!-- powerful opensource driver included in Linux kernel -->
<!-- mention inferior proprietary driver (only for specific use cases) -->

---

### Generating Rust bindings

<!-- mention required filtering of functions and structs -->
<!-- explain a bit about macros to handle calling and error handling -->

---

## Code

<!-- TODO: lead through code -->
<!-- start with going through the project structure -->
<!-- then the Rust code, choose one or two functions as examples -->
<!-- go through JVM bindings, jextract output, own workarounds -->
<!-- go through mapping to JVM types -->
<!-- explain final GraalVM build using Quarkus -->

---

## Demo

<!-- show off locally, via SSH on BEAST, and via SSH on SteamDeck -->
<!-- TODO: prepare video in advance in case something goes tits-up during the demo -->

---

## Takeaways

- The JVM is a powerful platform
- Now with powerful friends!
- Think outside the box!

---

## Links

[enc-dec-hwscan @ GitHub](https://github.com/v47-io/enc-dec-hwscan)
<!-- links to project on github -->
<!-- links to helpful resources (one or two) -->

<!-- link to original Jaffree and my own Jaffree -->
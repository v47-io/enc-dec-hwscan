# Going Full Platform-Specific

> This shit's gonna have Rust in it. &mdash; Deadpool (allegedly)

---

## `$ whoami`

- __Alex Katlein__
- Freelancing software consultant
- Avatar of NIH
- Drawing from vacation high

<!-- add vacation photo collage -->

---

### Tech

- Kotlin since 2015 (`M13`)
- Rust since 2018 (`1.26.0`)

<!-- TODO: talk about a few standout features of those releases -->
<!-- add rust and kotlin logo -->

---

### Work

- IoT for media streaming (🚄)
- Neobanking (until bankruptcy)
- Single point of failure for e-commerce
- Rearchitecting core insurance system

<!-- TODO: mention it's just a few and just give one or two notable things for each -->
<!-- add logos of various customers -->

---

### Projects

- The Movie Database API client
- Jaffree Fork
- ez-dyndns-rs
- ...

<!-- TODO: mention how most projects seem to stem from Media Server 47 -->
<!-- also, hostile Jaffree Fork -->

---

## Thanks

- His Majesty, Rainer
- Specific-Group Austria

<!-- add SPG logo -->

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

<!-- main goals, ease of setup, autodetect everything -->
<!-- why??? low barrier to entry, trying new stuff -->

---

### Jellyfin

<!-- encoding capabilities, esp jellyfin-ffmpeg -->
<!-- mention .NET (bleh) -->

<!-- add jellyfin screenshot, stylized -->

---

### Plex

<!-- full media consumption platform for the home user -->
<!-- but hostile to the customer, e.g. registration required for HW transcoding -->

<!-- add plex promo art -->

---

### Others...

<!-- e.g. Emby, etc... -->

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

# INTERMISSION

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

<!-- why not use JNA -->
<!-- why not use uniffi -->
<!-- why not use Kotlin/Native -->

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

<!-- JVM is still a powerful platform -->
<!-- with powerful friends -->
<!-- don't be afraid to think outside the box -->

---

## Links

<!-- links to project on github -->
<!-- links to helpful resources (one or two) -->

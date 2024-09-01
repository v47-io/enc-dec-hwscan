## Going Full Platform-Specific

> This shit's gonna have Rust in it. &mdash; Deadpool (allegedly)

---

## `$ whoami`

- Alex Katlein
- Avocado-toast-eater
- Self-employed software architect
- Avatar of NIH

...

### Tech

- Kotlin since 2015 (`M13`)
- Rust since 2018 (`1.26.0`)

<!-- TODO -->

...

### Projects

<!-- TODO -->

...

### Work

<!-- TODO -->

---

## Thanks

> To Rainer, SPG, etc ...

<!-- TODO -->

---

## `$ ls -l`

- Background
- Constraints
- The Problem
- Solution
- Rust
- Kotlin
- Code

---

## Background

<!-- TODO -->

- creation of a home-streaming solution
- ease of setup
- true freedom

...

### Why stream at home

<!-- TODO -->

- freedom
- no one can take it away
- your selection
- talk about piracy, copyright and archival copies (mention situation in AT and DE)

...

### Existing home-streaming solutions

<!-- TODO -->

- give overview over jellyfin
- and why I won't contribute
- give absolute showstopper of Plex (Plex Pass, paying for hardware transcoding)

...

### Configuration mess

- what is required (esp. for nvidia GPUs)
- with jellyfin screenshots

<!-- TODO -->

...

### Streaming issues

<!-- TODO -->

- Jellyfin not taking into account individual capabilities

---

## Constraints

- encoding is computationally expensive
- every homelab has different hardware
- storage is typically limited

<!-- TODO -->

---

### Shifting work to the compiling phase

- expensive encoding to enable cheap decoding
- popular codecs

<!-- TODO -->

...

### H.264 / HEVC

<!-- TODO -->

- patent encumbered
- platform shortcomings

...

### VP9

<!-- TODO -->

...

### AV1

<!-- TODO -->

- really fucking expensive to encode
- but best results with minimal file size

---

### State of consumer hardware

<!-- TODO -->

...

#### NVIDIA

- mention keylase patch

<!-- TODO -->

...

#### AMD

- poorer quality, and different APIs (proprietary, Va-api)

<!-- TODO -->

...

#### Intel

- mention ubiquity

<!-- TODO -->

---

### Storage considerations

<!-- TODO -->

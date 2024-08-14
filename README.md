# Encode/Decode Hardware Scan

A library illustrating the use of Kotlin, Rust, Java 22's Foreign Function and Memory API, Quarkus,
and GraalVM Native Image to provide a focused development experience when targeting a singular
platform (x86_64 Linux).

Using Java 22 makes it possible to provide a simple native interface without the usual JNI cruft.
This results in a native library that can be used outside the confines of this project like
any other shared library.

Currently implemented are support for `CUDA >= 5.0` and `NvEnc >= 7.0` to retrieve information
about NVIDIA encoding/decoding hardware, and VA-API for any other supported GPUs (Intel, AMD, ...).
This means that all consumer NVIDIA cards starting with the GeForce 700 series are supported.

## Usage

Start `integration-test` and access [localhost:8080/devices](http://localhost:8080/devices) to
retrieve the list of detected hardware and its capabilities.

Example output:

```json
{
  "devices": [
    {
      "driver": "Nvidia",
      "ordinal": 0,
      "path": null,
      "name": "NVIDIA GeForce RTX 3090",
      "codecs": {
        "Mpeg1": {
          "codec": "Mpeg1",
          "decodingSpecs": [
            {
              "chroma": "Yuv420",
              "colorDepth": "Bit8",
              "maxWidth": 4080,
              "maxHeight": 4080
            }
          ],
          "encodingSpecs": []
        },
        "Mpeg2": {
          "codec": "Mpeg2",
          "decodingSpecs": [
            {
              "chroma": "Yuv420",
              "colorDepth": "Bit8",
              ...
```

## Development

### Prerequisites

- Linux (no Windows or macos)
- Mandrel or Liberica NIK 24.r22 (JDK 22, for `native-image`)
- [Jextract](https://jdk.java.net/jextract/)
- Clang
- Rust 2021 Edition
- `cuda.h` in `/usr/include` (get it from [Cuda Toolkit][cuda-tk])
- VA-API:
    - Debian like: `libva-dev`
    - RHEL like: `libva-devel`

[cuda-tk]: https://developer.nvidia.com/cuda-toolkit

### Building

```shell
./gradlew build
```

Native Image:

```shell
./gradlew build \
  -Dquarkus.package.jar.enabled=false \
  -Dquarkus.native.enabled=true
```

The final result is a native executable that can be run on any machine provided the required
libc version is available.

pluginManagement {
    plugins {
        val kotlinVersion = "2.0.10"
        kotlin("jvm") version kotlinVersion
        kotlin("kapt") version kotlinVersion
        kotlin("plugin.allopen") version kotlinVersion

        id("com.github.hierynomus.license") version "0.16.1"
        id("fr.stardustenterprises.rust.importer") version "3.2.5"
        id("fr.stardustenterprises.rust.wrapper") version "3.2.5"
        id("io.github.krakowski.jextract") version "0.5.0"
        id("io.quarkus") version "3.12.0"
        id("io.quarkus.extension") version "3.12.0"
    }
}

dependencyResolutionManagement {
    @Suppress("UnstableApiUsage")
    repositories {
        mavenCentral()
    }
}

rootProject.name = "enc-dec-hwscan"

include(
    "native",
    "bindings",
    "library",
    "deployment",
    "integration-test"
)

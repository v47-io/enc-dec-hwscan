pluginManagement {
    plugins {
        val kotlinVersion = "2.0.10"
        kotlin("jvm") version kotlinVersion
        kotlin("kapt") version kotlinVersion
        kotlin("plugin.allopen") version kotlinVersion
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

project(":library").name = "enc-dec-hwscan"
project(":deployment").name = "enc-dec-hwscan-deployment"

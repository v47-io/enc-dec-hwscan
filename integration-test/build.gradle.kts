plugins {
    kotlin("jvm")
    kotlin("kapt")
    kotlin("plugin.allopen")

    id("io.quarkus")
}

dependencies {
    implementation(platform(libs.quarkus.bom))

    implementation(project(":library"))

    implementation(libs.quarkus.rest.jackson)
    implementation(libs.quarkus.rest.kotlin)

    testImplementation(libs.jackson.module.kotlin)
    testImplementation(libs.quarkus.junit5)
    testImplementation(libs.rest.assured)
}

allOpen {
    annotation("io.quarkus.test.junit.QuarkusTest")
    annotation("jakarta.ws.rs.Path")
}

tasks.test {
    useJUnitPlatform()
}

tasks.quarkusBuild {
    nativeArgs {
        @Suppress("UNCHECKED_CAST")
        this as MutableMap<String, Any>

        put(
            "additional-build-args",
            listOf(
                "-Ob",
                "-H:+UnlockExperimentalVMOptions",
                "-H:+ForeignAPISupport",
                "-H:-UnlockExperimentalVMOptions",
                "--enable-native-access=ALL-UNNAMED"
            ).joinToString(",")
        )

        put(
            "builder-image",
            "quay.io/quarkus/ubi-quarkus-mandrel-builder-image:24.0-jdk-22"
        )
    }
}

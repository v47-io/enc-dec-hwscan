plugins {
    kotlin("jvm")
    kotlin("kapt")
    kotlin("plugin.allopen")

    alias(libs.plugins.quarkus)

    alias(libs.plugins.detekt)
}

dependencies {
    implementation(platform(libs.quarkus.bom))

    implementation(project(":enc-dec-hwscan"))

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
    }
}

plugins {
    kotlin("jvm")
    kotlin("kapt")

    alias(libs.plugins.quarkus.extension)

    alias(libs.plugins.detekt)
    alias(libs.plugins.dokka)

    `maven-publish`
}

dependencies {
    implementation(platform(libs.quarkus.bom))

    implementation(project(":bindings"))
    implementation(libs.jackson.annotations)
    implementation(libs.quarkus.arc)

    testImplementation(libs.quarkus.junit5)
}

quarkusExtension {
    deploymentModule.set(":enc-dec-hwscan-deployment")
}

tasks.test {
    useJUnitPlatform()
}

publishing {
    publications {
        named<MavenPublication>("maven") {
            pom {
                name.set("Encode/Decode Hardware Scan")
                description.set("A library for detecting detailed hardware support for video encoding and decoding")
            }
        }
    }
}

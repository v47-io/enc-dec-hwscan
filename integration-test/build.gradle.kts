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

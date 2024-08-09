plugins {
    kotlin("jvm")
    kotlin("kapt")

    id("io.quarkus.extension")
}

dependencies {
    implementation(platform(libs.quarkus.bom))

    implementation(project(":bindings"))
    implementation(libs.jackson.annotations)
    implementation(libs.quarkus.arc)

    testImplementation(libs.quarkus.junit5)
}

quarkusExtension {
    deploymentModule.set(":deployment")
}

tasks.test {
    useJUnitPlatform()
}

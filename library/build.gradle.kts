plugins {
    kotlin("jvm")
    kotlin("kapt")

    alias(libs.plugins.quarkus.extension)

    alias(libs.plugins.detekt)
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

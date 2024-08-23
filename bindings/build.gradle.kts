plugins {
    kotlin("jvm")
    kotlin("plugin.allopen")

    alias(libs.plugins.jextract)
    alias(libs.plugins.rust.importer)

    alias(libs.plugins.detekt)
    alias(libs.plugins.dokka)

    `maven-publish`
}

dependencies {
    implementation(platform(libs.quarkus.bom))

    rust(project(":native"))

    implementation(libs.quarkus.core)
    implementation(libs.yanl)

    compileOnly(libs.native.image.svm)

    testImplementation(libs.quarkus.junit5)
}

tasks.jextract {
    dependsOn(":native:build")

    header("${project(":native").projectDir}/target/enc-dec-hwscan.h") {
        targetPackage.set("io.v47.encDecHwscan.bindings")
        className.set("EncDecHwscan")
    }
}

allOpen {
    annotation("io.quarkus.runtime.annotations.Recorder")
}

tasks.test {
    useJUnitPlatform()
}

publishing {
    publications {
        named<MavenPublication>("maven") {
            pom {
                name.set("Encode/Decode Hardware Scan :: Bindings")
                description.set("Native bindings and shared library for enc-dec-hwscan")
            }
        }
    }
}

plugins {
    kotlin("jvm")
    kotlin("plugin.allopen")

    id("fr.stardustenterprises.rust.importer")
    id("io.github.krakowski.jextract")
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

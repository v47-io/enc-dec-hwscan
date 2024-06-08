plugins {
    library

    id("fr.stardustenterprises.rust.importer")
    id("io.github.krakowski.jextract")
}

dependencies {
    rust(project(":libs:enc-dec-hwscan:enc-dec-hwscan-native"))

    implementation(Catalog.Yanl)
    implementation(Catalog.Quarkus.Core)

    compileOnly(Catalog.NativeImageSvm)
}

tasks.jextract {
    dependsOn(":libs:enc-dec-hwscan:enc-dec-hwscan-native:build")

    header("${project(":libs:enc-dec-hwscan:enc-dec-hwscan-native").projectDir}/target/enc-dec-hwscan.h") {
        targetPackage.set("io.v47.encDecHwscan.bindings")
        className.set("EncDecHwscan")
    }
}

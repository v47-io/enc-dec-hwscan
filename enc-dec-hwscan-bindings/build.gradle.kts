plugins {
    library

    id("fr.stardustenterprises.rust.importer")
    id("io.github.krakowski.jextract")
}

dependencies {
    rust(project("${parent!!.path}:enc-dec-hwscan-native"))

    implementation(Catalog.Yanl)
    implementation(Catalog.Quarkus.Core)

    compileOnly(Catalog.NativeImageSvm)
}

tasks.jextract {
    dependsOn("${parent!!.path}:enc-dec-hwscan-native:build")

    header("${project("${parent!!.path}:enc-dec-hwscan-native").projectDir}/target/enc-dec-hwscan.h") {
        targetPackage.set("io.v47.encDecHwscan.bindings")
        className.set("EncDecHwscan")
    }
}

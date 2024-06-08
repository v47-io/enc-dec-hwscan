plugins {
    `quarkus-extension`
}

dependencies {
    implementation(project("$path:enc-dec-hwscan-bindings"))
    implementation(Catalog.Quarkus.Arc)
}

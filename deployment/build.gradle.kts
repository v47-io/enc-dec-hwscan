plugins {
    kotlin
}

dependencies {
    implementation(project(parent!!.path))
    implementation(project("${parent!!.path}:enc-dec-hwscan-bindings"))

    implementation(Catalog.Quarkus.Arc.Deployment)
    implementation(Catalog.Quarkus.Core.Deployment)
}

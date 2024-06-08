plugins {
    `quarkus-extension-it`
}

dependencies {
    implementation(Catalog.Quarkus.RestJackson)
    implementation(Catalog.Quarkus.RestKotlin)

    implementation(project(":libs:utils"))
}

import org.jetbrains.kotlin.gradle.internal.Kapt3GradleSubplugin.Companion.getKaptConfigurationName

plugins {
    kotlin("jvm")
    kotlin("kapt")

    alias(libs.plugins.detekt)
    alias(libs.plugins.dokka)

    `maven-publish`
}

configurations.getByName(getKaptConfigurationName(sourceSets.main.name)) {
    extendsFrom(configurations.getByName(JavaPlugin.ANNOTATION_PROCESSOR_CONFIGURATION_NAME))
}

configurations.getByName(getKaptConfigurationName(sourceSets.test.name)) {
    extendsFrom(configurations.getByName(JavaPlugin.TEST_ANNOTATION_PROCESSOR_CONFIGURATION_NAME))
}

dependencies {
    implementation(platform(libs.quarkus.bom))

    implementation(project(":enc-dec-hwscan"))
    implementation(project(":bindings"))

    implementation(libs.quarkus.arc.deployment)
    implementation(libs.quarkus.core.deployment)
}

publishing {
    publications {
        named<MavenPublication>("maven") {
            pom {
                name.set("Encode/Decode Hardware Scan :: Deployment")
                description.set("Quarkus deployment module for enc-dec-hwscan")
            }
        }
    }
}

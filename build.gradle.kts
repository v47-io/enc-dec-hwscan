import nl.javadude.gradle.plugins.license.LicenseExtension
import java.util.Calendar

plugins {
    kotlin("jvm") apply false

    alias(libs.plugins.license) apply false
}

allprojects {
    apply(plugin = "com.github.hierynomus.license")

    extensions.configure<LicenseExtension> {
        excludePatterns = setOf(
            "**/*.gitkeep",
            "**/*.json",
            "**/*.ya?ml",
            "**/bindings/*.java",
            "*.properties",
            "*.txt",
            "META-INF/**/*"
        )

        header = rootProject.file("HEADER.txt")

        skipExistingHeaders = true

        mapping("rs", "SLASHSTAR_STYLE")

        ext {
            set("year", Calendar.getInstance().get(Calendar.YEAR))
        }
    }
}

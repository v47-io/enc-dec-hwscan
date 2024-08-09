import com.hierynomus.gradle.license.tasks.LicenseFormat

plugins {
    id("fr.stardustenterprises.rust.wrapper")
}

rust {
    release.set(true)

    targets += defaultTarget()
}

val licenseFormatRust = tasks.register("licenseFormatRust", LicenseFormat::class.java) {
    group = "license"

    source = fileTree(projectDir) {
        include("**/*.rs")
        exclude("**/target")
    }
}

tasks.licenseFormat {
    dependsOn(licenseFormatRust)
}

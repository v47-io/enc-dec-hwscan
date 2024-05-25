import com.hierynomus.gradle.license.tasks.LicenseFormat
import tasks.resetVersionInCargoTomlTask
import tasks.setVersionInCargoTomlTask

plugins {
    `ms47-base`

    id("fr.stardustenterprises.rust.wrapper")
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

rust {
    release.set(true)

    targets += defaultTarget()
}

val setVersionInCargoTomlTask = tasks.setVersionInCargoTomlTask()
val resetVersionInCargoTomlTask = tasks.resetVersionInCargoTomlTask()

tasks.build {
    dependsOn(setVersionInCargoTomlTask)
    finalizedBy(resetVersionInCargoTomlTask)

    doFirst {
        delete(project.layout.buildDirectory.dir("rust"))
    }
}

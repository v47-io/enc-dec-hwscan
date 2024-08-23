import name.remal.gradle_plugins.dsl.extensions.sourceSets
import name.remal.gradle_plugins.plugins.publish.ossrh.RepositoryHandlerOssrhExtension
import nl.javadude.gradle.plugins.license.LicenseExtension
import org.jetbrains.dokka.DokkaConfiguration
import org.jetbrains.dokka.Platform
import org.jetbrains.dokka.gradle.DokkaTask
import java.util.Calendar

buildscript {
    repositories { mavenCentral() }

    dependencies {
        classpath(libs.remalGradlePlugins)
    }
}

plugins {
    kotlin("jvm") apply false

    alias(libs.plugins.license) apply false
    alias(libs.plugins.dokka) apply false

    alias(libs.plugins.release)
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

    pluginManager.withPlugin(rootProject.libs.plugins.dokka.get().pluginId) {
        tasks.withType<DokkaTask> {
            dokkaSourceSets {
                configureEach {
                    documentedVisibilities.set(setOf(DokkaConfiguration.Visibility.PUBLIC))
                    jdkVersion.set(21)
                    includes.from(project.files(), "packages.md")
                    platform.set(Platform.jvm)
                }
            }
        }
    }

    pluginManager.withPlugin("maven-publish") {
        val dokkaJavadoc = tasks.register<Jar>("dokkaJavadocJar") {
            dependsOn("dokkaJavadoc")
            from(tasks["dokkaJavadoc"].outputs)
            archiveClassifier.set("javadoc")
        }

        val sourcesJar by tasks.registering(Jar::class) {
            archiveClassifier.set("sources")
            from(sourceSets["main"].allSource)
        }

        extensions.configure<PublishingExtension> {
            publications {
                create<MavenPublication>("maven") {
                    groupId = "${project.group}"
                    artifactId = this@allprojects.name
                    version = "${project.version}"

                    from(components["java"])

                    artifact(dokkaJavadoc)
                    artifact(sourcesJar)

                    pom {
                        url.set("https://github.com/v47-io/enc-dec-hwscan")

                        licenses {
                            license {
                                name.set("GNU General Public License version 3")
                                url.set("https://www.gnu.org/licenses/gpl-3.0.en.html")
                            }
                        }

                        developers {
                            developer {
                                id.set("vemilyus")
                                name.set("Alex Katlein")
                                email.set("dev@vemilyus.com")
                                url.set("https://v47.io")
                            }
                        }

                        scm {
                            connection.set("scm:git:git://github.com/v47-io/enc-dec-hwscan.git")
                            developerConnection.set("scm:git:git://github.com/v47-io/enc-dec-hwscan.git")
                            url.set("https://github.com/v47-io/enc-dec-hwscan")
                        }
                    }
                }
            }

            val ossrhUser: String? = project.findProperty("ossrhUser") as? String ?: System.getenv("OSSRH_USER")
            val ossrhPass: String? = project.findProperty("osshrPass") as? String ?: System.getenv("OSSRH_PASS")

            if (
                !ossrhUser.isNullOrBlank() &&
                !ossrhPass.isNullOrBlank() &&
                !"${project.version}".endsWith("-SNAPSHOT")
            ) {
                this@allprojects.apply(plugin = "signing")
                this@allprojects.apply(plugin = "name.remal.maven-publish-ossrh")

                repositories {
                    @Suppress("DEPRECATION")
                    withConvention(RepositoryHandlerOssrhExtension::class) {
                        ossrh {
                            credentials {
                                username = ossrhUser
                                password = ossrhPass
                            }
                        }
                    }
                }
            }
        }
    }
}

release {
    tagTemplate.set("v\$version")
}

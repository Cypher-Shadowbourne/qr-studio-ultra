buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:8.11.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.25")
    }
}

allprojects {
    repositories {
        google()
        mavenCentral()
    }

    project.plugins.withId("com.android.library") {
        if (!project.plugins.hasPlugin("org.jetbrains.kotlin.android")) {
            project.plugins.apply("org.jetbrains.kotlin.android")
        }
    }

    project.afterEvaluate {
        tasks.matching { it.name.contains("Kotlin") }.configureEach {
            try {
                val options = this.javaClass.getMethod("getKotlinOptions").invoke(this)
                options.javaClass.getMethod("setJvmTarget", String::class.java).invoke(options, "1.8")
            } catch (e: Exception) {
                // Ignore if not a Kotlin task
            }
        }
    }
}

tasks.register("clean").configure {
    delete("build")
}


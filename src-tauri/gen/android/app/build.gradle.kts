import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

val tauriProperties = Properties().apply {
    val propFile = file("tauri.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}

val signingProperties = Properties().apply {
    val candidateFiles = listOf(
        rootProject.file("key.properties"),
        rootProject.file("local.properties"),
        rootProject.file("../../../local.properties"),
        rootProject.file("../../../keystore.properties")
    )
    candidateFiles.firstOrNull { it.exists() }?.inputStream()?.use { load(it) }
}

fun signingValue(envName: String, propertyName: String): String? =
    System.getenv(envName)?.takeIf { it.isNotBlank() }
        ?: signingProperties.getProperty(propertyName)?.takeIf { it.isNotBlank() }

android {
    compileSdk = 36
    namespace = "com.cypher.qrstudioultra"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        applicationId = "com.cypher.qrstudioultra"
        minSdk = 24
        targetSdk = 36
        versionCode = tauriProperties.getProperty("tauri.android.versionCode", "1").toInt()
        versionName = tauriProperties.getProperty("tauri.android.versionName", "1.0")
    }
    signingConfigs {
        create("release") {
            val keystoreFile = rootProject.file("../../../my-release-key.jks")
            if (keystoreFile.exists()) {
                val resolvedStorePassword = signingValue("ANDROID_KEYSTORE_PASS", "storePassword")
                val resolvedKeyAlias = signingValue("ANDROID_KEY_ALIAS", "keyAlias") ?: "qrstudio"
                val resolvedKeyPassword = signingValue("ANDROID_KEY_PASS", "keyPassword")

                check(!resolvedStorePassword.isNullOrBlank()) {
                    "Release signing is configured with ${keystoreFile.name}, but no store password was found. " +
                        "Set ANDROID_KEYSTORE_PASS or add storePassword to key.properties/local.properties."
                }
                check(!resolvedKeyPassword.isNullOrBlank()) {
                    "Release signing is configured with ${keystoreFile.name}, but no key password was found. " +
                        "Set ANDROID_KEY_PASS or add keyPassword to key.properties/local.properties."
                }

                storeFile = keystoreFile
                storePassword = resolvedStorePassword
                keyAlias = resolvedKeyAlias
                keyPassword = resolvedKeyPassword
            }
        }
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
            signingConfig = signingConfigs.getByName("release")
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        buildConfig = true
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("androidx.print:print:1.1.0")
    implementation("com.google.android.material:material:1.12.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")

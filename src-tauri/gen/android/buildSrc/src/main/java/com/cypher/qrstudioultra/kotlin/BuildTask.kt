import java.io.File
import org.apache.tools.ant.taskdefs.condition.Os
import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.logging.LogLevel
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction

open class BuildTask : DefaultTask() {
    @Input
    var rootDirRel: String? = null
    @Input
    var target: String? = null
    @Input
    var release: Boolean? = null

    @TaskAction
    fun assemble() {
        val rootDirRel = rootDirRel ?: throw GradleException("rootDirRel cannot be null")
        val target = target ?: throw GradleException("target cannot be null")
        val release = release ?: throw GradleException("release cannot be null")

        val tauriArgs = mutableListOf("tauri", "android", "android-studio-script")

        if (project.logger.isEnabled(LogLevel.DEBUG)) {
            tauriArgs.add("-vv")
        } else if (project.logger.isEnabled(LogLevel.INFO)) {
            tauriArgs.add("-v")
        }

        if (release) {
            tauriArgs.add("--release")
        }

        tauriArgs.add("--target")
        tauriArgs.add(target)

        project.exec {
            workingDir(File(project.projectDir, rootDirRel))

            if (Os.isFamily(Os.FAMILY_WINDOWS)) {
                // Using 'cmd /c npx' is more stable on Windows than calling 'npx.cmd' directly
                // and avoids the -1073740791 (0xC0000409) exit code error.
                executable("cmd")
                args("/c", "npx")
            } else {
                executable("npx")
            }
            args(tauriArgs)

            // Skip unnecessary checks that might fail in restricted environments
            environment("TAURI_SKIP_UPDATE_CHECK", "true")
            environment("TAURI_OFFLINE", "true")
            // Avoid WebSocket initialization
            environment("TAURI_WEBVIEW_DEBUG_PORT", "")
            // Ensure no dev server is expected
            environment("TAURI_DEV_HOST", "")
            // Use local IP for WebSocket just in case it still tries to connect
            environment("TAURI_DEV_URL", "http://127.0.0.1:0")
        }.assertNormalExitValue()
    }
}

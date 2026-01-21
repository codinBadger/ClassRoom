use std::process::{Command, Stdio};
use std::time::Instant;
use crate::models::ExecutionResult;

// SECURITY WARNING: This code executor runs user-submitted code directly on the server.
// In a production environment, you MUST implement proper sandboxing such as:
// - Docker containers with resource limits
// - Virtual machines
// - Specialized sandboxing solutions (e.g., gVisor, Firecracker)
// - Network isolation
// - File system restrictions
// - Time limits and resource quotas
// Current implementation is for demonstration purposes only.

pub async fn execute_code(language: &str, code: &str) -> ExecutionResult {
    let start = Instant::now();
    
    let result = match language.to_lowercase().as_str() {
        "python" | "python3" => execute_python(code),
        "javascript" | "js" | "node" => execute_javascript(code),
        "rust" => execute_rust(code),
        "c++" | "cpp" => execute_cpp(code),
        "java" => execute_java(code),
        _ => return ExecutionResult {
            output: String::new(),
            execution_time_ms: 0,
            success: false,
            error: Some(format!("Unsupported language: {}", language)),
        },
    };

    let execution_time_ms = start.elapsed().as_millis();

    match result {
        Ok(output) => ExecutionResult {
            output,
            execution_time_ms,
            success: true,
            error: None,
        },
        Err(error) => ExecutionResult {
            output: String::new(),
            execution_time_ms,
            success: false,
            error: Some(error),
        },
    }
}

fn execute_python(code: &str) -> Result<String, String> {
    let child = Command::new("python3")
        .arg("-c")
        .arg(code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute Python: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for Python: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn execute_javascript(code: &str) -> Result<String, String> {
    let child = Command::new("node")
        .arg("-e")
        .arg(code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute JavaScript: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for JavaScript: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn execute_rust(code: &str) -> Result<String, String> {
    use std::fs;
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let file_name = format!("/tmp/rust_{}.rs", id);
    
    fs::write(&file_name, code)
        .map_err(|e| format!("Failed to write Rust file: {}", e))?;

    let output = Command::new("rustc")
        .arg(&file_name)
        .arg("-o")
        .arg(format!("/tmp/rust_{}", id))
        .output()
        .map_err(|e| format!("Failed to compile Rust: {}", e))?;

    if !output.status.success() {
        fs::remove_file(&file_name).ok();
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let exec_output = Command::new(format!("/tmp/rust_{}", id))
        .output()
        .map_err(|e| format!("Failed to execute Rust: {}", e))?;

    // Cleanup
    fs::remove_file(&file_name).ok();
    fs::remove_file(format!("/tmp/rust_{}", id)).ok();

    if exec_output.status.success() {
        Ok(String::from_utf8_lossy(&exec_output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&exec_output.stderr).to_string())
    }
}

fn execute_cpp(code: &str) -> Result<String, String> {
    use std::fs;
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let file_name = format!("/tmp/cpp_{}.cpp", id);
    
    fs::write(&file_name, code)
        .map_err(|e| format!("Failed to write C++ file: {}", e))?;

    let output = Command::new("g++")
        .arg(&file_name)
        .arg("-o")
        .arg(format!("/tmp/cpp_{}", id))
        .output()
        .map_err(|e| format!("Failed to compile C++: {}", e))?;

    if !output.status.success() {
        fs::remove_file(&file_name).ok();
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let exec_output = Command::new(format!("/tmp/cpp_{}", id))
        .output()
        .map_err(|e| format!("Failed to execute C++: {}", e))?;

    // Cleanup
    fs::remove_file(&file_name).ok();
    fs::remove_file(format!("/tmp/cpp_{}", id)).ok();

    if exec_output.status.success() {
        Ok(String::from_utf8_lossy(&exec_output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&exec_output.stderr).to_string())
    }
}

fn execute_java(code: &str) -> Result<String, String> {
    use std::fs;
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let class_name = format!("Main_{}", id.to_string().replace("-", "_"));
    let file_name = format!("/tmp/{}.java", class_name);
    
    // Replace "public class Main" with our generated class name
    let modified_code = code.replace("class Main", &format!("class {}", class_name));
    
    fs::write(&file_name, modified_code)
        .map_err(|e| format!("Failed to write Java file: {}", e))?;

    let output = Command::new("javac")
        .arg(&file_name)
        .current_dir("/tmp")
        .output()
        .map_err(|e| format!("Failed to compile Java: {}", e))?;

    if !output.status.success() {
        fs::remove_file(&file_name).ok();
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let exec_output = Command::new("java")
        .arg(&class_name)
        .current_dir("/tmp")
        .output()
        .map_err(|e| format!("Failed to execute Java: {}", e))?;

    // Cleanup
    fs::remove_file(&file_name).ok();
    fs::remove_file(format!("/tmp/{}.class", class_name)).ok();

    if exec_output.status.success() {
        Ok(String::from_utf8_lossy(&exec_output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&exec_output.stderr).to_string())
    }
}

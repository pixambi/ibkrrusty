use ibkrrusty::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Some("https://localhost:5000/v1/api/"))?;

    println!("🚀 Starting IBKR session management example");

    println!("\n📊 Checking authentication status...");
    match client.auth_status().await {
        Ok(status) => {
            println!("✅ Authentication Status:");
            println!("  - Authenticated: {}", status.authenticated);
            println!("  - Connected: {}", status.connected);
            println!("  - Competing: {}", status.competing);
            println!("  - Message: {}", status.message);
            if let Some(server_info) = &status.server_info {
                println!("  - Server: {} ({})", server_info.server_name, server_info.server_version);
            }
        }
        Err(e) => {
            println!("❌ Failed to get auth status: {:?}", e);
            println!("💡 Make sure IBKR Client Portal Gateway is running on localhost:5000");
            return Ok(());
        }
    }

    println!("\n🔐 Initializing brokerage session...");
    match client.init_session(true).await {
        Ok(init_response) => {
            println!("✅ Session Initialized:");
            println!("  - Authenticated: {}", init_response.authenticated);
            println!("  - Connected: {}", init_response.connected);
            println!("  - Competing: {}", init_response.competing);
        }
        Err(e) => {
            println!("❌ Failed to initialize session: {:?}", e);
        }
    }

    println!("\n📈 Initializing HMDS endpoints...");
    match client.init_hmds().await {
        Ok(hmds_response) => {
            println!("✅ HMDS Initialized:");
            println!("  - Authenticated: {}", hmds_response.authenticated);
        }
        Err(e) => {
            println!("⚠️  HMDS initialization failed: {:?}", e);
            println!("   This might be normal if historical data is not needed");
        }
    }

    println!("\n🔍 Validating SSO session...");
    match client.validate_sso().await {
        Ok(sso_response) => {
            println!("✅ SSO Validation:");
            println!("  - Result: {}", sso_response.result);
            println!("  - User: {}", sso_response.user_name);
            println!("  - Login Type: {} (1=Live, 2=Paper)", sso_response.login_type);
            println!("  - Is Master Account: {}", sso_response.is_master);
            println!("  - Expires in: {}ms", sso_response.expires);
            if let Some(features) = &sso_response.features {
                println!("  - Features: Bond={}, Options={}, Realtime={}",
                         features.bond, features.option_chains, features.realtime);
            }
        }
        Err(e) => {
            println!("⚠️  SSO validation failed: {:?}", e);
            println!("   This might be normal if not using OAuth");
        }
    }

    println!("\n❤️  Sending keepalive ping...");
    match client.tickle().await {
        Ok(tickle_response) => {
            println!("✅ Tickle Response:");
            println!("  - Session ID: {}", tickle_response.session);
            println!("  - SSO Expires: {}ms", tickle_response.sso_expires);
            println!("  - User ID: {}", tickle_response.user_id);

            if let Some(iserver) = &tickle_response.iserver {
                println!("  - IServer Status: authenticated={}",
                         iserver.auth_status.authenticated);
            }

            if let Some(hmds) = &tickle_response.hmds {
                if let Some(error) = &hmds.error {
                    println!("  - HMDS Status: {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Tickle failed: {:?}", e);
        }
    }

    println!("\n⏰ Demonstrating session keepalive pattern...");
    println!("In a real application, you would call tickle() every 60 seconds");

    for i in 1..=3 {
        println!("  Keepalive #{}", i);
        sleep(Duration::from_secs(2)).await; // Simulate work

        match client.tickle().await {
            Ok(_) => println!("    ✅ Session maintained"),
            Err(e) => println!("    ❌ Keepalive failed: {:?}", e),
        }
    }


    println!("\n🎉 Session management example completed!");
    println!("\n💡 Next steps:");
    println!("   - Implement account endpoints for portfolio data");
    println!("   - Add market data endpoints for real-time quotes");
    println!("   - Implement trading endpoints for order management");
    println!("   - Set up proper session keepalive in your application");

    Ok(())
}

#[allow(dead_code)]
async fn session_keepalive_manager(client: Client, interval_seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = tokio::time::interval(Duration::from_secs(interval_seconds));

    loop {
        interval.tick().await;

        match client.tickle().await {
            Ok(response) => {
                println!("✅ Session keepalive successful. Expires in: {}ms", response.sso_expires);

                if response.sso_expires < 300_000 {
                    println!("⚠️  Session expiring soon! Consider reauthenticating.");
                }
            }
            Err(e) => {
                println!("❌ Session keepalive failed: {:?}", e);
                println!("🔄 Attempting to reinitialize session...");

                match client.init_session(true).await {
                    Ok(_) => println!("✅ Session reinitialized successfully"),
                    Err(e) => {
                        println!("❌ Failed to reinitialize session: {:?}", e);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
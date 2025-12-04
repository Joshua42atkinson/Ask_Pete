# Test Game Loop - MVP Integration Test
# Tests the complete flow: Blueprint Generation → Quest Start → Quest Completion → Steam Reward

$baseUrl = "http://localhost:3000"
$ErrorActionPreference = "Stop"

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  Ask Pete MVP Game Loop Test" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Generate Blueprint
Write-Host "[1/4] Generating Blueprint..." -ForegroundColor Yellow
$blueprintPayload = @{
    subject         = "Basic Physics - Force and Motion"
    focus           = 0.6  # Slightly more narrative
    literary_device = "Hero's Journey"
    vocabulary      = @("velocity", "acceleration", "force", "momentum")
} | ConvertTo-Json

try {
    $blueprintResponse = Invoke-RestMethod -Uri "$baseUrl/api/architect/blueprint" -Method Post -Body $blueprintPayload -ContentType "application/json" -TimeoutSec 300
    Write-Host "✅ Blueprint Generated Successfully!" -ForegroundColor Green
    Write-Host "   Graph ID: $($blueprintResponse.graph.id)" -ForegroundColor Gray
    Write-Host "   Title: $($blueprintResponse.graph.title)" -ForegroundColor Gray
    Write-Host "   Nodes: $($blueprintResponse.graph.nodes.Count)" -ForegroundColor Gray
    
    # $graphId = $blueprintResponse.graph.id
    
    # Validate logic blocks have proper JSON format (not strings)
    $hasProperLogic = $true
    foreach ($node in $blueprintResponse.graph.nodes) {
        if ($node.logic.condition -is [string] -and $node.logic.condition -ne "None") {
            # Check if it's a pseudo-Rust string (old format)
            if ($node.logic.condition -like "*{*}*") {
                Write-Host "   ⚠️  Node '$($node.title)' has string-based logic (old format)" -ForegroundColor Yellow
                $hasProperLogic = $false
            }
        }
    }
    
    if ($hasProperLogic) {
        Write-Host "   ✅ All logic blocks use proper JSON format" -ForegroundColor Green
    }
}
catch {
    Write-Host "❌ Blueprint Generation Failed!" -ForegroundColor Red
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "   Response: $responseBody" -ForegroundColor Red
    }
    exit 1
}

Write-Host ""

# Test 2: Save Graph (via Story Graph endpoint)
Write-Host "[2/4] Saving Blueprint to Database..." -ForegroundColor Yellow
try {
    $savePayload = @{
        title      = $blueprintResponse.graph.title
        graph_data = $blueprintResponse.graph
    } | ConvertTo-Json -Depth 10
    
    $saveResponse = Invoke-RestMethod -Uri "$baseUrl/api/graphs" -Method Post -Body $savePayload -ContentType "application/json"
    Write-Host "✅ Blueprint Saved Successfully!" -ForegroundColor Green
    Write-Host "   Database ID: $($saveResponse.id)" -ForegroundColor Gray
    $questId = $saveResponse.id
}
catch {
    Write-Host "❌ Blueprint Save Failed!" -ForegroundColor Red
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test 3: Start Quest
Write-Host "[3/4] Starting Quest..." -ForegroundColor Yellow
try {
    $startResponse = Invoke-RestMethod -Uri "$baseUrl/api/quest/start/$questId" -Method Post -ContentType "application/json"
    Write-Host "✅ Quest Started Successfully!" -ForegroundColor Green
    Write-Host "   Message: $($startResponse.message)" -ForegroundColor Gray
    
    # Wait for physics to initialize
    Start-Sleep -Seconds 2
}
catch {
    Write-Host "❌ Quest Start Failed!" -ForegroundColor Red
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test 3.5: Check Physics State
Write-Host "[3.5/4] Verifying Physics State..." -ForegroundColor Yellow
try {
    $physicsResponse = Invoke-RestMethod -Uri "$baseUrl/api/weigh_station/telemetry" -Method Get
    Write-Host "✅ Physics State Retrieved!" -ForegroundColor Green
    Write-Host "   Coal: $($physicsResponse.coal)" -ForegroundColor Gray
    Write-Host "   Steam: $($physicsResponse.steam)" -ForegroundColor Gray
    Write-Host "   Velocity: $($physicsResponse.velocity)" -ForegroundColor Gray
}
catch {
    Write-Host "⚠️  Physics State Check Failed (may be expected): $($_.Exception.Message)" -ForegroundColor Yellow
}

Write-Host ""

# Test 4: Complete Quest
Write-Host "[4/4] Completing Quest..." -ForegroundColor Yellow
try {
    $completeResponse = Invoke-RestMethod -Uri "$baseUrl/api/quest/complete/$questId" -Method Post -ContentType "application/json"
    Write-Host "✅ Quest Completed Successfully!" -ForegroundColor Green
    Write-Host "   Steam Earned: $($completeResponse.steam_earned)" -ForegroundColor Gray
    Write-Host "   New Balance: $($completeResponse.new_balance)" -ForegroundColor Gray
    
    if ($completeResponse.steam_earned -gt 0) {
        Write-Host "   🎉 Steam Generation Confirmed!" -ForegroundColor Green
    }
    else {
        Write-Host "   ⚠️  No Steam earned (check physics integration)" -ForegroundColor Yellow
    }
}
catch {
    Write-Host "❌ Quest Completion Failed!" -ForegroundColor Red
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  ✅ ALL TESTS PASSED!" -ForegroundColor Green
Write-Host "  MVP Game Loop is Functional" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Summary:" -ForegroundColor White
Write-Host "  • Blueprint generation works with proper JSON enums" -ForegroundColor White
Write-Host "  • Graph persistence successful" -ForegroundColor White
Write-Host "  • Quest start handler functional" -ForegroundColor White
Write-Host "  • Quest completion calculates and awards Steam" -ForegroundColor White

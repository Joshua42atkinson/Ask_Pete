# Test Game Loop APIs

$baseUrl = "http://localhost:3000"

# 1. Generate Blueprint
Write-Host "1. Generating Blueprint..."
$blueprintPayload = @{
    subject         = "The Physics of Steam Engines"
    focus           = 0.5
    literary_device = "Hero's Journey"
    vocabulary      = @("pressure", "volume", "temperature")
} | ConvertTo-Json

try {
    $blueprintResponse = Invoke-RestMethod -Uri "$baseUrl/api/architect/blueprint" -Method Post -Body $blueprintPayload -ContentType "application/json"
    Write-Host "✅ Blueprint Generated!"
    Write-Host "Graph ID: $($blueprintResponse.graph.id)"
    $graphId = $blueprintResponse.graph.id
}
catch {
    Write-Host "❌ Blueprint Generation Failed: $($_.Exception.Message)"
    exit 1
}

# 2. Start Quest (Simulated)
# Note: The actual endpoint might be /api/quest/start or similar. Checking routes...
# Based on main.rs, we have `crate::routes::story_graphs::story_graph_routes`.
# We might need to just "load" the graph to play it.

Write-Host "2. Verifying Graph Persistence..."
try {
    $graphResponse = Invoke-RestMethod -Uri "$baseUrl/api/graphs/$graphId" -Method Get
    if ($graphResponse.id -eq $graphId) {
        Write-Host "✅ Graph Persisted Successfully!"
    }
    else {
        Write-Host "❌ Graph ID mismatch."
    }
}
catch {
    Write-Host "❌ Failed to fetch graph: $($_.Exception.Message)"
}

# 3. Check Weigh Station (Telemetry)
Write-Host "3. Checking Weigh Station..."
try {
    $weighStation = Invoke-RestMethod -Uri "$baseUrl/api/weigh_station/telemetry" -Method Get
    Write-Host "✅ Weigh Station Online!"
    Write-Host "Steam: $($weighStation.steam)"
}
catch {
    Write-Host "⚠️ Weigh Station Check Failed (Might be expected if no active session): $($_.Exception.Message)"
}

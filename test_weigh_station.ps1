$ErrorActionPreference = "Stop"

function Test-WeighStation {
    Write-Host "🧪 Testing Weigh Station..." -ForegroundColor Cyan

    # 1. Define Payload (Node with long text to trigger AI)
    $payload = @{
        id          = "test_graph_weigh"
        title       = "Weigh Station Test"
        nodes       = @(
            @{
                id               = "node_heavy"
                title            = "Heavy Node"
                content          = "The concept of cognitive load theory suggests that the human brain has a limited capacity for processing information in working memory. When this capacity is exceeded, learning is hampered. Therefore, instructional design must manage intrinsic, extraneous, and germane load effectively."
                x                = 0
                y                = 0
                passenger_count  = 0
                complexity_level = 1
                learner_profiles = @()
                gardens_active   = @()
                required_stats   = @{}
                logic            = @{
                    condition = "None"
                    effect    = "None"
                }
                style            = @{}
                quest            = $null
                mass             = $null # Explicitly null to trigger weighing
                analysis_hash    = $null
            }
        )
        connections = @()
    } | ConvertTo-Json -Depth 10

    # 2. Send to Server
    Write-Host "Sending Graph to Server..."
    try {
        $response = Invoke-RestMethod -Uri "http://localhost:3000/api/expert/graph" -Method Post -Body $payload -ContentType "application/json"
        Write-Host "✅ Graph Saved." -ForegroundColor Green
    }
    catch {
        Write-Error "Failed to save graph: $_"
    }

    # 3. Wait for Weighing (Server logs should show activity)
    Write-Host "⏳ Waiting 10 seconds for Weigh Station..."
    Start-Sleep -Seconds 10

    # 4. Verify (We can't easily check the server memory state via API yet without a GET endpoint that returns mass)
    # But we can check if the server is still alive and responsive.
    
    Write-Host "✅ Test Request Sent. Check server logs for 'Weigh Station: Node weighed'." -ForegroundColor Yellow
}

Test-WeighStation

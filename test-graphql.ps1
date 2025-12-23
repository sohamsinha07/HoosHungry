# PowerShell helper function for GraphQL queries
function Invoke-GraphQL {
    param(
        [string]$Uri = "http://localhost:8080/graphql",
        [string]$Query,
        [hashtable]$Variables = @{}
    )
    
    $body = @{
        query = $Query
    }
    
    if ($Variables.Count -gt 0) {
        $body.variables = $Variables
    }
    
    $jsonBody = $body | ConvertTo-Json -Depth 10 -Compress
    
    try {
        $response = Invoke-RestMethod -Uri $Uri -Method Post -ContentType "application/json" -Body $jsonBody
        return $response | ConvertTo-Json -Depth 10
    }
    catch {
        Write-Error "GraphQL request failed: $_"
        return $null
    }
}

# Example usage:
# Invoke-GraphQL -Query "{ diningHalls { id name } }"
# 
# Or with variables:
# $vars = @{
#     hallId = 1
#     limit = 10
#     prefs = @{
#         veganOnly = $false
#         maxCalories = 700
#     }
# }
# Invoke-GraphQL -Query "query Recommend(`$hallId: Int!) { ... }" -Variables $vars


$ErrorActionPreference = "Stop"

$ModelDir = "models"
$ModelUrl = "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf"
$TokenizerUrl = "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/tokenizer.json"

# Create models directory if it doesn't exist
if (-not (Test-Path -Path $ModelDir)) {
    New-Item -ItemType Directory -Path $ModelDir | Out-Null
    Write-Host "Created directory: $ModelDir"
}

# Download Model
$ModelFile = Join-Path $ModelDir "Llama-3.2-3B-Instruct-Q4_K_M.gguf"
if (-not (Test-Path -Path $ModelFile)) {
    Write-Host "Downloading model from $ModelUrl..."
    Invoke-WebRequest -Uri $ModelUrl -OutFile $ModelFile
    Write-Host "Model downloaded to $ModelFile"
}
else {
    Write-Host "Model file already exists: $ModelFile"
}

# Download Tokenizer
# Note: Bartowski's repo might not have tokenizer.json directly at root or it might be named differently.
# Usually GGUF repos don't always have the tokenizer.json. 
# We should check if we can get it from the original repo if missing, but let's try Bartowski's first.
# If this fails, we might need to point to the original meta-llama/Llama-3.2-3B-Instruct repo (requires auth usually) 
# or a mirror. 
# Let's try to get it from a reliable source if Bartowski doesn't have it.
# Actually, GGUF models often embed the tokenizer, but Candle often needs `tokenizer.json` for the `Tokenizer` struct.
# Let's check if we can download it from a public non-gated repo.
# `unsloth/Llama-3.2-3B-Instruct` is often open.

$TokenizerUrl = "https://huggingface.co/unsloth/Llama-3.2-3B-Instruct/resolve/main/tokenizer.json"
$TokenizerFile = Join-Path $ModelDir "tokenizer.json"

if (-not (Test-Path -Path $TokenizerFile)) {
    Write-Host "Downloading tokenizer from $TokenizerUrl..."
    try {
        Invoke-WebRequest -Uri $TokenizerUrl -OutFile $TokenizerFile
        Write-Host "Tokenizer downloaded to $TokenizerFile"
    }
    catch {
        Write-Warning "Failed to download tokenizer. You may need to manually download 'tokenizer.json' from Hugging Face."
    }
}
else {
    Write-Host "Tokenizer file already exists: $TokenizerFile"
}

Write-Host "Download complete. You can now run the AI tests."

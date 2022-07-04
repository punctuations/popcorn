
#######################################
# popcorn command-line installation #
#######################################

# Get latest version of instl
$releases = "https://api.github.com/repos/punctuations/popcorn/releases"
$tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name

# Set path variables
$download = "https://github.com/punctuations/popcorn/releases/download/$tag/popcorn-windows-latest.zip" # Download URL
$name = "popcorn-windows-latest" # Name of saved file
$zip = "$env:TEMP\$name-$tag.zip" # Path where the download will be stored

# Download latest release of instl
Invoke-WebRequest $download -Out $zip # Download latest version of instl
Expand-Archive $zip -DestinationPath "$Env:TEMP\popcorn-download" -Force # Unpack the download

# Move to ~/popcorn
New-Item $Env:TEMP\popcorn-download -ItemType Directory -Force
cd $Env:TEMP\popcorn-download if ($?) { move popcorn.exe "$HOME\.popcorn\" }

# Remove instl and temorary files
Remove-Item "$Env:TEMP\popcorn-download" -Recurse -Force -ErrorAction SilentlyContinue # Remove popcorn
Remove-Item "$name.zip" -Recurse -Force -ErrorAction SilentlyContinue # Remove the zip file

# Test to make sure that everything worked!
if ([System.IO.Directory]::Exists("$HOME\.popcorn")) {
    # Update system path
    [Environment]::SetEnvironmentVariable("PATH", "$env:PATH;$HOME\.popcorn", "User")
    echo "[38;5;10m 🍿  Popcorn successfully installed, please restart terminal for changes to apply. [0;0m"
} else {
    echo "[38;5;1m error - [0;0m something went wrong while installing."
}
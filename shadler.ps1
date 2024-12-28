Write-Host "Script is running on PowerShell $($PSVersionTable.PSVersion)"
Write-Host "Current execution policy is $(Get-ExecutionPolicy)"

$global:ANIME_QUERY_VARS = "{%22search%22:{%22query%22:%22#QUERY#%22,%22allowAdult%22:false,%22allowUnknown%22:false},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}"
$global:ANIME_STREAM_VARS = "{%22showId%22:%22#ANIME_ID#%22,%22translationType%22:%22sub%22,%22episodeString%22:%22#EPISODE#%22r}"
$global:ANIME_QUERY_HASH = "06327bc10dd682e1ee7e07b6db9c16e9ad2fd56c1b769e47513128cd5c9fc77a"
$global:ANIME_STREAM_HASH = "5f1a64b73793cc2234a389cf3a8f93ad82de7043017dd551f38f65b89daa65e0"
$global:ANIME_DETAIL_HASH = "9d7439c90f203e534ca778c4901f9aa2d3ad42c06243ab2c5e6b79612af32028"

$global:MANGA_QUERY_VARS = '{%22search%22:{%22query%22:%22#QUERY#%22,%22isManga%22:true},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}'
$global:MANGA_READ_VARS = '{%22mangaId%22:%22#MANGA_ID#%22,%22translationType%22:%22sub%22,%22chapterString%22:%22#CHAPTER#%22,%22limit%22:10,%22offset%22:0}'
$global:MANGA_QUERY_HASH = "a27e57ef5de5bae714db701fb7b5cf57e13d57938fc6256f7d5c70a975d11f3d"
$global:MANGA_DETAIL_HASH = 'a42e1106694628f5e4eaecd8d7ce0c73895a22a3c905c29836e2c220cf26e55f'
$global:MANGA_READ_HASH="121996b57011b69386b65ca8fc9e202046fc20bf68b8c8128de0d0e92a681195"

$global:DETAIL_VARS = "{%22_id%22:%22#DEATH#%22}"
$global:API_EXT = '{%22persistedQuery%22:{%22version%22:1,%22sha256Hash%22:%22#HASH#%22}}'
$global:RETURN_VALUE

function Show-ShadlerHelp {
    Write-Host @"
Usage: shadler <subcommand> [options]

Example: shadler anime -v -r 8 -s -q "oshi no ko"
         shadler manga -r 10 12 -s -q "kaoru hana wa rin to saku"

Options:

    -q | --query <keywords>         Search anime or manga with specified keywords. Please quote the keywords
    -v | --vlc                      Stream using VLC media player
    -o | --output                   Write downloaded file to a specified directory. Only works on anime subcommand
    -n | --nextplay                 Stream using NextPlayer. Available only on Android
    -s | --stream                   Stream anime episode or read manga chapters with online
    -d | --download                 Download anime episode or download manga chapters for offline reading
    -r | --range <lower> <upper>    Specify episode/chapter range
"@

}

function Validate-Integer {
    Param(
    [Parameter(Mandatory=$true)][int] $InputLower,
    [Parameter(Mandatory=$false)][int] $InputUpper,
    [Parameter(Mandatory=$true)][int] $RangeLower,
    [Parameter(Mandatory=$true)][int] $RangeUpper
    )

    if (! $InputUpper) {

        try {
            $ActuallyInputLower = [int]$InputLower

        } catch {
            return 1

        }

        if (($InputLower -lt $RangeLower) || ($InputLower -gt $RangeUpper)) {
            return 2

        }

    } elseif ($InputUpper){

        try {
            $ActuallyInputLower = [int]$InputLower
            $ActuallyInputUpper = [int]$InputUpper

        } catch {
            return 1

        }

        if (($InputLower -gt $InputUpper) || ($InputUpper -lt $InputLower) || ($InputLower -lt $RangeLower) || ($InputUpper -gt $RangeUpper)) {
            return 2

        }

    }

}

function Prompt-Integer {
    Param(
    [Parameter(Mandatory=$true)][string] $Prompt,
    [Parameter(Mandatory=$true)][string] $OORMessage,
    [Parameter(Mandatory=$true)][int] $RangeLower,
    [Parameter(Mandatory=$true)][int] $RangeUpper,
    [Parameter(Mandatory=$false)][int] $SaveData
    )

    while ($true) {

        Write-Host -NoNewline -ForegroundColor Magenta $Prompt
	$RawInputRange = Read-Host
	$InputRange = $RawInputRange.split(" ")

        if ($InputRange[0] -eq "CURR") {
            $InputRange[0] = $SaveData

        }

        if (! $InputRange[0]) {
            Write-Host -ForegroundColor Red "Error: Input cannot be empty"
            continue

        } else {
            $ReturnCode = Validate-Integer -InputLower $InputRange[0] -InputUpper $InputRange[1] -RangeLower $RangeLower -RangeUpper $RangeUpper

        }

        if ($ReturnCode -eq 1) {
            Write-Host -ForegroundColor Red "Error: Invalid number"
            continue

        } elseif ($ReturnCode -eq 2) {
            Write-Host -ForegroundColor Red $OORMessage
            continue

        }

        break

    }

    $global:RETURN_VALUE = "$InputRange[0] $InputRange[1]"

}

function Get-QueryURL {
    Param(
    [Parameter(Mandatory=$true)][string] $SearchQuery,
    [Parameter(Mandatory=$true)][string] $QueryType
    )

    $QueryString = $SearchQuery.Replace(" ", "%20")

    if ($QueryType -eq "anime") {
        $QueryURL = $global:ANIME_QUERY_VARS.Replace("#QUERY#", $QueryString)
        $ExtURL = $global:API_EXT.Replace("#HASH#", $global:ANIME_QUERY_HASH)

    } elseif ($QueryType -eq "manga") {
        $QueryURL = $global:MANGA_QUERY_VARS.Replace("#QUERY#", $QueryString)
        $ExtURL = $global:API_EXT.Replace("#HASH#", $global:MANGA_QUERY_HASH)

    }

    $global:RETURN_VALUE = "https://api.allanime.day/api?variables={0}&extensions={1}" -f $QueryURL, $ExtURL

}
cd ..\
cargo build --release
cd performance_testing

function Invoke-LLL {
  param (
      $TextFile, $RepeatedText, $NumChars
  )
  Write-Output "LLL"
  Write-Output $TextFile
  Write-Output $RepeatedText
  Write-Output $NumChars
  Get-Content -Raw -Path $TextFile   | ../target/release/lll.exe --pattern $RepeatedText --before 100 --after 100 > tmp\matches.lll.$NumChars.txt
  # Get-Content -Raw -Path tmp\large_file.1000000.txt |  ..\target\release\lll.exe --before 100 --after 100 --pattern NhzvRcmLtIOtWgrpJPDcBHxS6E9PcLxFgXfWuOcTpPj4KLG9sCD5n10oSqIXGuAbSVdOts5Iig8xC7loejWL427M6AtG5pd5Xxhi > tmp\matches.lll.1000000.txt
}
function Invoke-RG {
  param (
      $TextFile, $RepeatedText, $NumChars
  )
  Write-Output "RG"
  Write-Output $TextFile
  Write-Output $RepeatedText
  Write-Output $NumChars
  Get-Content -Raw -Path $TextFile  | rg --only-matching ".{0,100}$RepeatedText.{0,100}" > tmp\matches.rg.$NumChars.txt
  # Get-Content -Raw -Path tmp\large_file.1000000.txt  | rg --only-matching ".{0,100}NhzvRcmLtIOtWgrpJPDcBHxS6E9PcLxFgXfWuOcTpPj4KLG9sCD5n10oSqIXGuAbSVdOts5Iig8xC7loejWL427M6AtG5pd5Xxhi.{0,100}" > tmp\matches.rg.1000000.txt
}

if (Test-Path "../target/release/lll.exe" -PathType leaf) {
	#      1,000,000
	#     10,000,000
	#    100,000,000
	#  1,000,000,000
	#foreach ($num_chars in 1000000, 10000000, 100000000, 1000000000) {
	foreach ($num_chars in 1000000, 10000000, 100000000) {
		Get-Location
		$text_file = "tmp\large_file.$num_chars.txt"
		$repeated_file = "tmp\repeated.$num_chars.txt"
		if (Test-Path $text_file -PathType leaf) {
			$text_file
			if (Test-Path $repeated_file -PathType leaf) {
				$repeated_file
				$repeated_text = Get-Content -Raw -Path $repeated_file
				$repeated_text

				(Measure-Command  {Invoke-LLL -TextFile $text_file -RepeatedText $repeated_text -NumChars $num_chars | Out-Default}).TotalMilliseconds
				(Measure-Command  {Invoke-RG -TextFile $text_file -RepeatedText $repeated_text -NumChars $num_chars | Out-Default}).TotalMilliseconds
			} else {
				{"$repeated_file does not exist. run .\generate_files.ps1"}
			}
		} else {
			{"$text_file does not exist. run .\generate_files.ps1"}
		}
	}
} else {
  {"../target/release/lll.exe does not exist. Please run 'cargo build --release'"}
}



#!/usr/bin/fish

# Search for all .kdl files in the current directory and its subdirectories
set files (find . -type f -name '*.kdl' -print)

# Use fzf to choose one of the files
set chosen_file (echo $files | fzf)

# Check if a file was chosen
if test -n "$chosen_file"
    # Assign the chosen file to a variable
	zellij action new-tab -l $chosen_file
else
    echo "No file chosen."
end

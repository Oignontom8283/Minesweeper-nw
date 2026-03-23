#!/bin/bash

_SELF_NAME="bootstrap.sh"

PATH_GIVED=""

# Analyze arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --name) # Argument --name
            PATH_GIVED="$2"
            shift 2
            ;;
        --help) # Display help
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --name <value>    Name of the project"
            echo "  --help            Display this help message"
            exit 0
            ;;
        *) # Unknown argument
            echo "Unknown argument: $1"
            echo "Use --help to see available options."
            exit 1
            ;;
    esac
done

echo ""
echo "Starting the project initialization script via eadkp template..."

# Determine if the script is being run locally or remotely
EXECUTION_SOURCE=$([[ -t 0 ]] && echo "local" || echo "remote")

echo "Execution source: $EXECUTION_SOURCE"

# Check git is installed
if ! command -v git >/dev/null 2>&1; then
    echo "Error: git is not installed. Please install git and try again."
    exit 1
else
    echo "git is installed."
fi


# If no name provided, prompt the user for it
# except if the script is run locally from its own directory
if [[ -z "$PATH_GIVED" ]]; then
    SCRIPT_DIR="$(dirname "$(realpath "$0")")"
    if [[ "$EXECUTION_SOURCE" == "local" && "$SCRIPT_DIR" == "$(pwd)" ]]; then
        PATH_GIVED="$SCRIPT_DIR"
    else
        read -p "Please enter the project name: " PATH_GIVED
    fi
fi

# Convert to absolute path
PATH_GIVED="$(realpath "$PATH_GIVED")"
PROJECT_NAME="$(basename "$PATH_GIVED")"
ORIGINAL_PROJECT_NAME="$PROJECT_NAME"

# Determine if we are auto-building (i.e., the provided path is the current directory)
IS_AUTO_BUILDING=$([[ "$PATH_GIVED" == "$(pwd)" ]] && echo "true" || echo "false")

# Build a Cargo-compatible name:
# - lowercase
# - spaces and hyphens replaced with underscores
FORMATTED_CARGO_NAME="$(echo "$ORIGINAL_PROJECT_NAME" | tr '[:upper:]' '[:lower:]' | tr ' -' '__')"

# Check that the project name contains only valid Cargo characters
if ! [[ "$ORIGINAL_PROJECT_NAME" =~ ^[a-z][a-z0-9_-]*$ ]]; then
    echo "Project name '$ORIGINAL_PROJECT_NAME' is not Cargo-compatible."
    echo "Proposed Cargo-compatible name: '$FORMATTED_CARGO_NAME'"

    read -p "Do you want to use this formatted name? (y/N): " USE_FORMATTED_NAME
    if [[ "$USE_FORMATTED_NAME" =~ ^[Yy]$ ]]; then
        if ! [[ "$FORMATTED_CARGO_NAME" =~ ^[a-z][a-z0-9_]*$ ]]; then
            echo "Error: The formatted name '$FORMATTED_CARGO_NAME' is still not Cargo-compatible."
            echo "Please choose a project name starting with a letter and containing only letters, numbers, spaces, hyphens, or underscores."
            exit 1
        fi
        echo "Using formatted Cargo name: '$FORMATTED_CARGO_NAME'"
    else
        echo "Initialization aborted by user."
        exit 1
    fi
fi

if ! [[ "$FORMATTED_CARGO_NAME" =~ ^[a-z][a-z0-9_]*$ ]]; then
    echo "Error: Cargo name '$FORMATTED_CARGO_NAME' is invalid."
    echo "Please use a project name that can be transformed into a valid Cargo name."
    exit 1
fi


echo ""
echo "Project name (directory): $ORIGINAL_PROJECT_NAME"
echo "Cargo name (text replacement): $FORMATTED_CARGO_NAME"
echo "Project will be initialized at: $PATH_GIVED"

# Create project directory recursively, if it does not exist
if ! mkdir -p "$PATH_GIVED"; then
    echo "Error: Failed to create directory '$PATH_GIVED'"
    exit 1
fi


# Change to the project directory
cd "$PATH_GIVED" || { echo "Failed to change directory to '$PATH_GIVED'"; exit 1; }

echo ""

# If auto-building, no clone is needed
if [[ "$IS_AUTO_BUILDING" == "true" ]]; then
    # Auto-building: use existing files

    echo "Auto-building detected: Using existing files in the current directory."
    echo "If the template files are incorrect, this may cause errors."
else
    # Clone the template repository using sparse-checkout

    echo "Cloning template repository into '$PATH_GIVED' ..."

    # Check if the directory is empty
    if ! ls -A "$PATH_GIVED" >/dev/null 2>&1; then
        echo "Error: Cannot access directory '$PATH_GIVED'"
        exit 1
    elif [ -n "$(ls -A "$PATH_GIVED" 2>/dev/null)" ]; then
        echo "Error: The directory '$PATH_GIVED' is not empty. Please choose an empty directory."
        exit 1
    fi

    # Clone the repository without checking out files
    if ! git clone --no-checkout --depth 1 --quiet https://github.com/Oignontom8283/eadkp_template.git .; then
        echo "Error: Failed to clone repository"
        exit 1
    fi

    # Enable sparse-checkout and checkout the repository
    if ! git sparse-checkout init >/dev/null 2>&1; then
        echo "Error: Failed to initialize sparse-checkout"
        exit 1
    fi

    # Configure sparse-checkout to include everything except bootstrap.sh
    cat > .git/info/sparse-checkout << "EOF"
/*
!${_SELF_NAME}
EOF
    if ! git checkout >/dev/null 2>&1; then
        echo "Error: Failed to checkout files"
        exit 1
    fi

    echo "Template files have been successfully cloned to '$PATH_GIVED' !"
fi


# Extract the list of excluded files from cargo-generate.toml dynamically
EXCLUDE_FILES=()
if [[ -f "cargo-generate.toml" ]]; then
    # Use a more robust approach to parse the exclude array
    EXCLUDE_FILES=($(grep -oP '(?<=exclude = \[)[^\]]*' cargo-generate.toml | tr -d '"' | tr ',' ' ' | tr -s ' '))
else
    echo "Warning: cargo-generate.toml not found, no files will be excluded from replacement"
fi

echo ""
echo "Files/dirs excluded from symbol replacement: ${EXCLUDE_FILES[*]}"
echo "..."

# Function to check if a file is in the exclude list
is_excluded() {
    local file="$1"
    # Remove ./ prefix if present for matching
    local clean_file="${file#./}"
    
    for exclude in "${EXCLUDE_FILES[@]}"; do
        if [[ "$clean_file" == $exclude* ]]; then
            return 0 # Exclu
        fi
    done
    return 1 # Non exclu
}

# Function to check if a file is a text file
is_text_file() {
    [[ -f "$1" ]] && file "$1" 2>/dev/null | grep -q "text"
}

FILES_REMPLACED=0
FILES_NOT_REPLACED=0
UNEXPECTED_FILES_NOT_REPLACED=0

# Replace {{project-name}} with cargo name in all files except excluded ones
# Use process substitution to avoid subshell variable scope issues
while IFS= read -r -d '' file; do
    
    # Skip excluded files
    if is_excluded "$file"; then
        FILES_NOT_REPLACED=$((FILES_NOT_REPLACED + 1))
        continue
    fi
    
    # Skip non-text files
    if ! is_text_file "$file"; then
        echo "WARN: Skipping ${file} because it is not a text file."
        UNEXPECTED_FILES_NOT_REPLACED=$((UNEXPECTED_FILES_NOT_REPLACED + 1))
        continue
    fi
    
    # Only process files that contain the pattern
    if ! grep -q "{{project-name}}" "$file"; then
        continue  # Skip files without the pattern (not counted)
    fi
    
    # Perform the replacement
    if sed -i "s/{{project-name}}/$FORMATTED_CARGO_NAME/g" "$file"; then
        FILES_REMPLACED=$((FILES_REMPLACED + 1))
    else
        echo "Error: Failed to update $file"
        UNEXPECTED_FILES_NOT_REPLACED=$((UNEXPECTED_FILES_NOT_REPLACED + 1))
    fi
    
done < <(find . -type f -not -path './.git/*' -print0)

echo "Symbol replacement completed ! :"
echo "${FILES_REMPLACED} : files have been successfully updated with the project name."
echo "${FILES_NOT_REPLACED} : files were excluded from replacement as per configuration."
echo "${UNEXPECTED_FILES_NOT_REPLACED} : files were skipped because they are not text files or due to errors."


echo ""
echo "Cleaning up..."

# Remove cargo generate file(s)
rm -f cargo-generate.toml > /dev/null 2>&1
rm -f cargo-generate.lock > /dev/null 2>&1
rm -f "${_SELF_NAME}" > /dev/null 2>&1 # Remove self

echo "Removed temporary files."

# Remove git and create a new repository
rm -rf .git
if ! git init --quiet >/dev/null 2>&1; then
    echo "Warning: Failed to initialize new git repository"
else
    echo "Created git repository."
fi

echo ""
echo "Project '$ORIGINAL_PROJECT_NAME' has been successfully initialized at '$PATH_GIVED' !"
echo "You can now start working on your project."
echo ""
#!/usr/bin/env bash
# Install git hooks for the project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "Installing git hooks..."

# Check if .git directory exists
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo "Error: .git directory not found. Are you in a git repository?"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p "$HOOKS_DIR"

# Copy pre-commit hook (create if doesn't exist)
if [ -f "$HOOKS_DIR/pre-commit" ]; then
    echo "⚠️  Pre-commit hook already exists. Backing up..."
    cp "$HOOKS_DIR/pre-commit" "$HOOKS_DIR/pre-commit.backup"
fi

# Create the pre-commit hook content
cat > "$HOOKS_DIR/pre-commit" << 'EOFHOOK'
#!/usr/bin/env bash
# Pre-commit hook for COSMIC Notification Applet
# This hook runs automatic code formatting and checks before each commit

set -e

echo "🔍 Running pre-commit checks..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track if any check failed
FAILED=0

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        FAILED=1
    fi
}

# 1. Check Rust formatting
echo "📝 Checking Rust code formatting..."
if cargo fmt --all -- --check > /dev/null 2>&1; then
    print_status 0 "Rust formatting"
else
    echo -e "${YELLOW}⚠${NC}  Code is not formatted. Running rustfmt..."
    cargo fmt --all
    print_status 0 "Rust formatting (auto-fixed)"
fi

# 2. Run Clippy (lint checker)
echo "🔎 Running Clippy linter..."
if cargo clippy --all-targets -- -D warnings 2>&1 | grep -q "warning:"; then
    print_status 1 "Clippy checks"
    echo -e "${RED}Please fix clippy warnings before committing${NC}"
else
    print_status 0 "Clippy checks"
fi

# 3. Check for common issues
echo "🔍 Checking for common issues..."

# Check for debug prints
if git diff --cached --name-only | grep '\.rs$' | xargs grep -n "dbg!" 2>/dev/null; then
    print_status 1 "No dbg!() macros found"
    echo -e "${YELLOW}⚠${NC}  Found dbg!() macro - consider removing for production"
else
    print_status 0 "No dbg!() macros"
fi

# Check for TODO/FIXME without issue reference
if git diff --cached --name-only | grep '\.rs$' | xargs grep -n "TODO\|FIXME" 2>/dev/null | grep -v "#[0-9]"; then
    echo -e "${YELLOW}⚠${NC}  Found TODO/FIXME without issue reference - consider linking to GitHub issue"
fi

# 4. Run tests (optional - can be slow)
if [ -z "$SKIP_TESTS" ]; then
    echo "🧪 Running tests..."
    if cargo test --quiet 2>&1 | tail -1 | grep -q "test result: ok"; then
        print_status 0 "Tests passed"
    else
        print_status 1 "Tests"
        echo -e "${RED}Please fix failing tests before committing${NC}"
    fi
else
    echo -e "${YELLOW}⚠${NC}  Skipping tests (SKIP_TESTS is set)"
fi

# 5. Check Nix files (if changed)
if git diff --cached --name-only | grep '\.nix$' > /dev/null; then
    echo "❄️  Checking Nix files..."

    # Check if nixpkgs-fmt is available
    if command -v nixpkgs-fmt > /dev/null 2>&1; then
        if git diff --cached --name-only | grep '\.nix$' | xargs nixpkgs-fmt --check > /dev/null 2>&1; then
            print_status 0 "Nix formatting"
        else
            echo -e "${YELLOW}⚠${NC}  Nix files not formatted. Running nixpkgs-fmt..."
            git diff --cached --name-only | grep '\.nix$' | xargs nixpkgs-fmt
            git diff --cached --name-only | grep '\.nix$' | xargs git add
            print_status 0 "Nix formatting (auto-fixed)"
        fi
    else
        echo -e "${YELLOW}⚠${NC}  nixpkgs-fmt not found, skipping Nix formatting check"
    fi

    # Check with statix if available
    if command -v statix > /dev/null 2>&1; then
        if git diff --cached --name-only | grep '\.nix$' | xargs statix check > /dev/null 2>&1; then
            print_status 0 "Nix linting (statix)"
        else
            print_status 1 "Nix linting (statix)"
            echo -e "${YELLOW}⚠${NC}  Run 'statix fix' to auto-fix some issues"
        fi
    fi
fi

echo ""

# Final result
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All pre-commit checks passed!${NC}"
    echo ""
    exit 0
else
    echo -e "${RED}❌ Some checks failed. Please fix the issues above.${NC}"
    echo ""
    echo "Tip: You can skip these checks with 'git commit --no-verify' (not recommended)"
    echo "Tip: Set SKIP_TESTS=1 to skip running tests: SKIP_TESTS=1 git commit"
    echo ""
    exit 1
fi
EOFHOOK

# Make hook executable
chmod +x "$HOOKS_DIR/pre-commit"

echo "✅ Pre-commit hook installed successfully!"
echo ""
echo "The hook will:"
echo "  - Check and auto-fix Rust formatting"
echo "  - Run Clippy linter"
echo "  - Check for common issues (dbg!, TODO/FIXME)"
echo "  - Run tests (set SKIP_TESTS=1 to skip)"
echo "  - Check Nix file formatting (if applicable)"
echo ""
echo "To bypass the hook (not recommended): git commit --no-verify"
echo "To skip tests: SKIP_TESTS=1 git commit"

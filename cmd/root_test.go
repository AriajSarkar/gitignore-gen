package cmd

import (
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"testing"
)

func TestAnalyzeProject(t *testing.T) {
	// Create temporary test directory
	tmpDir, err := os.MkdirTemp("", "gitignore-test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	// Create test files
	files := map[string]string{
		"main.go":         "",
		"package.json":    "{}",
		"app.py":         "",
		"pom.xml":        "",
		"Cargo.toml":     "",
	}

	for name, content := range files {
		err := os.WriteFile(filepath.Join(tmpDir, name), []byte(content), 0644)
		if err != nil {
			t.Fatal(err)
		}
	}

	// Test detection
	detected := analyzeProject(tmpDir)
	expected := []string{"Go", "Node", "Python", "Maven", "Rust"}

	// Sort both slices to ensure consistent comparison
	sort.Strings(detected)
	sort.Strings(expected)
	
	if !reflect.DeepEqual(detected, expected) {
		t.Errorf("Expected %v, got %v", expected, detected)
	}
}

func TestFetchGitignoreTemplate(t *testing.T) {
	technologies := []string{"Go", "Node"}
	content, err := fetchGitignoreTemplate(technologies)
	if err != nil {
		t.Errorf("Failed to fetch template: %v", err)
	}
	if content == "" {
		t.Error("Expected non-empty template content")
	}
}

package cmd

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "gitignore-gen",
	Short: "Generate .gitignore files based on project analysis",
	Long: `A CLI tool that analyzes your project structure and generates
appropriate .gitignore files by detecting technologies and frameworks.
Uses the gitignore.io API to fetch the latest templates.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		path, err := os.Getwd()
		if err != nil {
			return fmt.Errorf("failed to get current directory: %w", err)
		}

		detected := analyzeProject(path)
		if len(detected) == 0 {
			return fmt.Errorf("no supported technologies detected in the project")
		}

		content, err := fetchGitignoreTemplate(detected)
		if err != nil {
			return fmt.Errorf("failed to fetch gitignore template: %w", err)
		}

		err = os.WriteFile(".gitignore", []byte(content), 0644)
		if err != nil {
			return fmt.Errorf("failed to write .gitignore file: %w", err)
		}

		fmt.Printf("Generated .gitignore file for: %s\n", strings.Join(detected, ", "))
		return nil
	},
}

var uninstallCmd = &cobra.Command{
	Use:   "uninstall",
	Short: "Uninstall gitignore-gen",
	RunE: func(cmd *cobra.Command, args []string) error {
		executable, err := os.Executable()
		if err != nil {
			return fmt.Errorf("failed to get executable path: %w", err)
		}
		
		fmt.Printf("To uninstall, manually delete the binary at: %s\n", executable)
		fmt.Println("For more information, visit: https://github.com/AriajSarkar/gitignore-gen#uninstallation")
		return nil
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolP("force", "f", false, "Force overwrite existing .gitignore file")
	rootCmd.AddCommand(uninstallCmd)
}

func analyzeProject(path string) []string {
	var detected []string
	filepath.Walk(path, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			switch info.Name() {
			case "node_modules":
				detected = append(detected, "Node")
			case "venv", ".venv":
				detected = append(detected, "Python")
			case "vendor":
				detected = append(detected, "Go")
			}
			return nil
		}

		switch {
		case strings.HasSuffix(info.Name(), ".js"):
			detected = append(detected, "Node")
		case strings.HasSuffix(info.Name(), ".py"):
			detected = append(detected, "Python")
		case strings.HasSuffix(info.Name(), ".go"):
			detected = append(detected, "Go")
		case strings.HasSuffix(info.Name(), ".java"):
			detected = append(detected, "Java")
		case strings.HasSuffix(info.Name(), ".cs"):
			detected = append(detected, "VisualStudio")
		case info.Name() == "package.json":
			detected = append(detected, "Node")
		case info.Name() == "requirements.txt":
			detected = append(detected, "Python")
		case info.Name() == "go.mod":
			detected = append(detected, "Go")
		case info.Name() == "pom.xml":
			detected = append(detected, "Maven")
		case info.Name() == "Cargo.toml":
			detected = append(detected, "Rust")
		}
		return nil
	})
	return unique(detected)
}

func fetchGitignoreTemplate(technologies []string) (string, error) {
	url := fmt.Sprintf("https://www.toptal.com/developers/gitignore/api/%s", strings.Join(technologies, ","))
	resp, err := http.Get(url)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	content, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	return string(content), nil
}

func unique(elements []string) []string {
	seen := make(map[string]bool)
	var result []string
	for _, v := range elements {
		if !seen[v] {
			seen[v] = true
			result = append(result, v)
		}
	}
	return result
}
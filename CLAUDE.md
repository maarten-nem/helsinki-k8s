# DevOps with Kubernetes - Exercise Repository

Course: https://devopswithkubernetes.com/

## Repository Structure

```
helsinki-k8s/
├── README.md           # exercise links
├── CLAUDE.md           # this file
├── .github/workflows/  # CI workflows
│   └── build-X_Y.yml   # per-exercise build workflow
├── 1_1/                # exercise 1.1
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── README.md       # build/run/deploy instructions
│   ├── manifests/      # k8s manifests (added later)
│   └── src/
├── 1_2/                # exercise 1.2
└── ...
```

## Adding a New Exercise

1. Create directory `X_Y/` for exercise X.Y
2. Add application code, Dockerfile, and README.md
3. Add GitHub Actions workflow `.github/workflows/build-X_Y.yml`
4. Update `README.md` to add a link: `- [X.Y](X_Y/)`
5. Commit with message: `feat: complete exercise X.Y`
6. When the exercise is complete, create a GitHub release with tag `X.Y`

## CI Workflows

Each exercise has a workflow that builds and pushes to ghcr.io on push to main.

Images are published to: `ghcr.io/<owner>/helsinki-k8s/<app-name>`

Workflows trigger only when files in the exercise directory change.

## Creating a Release

```bash
git tag X.Y
git push origin main --tags
gh release create X.Y --title "X.Y" --notes "description"
```

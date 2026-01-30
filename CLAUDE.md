# DevOps with Kubernetes - Exercise Repository

Course: https://devopswithkubernetes.com/

## Repository Structure

```
helsinki-k8s/
├── README.md           # exercise links
├── CLAUDE.md           # this file
├── 1_1/                # exercise 1.1
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── manifests/      # k8s manifests (added later)
│   └── src/
├── 1_2/                # exercise 1.2
└── ...
```

## Adding a New Exercise

1. Create directory `X_Y/` for exercise X.Y
2. Add application code and Dockerfile
3. Update `README.md` to add a link: `- [X.Y.](X_Y/)`
4. Commit with message: `feat: complete exercise X.Y`
5. When the exercise is complete, create a GitHub release with tag `X.Y`

## Creating a Release

```bash
git tag X.Y
git push origin main --tags
gh release create X.Y --title "X.Y" --notes "description"
```

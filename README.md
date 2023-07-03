# package-analyzer

```shell
cd <monorepo_path> && git ls-files | grep "package.json$" > paths.txt
cat paths.txt | xargs cargo run -- --base <monorepo_path> --paths
```

import path from "node:path";

export type Classification = {
    fixturePackageJsons: string[];
    unmanagedPackageJsons: string[];
    workspacePackageJsons: string[];
};

const fixturePackageJsons = [
    "crates/swc/tests/fixture/jsc-paths/vercel-site/1/input/package.json",
    "crates/swc_ecma_loader/tests/hoisting/packages/app/package.json",
    "crates/swc_ecma_parser/tests/test262-parser/package.json",
    "crates/swc_ecma_transforms_proposal/tests/decorator-tests/package.json",
    "crates/swc_estree_compat/tests/package.json",
    "crates/swc_node_bundler/tests/package.json",
    "crates/swc_node_bundler/tests/integration/react/package.json",
    "crates/swc_node_bundler/tests/pass/resolve-name-fix/input/package.json",
].sort();

export async function classify(rootDir: string): Promise<Classification> {
    const allPackageJsons = Array.from(
        new Bun.Glob("**/package.json").scanSync({
            cwd: rootDir,
            onlyFiles: true,
        })
    )
        .filter((file) => !file.includes("/node_modules/"))
        .sort();

    const rootPackageJson = await Bun.file(path.join(rootDir, "package.json")).json();
    const workspacePatterns: string[] = rootPackageJson.workspaces;

    const workspacePackageJsons = Array.from(
        new Set([
            "package.json",
            ...workspacePatterns.flatMap((pattern) => {
                const normalizedPattern = pattern.replace(/^\.\//, "");
                const packageJsonPattern = path.posix.join(
                    normalizedPattern,
                    "package.json"
                );

                return Array.from(
                    new Bun.Glob(packageJsonPattern).scanSync({
                        cwd: rootDir,
                        onlyFiles: true,
                    })
                );
            }),
        ])
    )
        .filter((file) => !file.includes("/node_modules/"))
        .sort();

    const fixtureSet = new Set(fixturePackageJsons);
    const workspaceSet = new Set(workspacePackageJsons);

    const unmanagedPackageJsons = allPackageJsons.filter((file) => {
        return !fixtureSet.has(file) && !workspaceSet.has(file);
    });

    return {
        fixturePackageJsons,
        unmanagedPackageJsons,
        workspacePackageJsons,
    };
}

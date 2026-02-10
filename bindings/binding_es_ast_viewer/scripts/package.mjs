import { cp } from "node:fs/promises";

const pkgJson = await Bun.file("pkg/package.json").json();
pkgJson.name = "@swc/es-ast-viewer";
pkgJson.type = "module";
pkgJson.files.push("es_ast_viewer_node.js");
pkgJson.exports = {
    types: "./es_ast_viewer.d.ts",
    node: "./es_ast_viewer_node.js",
    default: "./es_ast_viewer.js",
};

await Promise.all([
    cp("src/es_ast_viewer_node.js", "pkg/es_ast_viewer_node.js"),
    Bun.write("pkg/package.json", JSON.stringify(pkgJson, null, 2)),
]);

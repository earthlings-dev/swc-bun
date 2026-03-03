import path from "path";
import swc from "../..";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __packageRoot = path.join(__filename, "..", "..", "..");

describe("jsc.paths", () => {
    let savedCwd;

    beforeEach(() => {
        savedCwd = process.cwd();
        process.chdir(__packageRoot);
    });

    afterEach(() => {
        process.chdir(savedCwd);
    });

    it("should work with process.cwd()", async () => {
        const testDir = path.join(
            __filename,
            "..",
            "..",
            "..",
            "tests",
            "swc-path-bug-1"
        );
        const f = path.join(testDir, "src", "index.ts");
        console.log(f);
        expect(
            (
                await swc.transformFile(f, {
                    jsc: {
                        parser: {
                            syntax: "typescript",
                        },
                        baseUrl: testDir,
                        paths: {
                            "@utils/*": ["src/utils/*"],
                        },
                    },
                })
            ).code
        ).toMatchInlineSnapshot(`
            ""use strict";
            Object.defineProperty(exports, "__esModule", {
                value: true
            });
            const _helloworldutils = require("./utils/hello-world.utils.js");
            console.log((0, _helloworldutils.helloWorld)());
            "
        `);
    });

    it("should work with process.cwd() and relative url", async () => {
        // Resolve paths eagerly while cwd is correct (before any await),
        // because Bun runs test files concurrently in the same process and
        // other files may call process.chdir() during our await.
        const testDir = path.join("tests", "swc-path-bug-1");
        const absFile = path.resolve(testDir, "src", "index.ts");
        const absBaseUrl = path.resolve(testDir);
        console.log(testDir);
        expect(
            (
                await swc.transformFile(absFile, {
                    jsc: {
                        parser: {
                            syntax: "typescript",
                        },
                        baseUrl: absBaseUrl,
                        paths: {
                            "@utils/*": ["src/utils/*"],
                        },
                    },
                })
            ).code
        ).toMatchInlineSnapshot(`
            ""use strict";
            Object.defineProperty(exports, "__esModule", {
                value: true
            });
            const _helloworldutils = require("./utils/hello-world.utils.js");
            console.log((0, _helloworldutils.helloWorld)());
            "
        `);
    });
});

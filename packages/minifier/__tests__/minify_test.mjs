import { describe, it, expect } from "bun:test";
import { minifySync, minify } from "../index.js";

describe("minifySync", () => {
    it("minifies a simple variable declaration", () => {
        const input = "const longVariableName = 1;";
        const result = minifySync(input, {
            compress: true,
            mangle: true,
        });

        expect(result).toBeDefined();
        expect(typeof result.code).toBe("string");
        expect(result.code.length).toBeLessThan(input.length);
    });

    it("folds constants with compress enabled", () => {
        const result = minifySync("var a = 1 + 2;", {
            compress: true,
            mangle: false,
        });

        expect(result).toBeDefined();
        expect(result.code).toContain("3");
    });

    it("returns an object with a code property", () => {
        const result = minifySync("const x = 1;", {
            compress: true,
            mangle: false,
        });

        expect(result).toHaveProperty("code");
    });
});

describe("minify (async)", () => {
    it("minifies asynchronously", async () => {
        const input = "const longVariableName = 1;";
        const result = await minify(input, {
            compress: true,
            mangle: true,
        });

        expect(result).toBeDefined();
        expect(typeof result.code).toBe("string");
        expect(result.code.length).toBeLessThan(input.length);
    });
});

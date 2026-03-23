import { describe, it, expect } from "bun:test";
import { parseSync } from "../../src/index.ts";

describe("parseSync span normalization", () => {
    it("returns 0-based spans for a single parse", () => {
        const source = "const x = 1;";
        const ast = parseSync(source, { syntax: "ecmascript" });

        expect(ast.span.start).toBe(0);
        expect(ast.span.end).toBe(Buffer.byteLength(source, "utf-8"));
    });

    it("returns 0-based spans after multiple prior parses", () => {
        // Parse 5 files of varying size to accumulate BytePos
        for (let i = 0; i < 5; i++) {
            parseSync(`const x${i} = ${"a".repeat(10000 * (i + 1))};`, {
                syntax: "ecmascript",
            });
        }

        const source = `import merge from 'lodash.merge';\nconst x = merge(a, b);\n`;
        const ast = parseSync(source, { syntax: "typescript" });

        expect(ast.span.start).toBe(0);
        expect(ast.span.end).toBe(Buffer.byteLength(source, "utf-8"));
    });

    it("handles unicode source correctly (byte offsets, not char offsets)", () => {
        const source = 'const emoji = "\u{1F600}";';
        const ast = parseSync(source, { syntax: "ecmascript" });

        expect(ast.span.start).toBe(0);
        // 4-byte emoji means byteLength > string length
        expect(ast.span.end).toBe(Buffer.byteLength(source, "utf-8"));
    });

    it("inner node spans are also 0-based", () => {
        // Parse a throwaway file first to shift the global counter
        parseSync("const throwaway = true;", { syntax: "ecmascript" });

        const source = "const x = 1;";
        const ast = parseSync(source, { syntax: "ecmascript" });

        // The variable declaration should start at 0
        const decl = ast.body[0];
        expect(decl.span.start).toBe(0);
        expect(decl.span.end).toBeLessThanOrEqual(
            Buffer.byteLength(source, "utf-8"),
        );
    });
});

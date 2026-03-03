import { describe, it, expect } from "bun:test";
import {
    isReactCompilerRequiredSync,
    isReactCompilerRequired,
} from "../index.js";

describe("isReactCompilerRequiredSync", () => {
    it("returns false for plain JavaScript without React", () => {
        const code = Buffer.from("const x = 1 + 2;");
        const result = isReactCompilerRequiredSync(code);

        expect(result).toBe(false);
    });

    it("returns a boolean for code with hooks", () => {
        const code = Buffer.from(`
            import { useState } from 'react';
            function Counter() {
                const [count, setCount] = useState(0);
                return count;
            }
        `);
        const result = isReactCompilerRequiredSync(code);

        expect(typeof result).toBe("boolean");
    });

    it("returns a boolean for a simple component", () => {
        const code = Buffer.from("function App() { return null; }");
        const result = isReactCompilerRequiredSync(code);

        expect(typeof result).toBe("boolean");
    });
});

describe("isReactCompilerRequired (async)", () => {
    it("resolves to a boolean", async () => {
        const code = Buffer.from("function App() { return null; }");
        const result = await isReactCompilerRequired(code);

        expect(typeof result).toBe("boolean");
    });
});

/**
 * ts-to-zod configuration.
 *
 * @type {import("ts-to-zod").TsToZodConfig}
 */
// eslint-disable-next-line no-undef
module.exports = {
    input: "src/bindings.ts",
    output: "src/bindings.zod.ts",
    nameFilter: (name) => name !== "Result" && name !== "__EventObj__",
};


/** @type {import('prettier').Config & import('prettier-plugin-tailwindcss').PluginOptions} */
export default {
    plugins: ["prettier-plugin-tailwindcss"],
    tailwindStylesheet: "./src/index.css",
    trailingComma: "es5",
    tabWidth: 4,
    semi: true,
    singleQuote: false,
};

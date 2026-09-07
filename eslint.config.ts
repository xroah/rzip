import js from "@eslint/js"
import globals from "globals"
import tseslint from "typescript-eslint"
import pluginReact from "eslint-plugin-react"
import { defineConfig } from "eslint/config"
import stylistic from "@stylistic/eslint-plugin"

export default defineConfig([
    {
        ignores: ["src-tauri/**"]
    },
    {
        files: ["**/*.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
        plugins: { js },
        extends: ["js/recommended"],
        languageOptions: { globals: globals.browser },
        settings: {
            react: {
                version: "19"
            }
        }
    },
    tseslint.configs.recommended,
    pluginReact.configs.flat.recommended,
    stylistic.configs.recommended,
    {
        rules: {
            "no-unused-vars": [1],
            "@typescript/no-unused-vars": [1],
            "@stylistic/indent": [1, 4],
            "@stylistic/quotes": [1, "double"],
            "@stylistic/comma-dangle": "off",
            "@stylistic/semi": [1, "never"],
            "@stylistic/arrow-parens": [1, "as-needed"],
            "react/react-in-jsx-scope": "off"
        },
    }
])

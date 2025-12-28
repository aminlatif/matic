// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt(
    {
        rules: {
            "@stylistic/semi": ["warn", "always"],
            "@stylistic/indent": ["warn", 4],
            "@stylistic/quotes": ["warn", "double"],
            "vue/html-indent": ["error", 4, {
                attribute: 1,
                baseIndent: 1,
                closeBracket: 0,
                alignAttributesVertically: false,
                ignores: []
            }],
            "vue/first-attribute-linebreak": ["warn", {
                singleline: "ignore",
                multiline: "ignore"
            }],
            "vue/max-attributes-per-line": ["warn", {
                singleline: {
                    max: 5
                },
                multiline: {
                    max: 5
                }
            }],
            "vue/html-closing-bracket-newline": [
                "error",
                {
                    singleline: "never",
                    multiline: "never",
                    selfClosingTag: {
                        singleline: "never",
                        multiline: "never"
                    }
                }
            ]
        }
    }
);

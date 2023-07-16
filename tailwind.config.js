module.exports = {
  content: ["./src/**/*.rs", "./src/**/*.css", "./index.html"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: [
      "light",
      {
        dark: {
          ...require("daisyui/src/theming/themes")["[data-theme=night]"],
        },
      },
    ],
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui"),
    function ({ addVariant }) {
      addVariant("child", "& > *");
    },
  ],
};

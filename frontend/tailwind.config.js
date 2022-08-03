/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./*.{html,js}"],
  theme: {
    colors: {
      seafoam: "#86A6A6",
      jade: "#537370",
      forest: "#28403B",
      field: "#36594C",
      gravel: "#D9D9D9",
    },
    extend: {},
  },
  plugins: [require("daisyui")],
};

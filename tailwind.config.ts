import type { Config } from 'tailwindcss';
import tailwindScrollbar from 'tailwind-scrollbar';

export default {
  content: ['src/**/*.{html,js,ts,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        intro: 'Roboto, sans-serif',
        sans: 'sans-serif',
      }
    }
  },
  plugins: [
    tailwindScrollbar({
      nocompatible: true,
      preferredStrategy: 'pseudoelements'
    }),
  ],
} satisfies Config


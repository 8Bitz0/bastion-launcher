import type { Config } from 'tailwindcss';
import remToPx from 'tailwindcss-rem-to-px';
import tailwindScrollbar from 'tailwind-scrollbar';

export default {
  content: ['src/**/*.{html,js,ts,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        intro: 'Roboto, sans-serif',
        sans: 'Roboto, sans-serif',
      },
    }
  },
  plugins: [
    remToPx({}),
    tailwindScrollbar({
      nocompatible: true,
      preferredStrategy: 'pseudoelements'
    }),
  ],
} satisfies Config

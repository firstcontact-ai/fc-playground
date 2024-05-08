import rollup_cjs from '@rollup/plugin-commonjs';
import rollup_nre from '@rollup/plugin-node-resolve';
import terser from '@rollup/plugin-terser';
import rollup_ts from 'rollup-plugin-typescript2';

// NOTE: Somehow, when one of the source is .ts only, the official @rollup/plugin-typescript
//       cannot resolve some of the modules.

export default [
  {
    input: './src-ui/src/index.ts',
    output: {
      file: './dist-ui/js/app-bundle.js',
      format: 'iife',
      name: 'bundle',
      sourcemap: true
    },
    plugins: [
      rollup_cjs(),
      rollup_nre(),
      rollup_ts({
        tsconfig: './src-ui/tsconfig.json'
      }),
      // terser({ compress: false, mangle: false, format: { comments: false } })
    ]
  }
]


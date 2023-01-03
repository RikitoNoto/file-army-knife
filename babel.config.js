module.exports = {
  presets: [
    ['@babel/preset-env', {targets: {node: 'current'}}],
    '@babel/preset-typescript',
    '@expo/next-adapter/babel',
  ],
  plugins: [
    // 'react-native-reanimated/plugin',
    ['@babel/plugin-proposal-private-methods', { loose: true }],
    ['@babel/plugin-proposal-class-properties', { loose: true }],
    ['@babel/plugin-proposal-private-property-in-object', { loose: true }]
  ],
  // overrides: [
  //   {
  //     test: './node_modules/react-native-reanimated/*',
  //     plugins: ['@babel/plugin-proposal-class-properties']
  //   }
  // ]
};

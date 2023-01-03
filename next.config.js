/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    unoptimized: true,
    disableStaticImages: true,
  },
  build: {
    babel: {
      plugins: [['@babel/plugin-proposal-class-properties', { loose: true }]],
      plugins: [['@babel/plugin-proposal-private-methods', { loose: true }]],
      plugins: [['@babel/plugin-proposal-private-property-in-object', { loose: true }]],
    },
  },
}

module.exports = nextConfig

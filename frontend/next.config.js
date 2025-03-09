/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:8000/api/:path*', // Proxy to backend
      },
      {
        source: '/ws',
        destination: 'http://localhost:8000/ws', // Proxy to WebSocket (browsers will upgrade the connection)
      },
    ]
  },
}

module.exports = nextConfig 
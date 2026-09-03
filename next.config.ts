import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "export",
  images: { unoptimized: true },
  serverExternalPackages: ["@tailwindcss/oxide", "@tailwindcss/oxide-linux-x64-gnu"],
};

export default nextConfig;

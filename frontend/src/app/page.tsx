'use client';

import React, { useState } from 'react';
import Link from 'next/link';

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col">
      {/* Header */}
      <header className="bg-gray-900 text-white shadow-md">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div className="flex items-center">
            <h1 className="text-2xl font-bold">ARB Platform</h1>
            <span className="ml-2 text-xs bg-primary-600 text-white px-2 py-1 rounded">BETA</span>
          </div>
          <nav className="hidden md:flex space-x-6">
            <Link href="/dashboard" className="hover:text-primary-400 transition-colors">
              Dashboard
            </Link>
            <Link href="/trading" className="hover:text-primary-400 transition-colors">
              Trading
            </Link>
            <Link href="/strategies" className="hover:text-primary-400 transition-colors">
              Strategies
            </Link>
            <Link href="/backtesting" className="hover:text-primary-400 transition-colors">
              Backtesting
            </Link>
            <Link href="/account" className="hover:text-primary-400 transition-colors">
              Account
            </Link>
            <Link href="/api-docs" className="hover:text-primary-400 transition-colors">
              API Docs
            </Link>
          </nav>
          <div className="flex items-center space-x-4">
            <button className="btn btn-primary">Login</button>
            <button className="btn btn-outline">Sign Up</button>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="bg-gradient-to-b from-gray-900 to-gray-800 text-white py-20">
        <div className="container mx-auto px-4">
          <div className="flex flex-col md:flex-row items-center">
            <div className="md:w-1/2 mb-10 md:mb-0">
              <h1 className="text-4xl md:text-5xl font-bold mb-6">
                High-Frequency Trading Platform
              </h1>
              <p className="text-xl mb-8">
                Advanced trading platform for stocks, bonds, crypto, and other assets with multiple arbitrage strategies.
              </p>
              <div className="flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4">
                <button className="btn btn-primary text-lg px-8 py-3">Get Started</button>
                <button className="btn btn-outline text-lg px-8 py-3">Learn More</button>
              </div>
            </div>
            <div className="md:w-1/2">
              <div className="bg-gray-800 p-6 rounded-lg shadow-xl">
                <div className="bg-gray-900 p-4 rounded-lg mb-4">
                  <div className="flex justify-between items-center mb-4">
                    <h3 className="text-lg font-semibold">BTC/USD</h3>
                    <span className="text-success-500">+2.34%</span>
                  </div>
                  <div className="h-40 bg-gray-800 rounded mb-4">
                    {/* Chart would go here */}
                    <div className="h-full flex items-center justify-center text-gray-500">
                      Price Chart
                    </div>
                  </div>
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <p className="text-gray-400 text-sm">Price</p>
                      <p className="text-xl font-semibold">$35,245.67</p>
                    </div>
                    <div>
                      <p className="text-gray-400 text-sm">24h Volume</p>
                      <p className="text-xl font-semibold">$1.2B</p>
                    </div>
                  </div>
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-gray-900 p-4 rounded-lg">
                    <h3 className="text-lg font-semibold mb-2">ETH/USD</h3>
                    <p className="text-xl font-semibold">$2,245.12</p>
                    <p className="text-danger-500">-0.87%</p>
                  </div>
                  <div className="bg-gray-900 p-4 rounded-lg">
                    <h3 className="text-lg font-semibold mb-2">SOL/USD</h3>
                    <p className="text-xl font-semibold">$82.45</p>
                    <p className="text-success-500">+3.21%</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 bg-white dark:bg-gray-900">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12 dark:text-white">Key Features</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div className="card">
              <div className="h-12 w-12 bg-primary-100 dark:bg-primary-900 text-primary-600 dark:text-primary-300 rounded-lg flex items-center justify-center mb-4">
                <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-2 dark:text-white">Multiple Arbitrage Strategies</h3>
              <p className="text-gray-600 dark:text-gray-300">
                Leverage event arbitrage, statistical arbitrage, information arbitrage, and latency arbitrage strategies.
              </p>
            </div>
            <div className="card">
              <div className="h-12 w-12 bg-primary-100 dark:bg-primary-900 text-primary-600 dark:text-primary-300 rounded-lg flex items-center justify-center mb-4">
                <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-2 dark:text-white">Multi-Asset Trading</h3>
              <p className="text-gray-600 dark:text-gray-300">
                Trade stocks, bonds, cryptocurrencies, forex, and other assets all from a single platform.
              </p>
            </div>
            <div className="card">
              <div className="h-12 w-12 bg-primary-100 dark:bg-primary-900 text-primary-600 dark:text-primary-300 rounded-lg flex items-center justify-center mb-4">
                <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-2 dark:text-white">24/7 Autonomous Operation</h3>
              <p className="text-gray-600 dark:text-gray-300">
                Set up your strategies and let the platform trade automatically, even while you sleep.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-white py-8 mt-auto">
        <div className="container mx-auto px-4">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <div className="mb-4 md:mb-0">
              <h2 className="text-xl font-bold">ARB Platform</h2>
              <p className="text-gray-400">High-Frequency Trading Platform</p>
            </div>
            <div className="flex space-x-6">
              <a href="#" className="hover:text-primary-400 transition-colors">Terms</a>
              <a href="#" className="hover:text-primary-400 transition-colors">Privacy</a>
              <a href="#" className="hover:text-primary-400 transition-colors">Contact</a>
              <Link href="/api-docs" className="hover:text-primary-400 transition-colors">API Documentation</Link>
            </div>
          </div>
          <div className="border-t border-gray-800 mt-6 pt-6 text-center md:text-left">
            <p className="text-gray-400">&copy; {new Date().getFullYear()} ARB Platform. All rights reserved.</p>
          </div>
        </div>
      </footer>
    </div>
  );
} 
'use client';

import React from 'react';

// Parameter type
interface EndpointParameter {
  name: string;
  type: string;
  required: boolean;
  description: string;
}

// Props interface
interface EndpointCardProps {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  path: string;
  description: string;
  parameters?: EndpointParameter[];
  requestExample?: string;
  responseExample: string;
  isActive: boolean;
}

const EndpointCard: React.FC<EndpointCardProps> = ({
  method,
  path,
  description,
  parameters,
  requestExample,
  responseExample,
  isActive,
}) => {
  // Method badge color
  const getMethodBadgeClasses = () => {
    switch (method) {
      case 'GET':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300';
      case 'POST':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300';
      case 'PUT':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300';
      case 'DELETE':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300';
      default:
        return '';
    }
  };

  return (
    <div 
      id={`${method}-${path}`}
      className={`bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6 ${
        isActive ? 'ring-2 ring-primary-500' : ''
      }`}
    >
      <div className="flex flex-wrap items-center gap-2 mb-4">
        <span className={`inline-block text-sm font-mono px-2 py-1 rounded ${getMethodBadgeClasses()}`}>
          {method}
        </span>
        <h3 className="text-lg font-mono dark:text-white">{path}</h3>
      </div>
      
      <p className="text-gray-600 dark:text-gray-300 mb-4">{description}</p>
      
      {parameters && parameters.length > 0 && (
        <div className="mb-4">
          <h4 className="font-semibold text-gray-700 dark:text-gray-200 mb-2">Parameters</h4>
          <div className="bg-gray-50 dark:bg-gray-900 rounded-lg overflow-hidden">
            <table className="min-w-full">
              <thead className="bg-gray-100 dark:bg-gray-800">
                <tr>
                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Type</th>
                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Required</th>
                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                {parameters.map((param) => (
                  <tr key={param.name}>
                    <td className="px-4 py-2 text-sm font-mono text-gray-800 dark:text-gray-200">{param.name}</td>
                    <td className="px-4 py-2 text-sm text-gray-600 dark:text-gray-300">{param.type}</td>
                    <td className="px-4 py-2 text-sm text-gray-600 dark:text-gray-300">
                      {param.required ? (
                        <span className="text-danger-600 dark:text-danger-400">Yes</span>
                      ) : (
                        <span className="text-gray-400">No</span>
                      )}
                    </td>
                    <td className="px-4 py-2 text-sm text-gray-600 dark:text-gray-300">{param.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
      
      {requestExample && (
        <div className="mb-4">
          <h4 className="font-semibold text-gray-700 dark:text-gray-200 mb-2">Request Example</h4>
          <div className="bg-gray-800 rounded-lg p-4 overflow-auto">
            <pre className="text-gray-100 font-mono text-sm leading-relaxed whitespace-pre-wrap">{requestExample}</pre>
          </div>
        </div>
      )}
      
      <div>
        <h4 className="font-semibold text-gray-700 dark:text-gray-200 mb-2">Response Example</h4>
        <div className="bg-gray-800 rounded-lg p-4 overflow-auto">
          <pre className="text-gray-100 font-mono text-sm leading-relaxed whitespace-pre-wrap">{responseExample}</pre>
        </div>
      </div>
    </div>
  );
};

export default EndpointCard; 
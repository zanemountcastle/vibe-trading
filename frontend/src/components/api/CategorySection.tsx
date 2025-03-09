'use client';

import React from 'react';
import EndpointCard from './EndpointCard';

// Interfaces
interface Endpoint {
  path: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  description: string;
  parameters?: {
    name: string;
    type: string;
    required: boolean;
    description: string;
  }[];
  responseExample: string;
  requestExample?: string;
}

interface CategorySectionProps {
  name: string;
  description: string;
  endpoints: Endpoint[];
  activeEndpoint: string | null;
}

const CategorySection: React.FC<CategorySectionProps> = ({
  name,
  description,
  endpoints,
  activeEndpoint,
}) => {
  return (
    <div>
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6">
        <h2 className="text-2xl font-bold mb-2 dark:text-white">
          {name.charAt(0).toUpperCase() + name.slice(1)} API
        </h2>
        <p className="text-gray-600 dark:text-gray-300 mb-4">{description}</p>
      </div>

      {endpoints.map((endpoint) => (
        <EndpointCard
          key={`${endpoint.method}-${endpoint.path}`}
          method={endpoint.method}
          path={endpoint.path}
          description={endpoint.description}
          parameters={endpoint.parameters}
          requestExample={endpoint.requestExample}
          responseExample={endpoint.responseExample}
          isActive={activeEndpoint === `${endpoint.method}-${endpoint.path}`}
        />
      ))}
    </div>
  );
};

export default CategorySection; 
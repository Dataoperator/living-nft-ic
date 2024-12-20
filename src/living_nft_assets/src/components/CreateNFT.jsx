import React, { useState } from 'react';
import { useCanister } from '../hooks/useCanister';

export function CreateNFT({ onSuccess }) {
  const [name, setName] = useState('');
  const [loading, setLoading] = useState(false);
  const { actor } = useCanister('living_nft');

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);

    try {
      await actor.create_nft(name);
      setName('');
      onSuccess?.();
    } catch (error) {
      console.error('Failed to create NFT:', error);
    }

    setLoading(false);
  };

  return (
    <div className="bg-gray-800 p-6 rounded-lg mb-8">
      <h2 className="text-xl font-bold mb-4">Create New Living NFT</h2>
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-2">
            Give your NFT a name:
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full px-4 py-2 bg-gray-700 rounded focus:ring-2 focus:ring-blue-500"
            placeholder="Enter name..."
            required
          />
        </div>
        <button
          type="submit"
          disabled={loading}
          className={`w-full px-4 py-2 rounded font-medium 
            ${loading 
              ? 'bg-gray-600 cursor-not-allowed'
              : 'bg-blue-600 hover:bg-blue-700'
            }`}
        >
          {loading ? 'Creating...' : 'Create NFT'}
        </button>
      </form>
    </div>
  );
}
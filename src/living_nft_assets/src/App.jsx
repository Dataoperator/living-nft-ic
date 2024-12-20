import React, { useState, useEffect } from 'react';
import { useAuth } from './auth';
import {
  NFTDashboard,
  CreateNFT,
  Interaction,
  PersonalityView
} from './components';

function App() {
  const { identity, login, logout } = useAuth();
  const [selectedNFT, setSelectedNFT] = useState(null);
  const [userNFTs, setUserNFTs] = useState([]);

  useEffect(() => {
    if (identity) {
      fetchUserNFTs();
    }
  }, [identity]);

  const fetchUserNFTs = async () => {
    // TODO: Implement NFT fetching from canister
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white">
      <nav className="px-6 py-4 flex justify-between items-center">
        <h1 className="text-2xl font-bold">Living NFTs</h1>
        {identity ? (
          <button 
            onClick={logout}
            className="px-4 py-2 bg-red-600 rounded hover:bg-red-700"
          >
            Logout
          </button>
        ) : (
          <button 
            onClick={login}
            className="px-4 py-2 bg-blue-600 rounded hover:bg-blue-700"
          >
            Login with Internet Identity
          </button>
        )}
      </nav>

      {identity ? (
        <main className="container mx-auto px-4 py-8">
          {!selectedNFT ? (
            <>
              <CreateNFT onSuccess={fetchUserNFTs} />
              <NFTDashboard 
                nfts={userNFTs} 
                onSelect={setSelectedNFT} 
              />
            </>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <PersonalityView nft={selectedNFT} />
              <Interaction 
                nft={selectedNFT}
                onInteraction={() => fetchUserNFTs()}
              />
              <button
                onClick={() => setSelectedNFT(null)}
                className="mt-4 px-4 py-2 bg-gray-600 rounded hover:bg-gray-700"
              >
                Back to Dashboard
              </button>
            </div>
          )}
        </main>
      ) : (
        <div className="container mx-auto px-4 py-32 text-center">
          <h2 className="text-4xl font-bold mb-4">Welcome to Living NFTs</h2>
          <p className="text-xl text-gray-300 mb-8">
            Create and interact with AI-powered NFTs that evolve and learn.
          </p>
          <button 
            onClick={login}
            className="px-8 py-4 bg-blue-600 rounded-lg text-xl hover:bg-blue-700"
          >
            Get Started
          </button>
        </div>
      )}
    </div>
  );
}

export default App;
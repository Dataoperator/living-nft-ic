import React, { useState, useRef, useEffect } from 'react';
import { useCanister } from '../hooks/useCanister';

export function Interaction({ nft, onInteraction }) {
  const [input, setInput] = useState('');
  const [messages, setMessages] = useState([]);
  const [loading, setLoading] = useState(false);
  const messagesEndRef = useRef(null);
  const { actor } = useCanister('living_nft');

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!input.trim()) return;

    setLoading(true);
    const userMessage = input;
    setInput('');
    
    // Add user message immediately
    setMessages(prev => [...prev, { type: 'user', content: userMessage }]);

    try {
      const response = await actor.interact(nft.id, userMessage);
      setMessages(prev => [...prev, { 
        type: 'nft', 
        content: response.message,
        changes: response.personality_changes 
      }]);
      onInteraction?.();
    } catch (error) {
      console.error('Interaction failed:', error);
      setMessages(prev => [...prev, { 
        type: 'error', 
        content: 'Sorry, I had trouble processing that.' 
      }]);
    }

    setLoading(false);
  };

  return (
    <div className="bg-gray-800 rounded-lg p-4 h-[600px] flex flex-col">
      <div className="flex-1 overflow-y-auto mb-4 space-y-4">
        {messages.map((message, index) => (
          <div
            key={index}
            className={`p-3 rounded-lg ${
              message.type === 'user'
                ? 'bg-blue-600 ml-8'
                : message.type === 'error'
                ? 'bg-red-600 mr-8'
                : 'bg-gray-700 mr-8'
            }`}
          >
            <p>{message.content}</p>
            {message.changes?.length > 0 && (
              <div className="mt-2 text-sm text-gray-300">
                <p className="font-medium">Personality Changes:</p>
                {message.changes.map((change, i) => (
                  <p key={i}>
                    {change.trait_name}: {change.old_value.toFixed(2)} → {change.new_value.toFixed(2)}
                  </p>
                ))}
              </div>
            )}
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      <form onSubmit={handleSubmit} className="flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="flex-1 px-4 py-2 bg-gray-700 rounded focus:ring-2 focus:ring-blue-500"
          placeholder="Say something..."
          disabled={loading}
        />
        <button
          type="submit"
          disabled={loading}
          className={`px-6 py-2 rounded font-medium ${
            loading
              ? 'bg-gray-600 cursor-not-allowed'
              : 'bg-blue-600 hover:bg-blue-700'
          }`}
        >
          Send
        </button>
      </form>
    </div>
  );
}
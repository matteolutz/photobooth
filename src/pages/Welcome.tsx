import { FC, useState } from "react";
import { User } from "../types/user";

const Welcome: FC<{ onStart: (user: User) => void }> = ({ onStart }) => {
  const [email, setEmail] = useState("");
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!email || !/^\S+@\S+\.\S+$/.test(email)) {
      setError("Please enter a valid email address.");
      return;
    }

    setError(null);
    onStart({ email });
  };

  return (
    <div className="bg-white/80 backdrop-blur-sm rounded-3xl shadow-2xl p-8 md:p-12 text-center transform transition-all duration-500 hover:scale-105">
      <h1 className="text-5xl md:text-6xl font-bold text-gray-800 mb-2">
        Ready?
      </h1>
      <p className="text-xl md:text-2xl text-gray-600 mb-8">
        Let's make some memories!
      </p>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div>
          <label htmlFor="email" className="sr-only">
            Email address
          </label>
          <input
            id="email"
            name="email"
            type="email"
            autoComplete="email"
            required
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full px-6 py-4 text-lg text-gray-700 placeholder-gray-500 bg-white border-2 border-transparent rounded-full focus:outline-none focus:ring-4 focus:ring-pink-400 focus:border-transparent transition-all"
            placeholder="Enter your email"
          />
          {error !== null && (
            <p className="text-red-500 mt-2 text-sm">{error}</p>
          )}
        </div>

        <button
          type="submit"
          className="w-full flex justify-center py-4 px-6 border border-transparent rounded-full shadow-lg text-xl font-bold text-white bg-pink-500 hover:bg-pink-600 focus:outline-none focus:ring-4 focus:ring-pink-400 transform hover:-translate-y-1 transition-all duration-300"
        >
          Start
        </button>
      </form>
      <p className="mt-6 text-xs text-gray-500">
        We'll send your photo strip to this email!
      </p>
    </div>
  );
};

export default Welcome;

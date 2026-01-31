import { useCallback, useEffect, useState } from "react";
import { PhotoboothState } from "./types/state";
import { User } from "./types/user";
import Welcome from "./pages/Welcome";

const App = () => {
  const [state, setState] = useState<PhotoboothState>({ state: "ready" });

  useEffect(() => {
    if (state.state !== "results") {
      return;
    }

    // TODO: send email
  }, [state]);

  const handleStart = useCallback((user: User) => {
    setState({ state: "countdown", user });
  }, []);

  const renderPage = () => {
    switch (state.state) {
      case "ready":
        return <Welcome onStart={handleStart} />;
      case "countdown":
        return <></>;
      case "results":
        return <></>;
    }
  };

  return (
    <div className="bg-linear-to-br from-purple-200 via-pink-200 to-red-200 min-h-screen w-full flex items-center justify-center p-4">
      <main className="w-full max-w-lg mx-auto">{renderPage()}</main>
    </div>
  );
};

export default App;

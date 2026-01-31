import { useCallback, useEffect, useState } from "react";
import { PhotoboothState } from "./types/state";
import { User } from "./types/user";
import Welcome from "./pages/Welcome";
import Countdown from "./pages/Countdown";
import Result from "./pages/Rseult";
import { Photos, resultFromState } from "./types/result";

import test1 from "./assets/test/1.jpg";
import test2 from "./assets/test/2.jpg";
import test3 from "./assets/test/3.jpg";
import test4 from "./assets/test/4.jpg";

const App = () => {
  const [state, setState] = useState<PhotoboothState>({ state: "ready" });

  useEffect(() => {
    if (state.state !== "result") {
      return;
    }

    // TODO: send email
  }, [state]);

  const handleStart = useCallback(
    (user: User) => {
      if (state.state !== "ready") return;
      setState({ state: "countdown", user });
    },
    [state],
  );

  const handleCountdownFinish = useCallback(() => {
    if (state.state !== "countdown") return;

    // TODO: take photos

    /*
    const testPhotos = Array.from(
      { length: 4 },
      (_, i) => `https://picsum.photos/400/400?random=${Date.now() + i}`,
      );*/
    const testPhotos = [test1, test2, test3, test4];

    setState({ ...state, state: "result", photos: testPhotos as Photos });
  }, [state]);

  const onReset = useCallback(() => setState({ state: "ready" }), []);

  const renderPage = () => {
    switch (state.state) {
      case "ready":
        return <Welcome onStart={handleStart} />;
      case "countdown":
        return <Countdown onFinish={handleCountdownFinish} />;
      case "result":
        return <Result result={resultFromState(state)} onReset={onReset} />;
    }
  };

  return (
    <div className="bg-linear-to-br from-purple-200 via-pink-200 to-red-200 min-h-screen w-full flex items-center justify-center p-4">
      <main className="w-full max-w-lg mx-auto">{renderPage()}</main>
    </div>
  );
};

export default App;

import { useCallback, useState } from "react";
import { PhotoboothState } from "./types/state";
import { User } from "./types/user";
import Welcome from "./pages/Welcome";
import Countdown from "./pages/Countdown";
import Result from "./pages/Result";
import { allPhotosTaken, resultFromState } from "./types/result";

import { invoke } from "@tauri-apps/api/core";
import Test from "./pages/Test";

const App = () => {
  const [state, setState] = useState<PhotoboothState>({ state: "ready" });

  const handleStart = useCallback(
    (user: User) => {
      if (state.state !== "ready") return;
      setState({ state: "countdown", user, photos: [] });
    },
    [state],
  );

  const handleCountdownFinish = useCallback(async () => {
    if (state.state !== "countdown") return;

    const takePhoto = async () => {
      await new Promise((resolve) => setTimeout(resolve, 2000));
      return invoke("take_photo").then((res) => res as string);
    };

    const photo = await takePhoto();

    const newPhotos = [...state.photos, photo];

    if (allPhotosTaken(newPhotos)) {
      // we have taken all photos
      setState({ ...state, state: "result", photos: newPhotos });
    } else {
      // we have to take another photo
      setState({
        ...state,
        state: "countdown",
        photos: newPhotos,
      });
    }
  }, [state]);

  const onReset = useCallback(() => setState({ state: "ready" }), []);

  const renderPage = () => {
    switch (state.state) {
      case "ready":
        return <Welcome onStart={handleStart} />;
      case "countdown":
        return (
          <Countdown
            nPhoto={[state.photos.length + 1, 4]}
            onFinish={handleCountdownFinish}
          />
        );
      case "result":
        return <Result result={resultFromState(state)} onReset={onReset} />;
      case "test":
        return <Test />;
    }
  };

  return (
    <div className="bg-linear-to-br from-purple-200 via-pink-200 to-red-200 min-h-screen w-full flex items-center justify-center p-4">
      <main className="w-full max-w-lg mx-auto">{renderPage()}</main>
      <div className="absolute bottom-0 w-full flex justify-end gap-2 p-2 text-gray-500 text-sm">
        &copy; {new Date().getFullYear()} Matteo Lutz
      </div>
    </div>
  );
};

export default App;
